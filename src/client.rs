/*
 * Copyright Stalwart Labs Ltd. See the COPYING
 * file at the top-level directory of this distribution.
 *
 * Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * option. This file may not be copied, modified, or distributed
 * except according to those terms.
 */

use std::{
    net::IpAddr,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};

use ahash::AHashSet;
use reqwest::{
    header::{self},
    redirect,
};
use serde::de::DeserializeOwned;

use crate::{
    blob,
    core::{
        request::{self, Request},
        response,
        session::{Session, URLPart},
    },
    Error,
};

const DEFAULT_TIMEOUT_MS: u64 = 10 * 1000;
static USER_AGENT: &str = concat!("jmap-client/", env!("CARGO_PKG_VERSION"));

#[derive(Debug, PartialEq, Eq)]
pub enum Credentials {
    Basic(String),
    Bearer(String),
}

pub struct Client {
    session: parking_lot::Mutex<Arc<Session>>,
    session_url: String,
    api_url: String,
    session_updated: AtomicBool,
    trusted_hosts: Arc<AHashSet<String>>,

    upload_url: Vec<URLPart<blob::URLParameter>>,
    download_url: Vec<URLPart<blob::URLParameter>>,
    #[cfg(feature = "async")]
    event_source_url: Vec<URLPart<crate::event_source::URLParameter>>,

    headers: header::HeaderMap,
    default_account_id: String,
    timeout: u64,

    #[cfg(feature = "websockets")]
    pub(crate) authorization: String,
    #[cfg(feature = "websockets")]
    pub(crate) ws: tokio::sync::Mutex<Option<crate::client_ws::WsStream>>,
}

pub struct ClientBuilder {
    credentials: Option<Credentials>,
    trusted_hosts: AHashSet<String>,
    forwarded_for: Option<String>,
    timeout: u64,
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ClientBuilder {
    pub fn new() -> Self {
        Self {
            credentials: None,
            trusted_hosts: AHashSet::new(),
            timeout: DEFAULT_TIMEOUT_MS,
            forwarded_for: None,
        }
    }

    pub fn credentials(mut self, credentials: impl Into<Credentials>) -> Self {
        self.credentials = Some(credentials.into());
        self
    }

    pub fn timeout(mut self, timeout: u64) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn follow_redirects(
        mut self,
        trusted_hosts: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        self.trusted_hosts = trusted_hosts.into_iter().map(|h| h.into()).collect();
        self
    }

    pub fn forwarded_for(mut self, forwarded_for: IpAddr) -> Self {
        self.forwarded_for = Some(match forwarded_for {
            IpAddr::V4(addr) => format!("for={}", addr),
            IpAddr::V6(addr) => format!("for=\"{}\"", addr),
        });
        self
    }

    #[cfg(feature = "async")]
    pub async fn connect(self, url: &str) -> crate::Result<Client> {
        let authorization = match self.credentials.expect("Missing credentials") {
            Credentials::Basic(s) => format!("Basic {}", s),
            Credentials::Bearer(s) => format!("Bearer {}", s),
        };
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::USER_AGENT,
            header::HeaderValue::from_static(USER_AGENT),
        );
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&authorization).unwrap(),
        );
        if let Some(forwarded_for) = self.forwarded_for {
            headers.insert(
                header::FORWARDED,
                header::HeaderValue::from_str(&forwarded_for).unwrap(),
            );
        }

        let trusted_hosts = Arc::new(self.trusted_hosts);

        let trusted_hosts_ = trusted_hosts.clone();
        let session_url = format!("{}/.well-known/jmap", url);
        let session: Session = serde_json::from_slice(
            &Client::handle_error(
                reqwest::Client::builder()
                    .timeout(Duration::from_millis(self.timeout))
                    .redirect(redirect::Policy::custom(move |attempt| {
                        if attempt.previous().len() > 5 {
                            attempt.error("Too many redirects.")
                        } else if matches!( attempt.url().host_str(), Some(host) if trusted_hosts_.contains(host) )
                        {
                            #[cfg(feature = "follow-trusted")]
                            {
                                attempt.follow_trusted()
                            }
                            
                            #[cfg(not(feature = "follow-trusted"))]
                            {
                                attempt.follow()
                            }
                        } else {
                            let message = format!(
                                "Aborting redirect request to unknown host '{}'.",
                                attempt.url().host_str().unwrap_or("")
                            );
                            attempt.error(message)
                        }
                    }))
                    .default_headers(headers.clone())
                    .build()?
                    .get(&session_url)
                    .send()
                    .await?,
            )
            .await?
            .bytes()
            .await?,
        )?;

        let default_account_id = session
            .primary_accounts()
            .next()
            .map(|a| a.1.to_string())
            .unwrap_or_default();

        headers.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        );

        Ok(Client {
            download_url: URLPart::parse(session.download_url())?,
            upload_url: URLPart::parse(session.upload_url())?,
            #[cfg(feature = "async")]
            event_source_url: URLPart::parse(session.event_source_url())?,
            api_url: session.api_url().to_string(),
            session: parking_lot::Mutex::new(Arc::new(session)),
            session_url,
            session_updated: true.into(),
            trusted_hosts,
            #[cfg(feature = "websockets")]
            authorization,
            timeout: self.timeout,
            headers,
            default_account_id,
            #[cfg(feature = "websockets")]
            ws: None.into(),
        })
    }

    #[cfg(feature = "blocking")]
    pub fn connect(self, url: &str) -> crate::Result<Client> {
        let authorization = match self.credentials.expect("Missing credentials") {
            Credentials::Basic(s) => format!("Basic {}", s),
            Credentials::Bearer(s) => format!("Bearer {}", s),
        };
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::USER_AGENT,
            header::HeaderValue::from_static(USER_AGENT),
        );
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&authorization).unwrap(),
        );
        if let Some(forwarded_for) = self.forwarded_for {
            headers.insert(
                header::FORWARDED,
                header::HeaderValue::from_str(&forwarded_for).unwrap(),
            );
        }

        let trusted_hosts = Arc::new(self.trusted_hosts);

        let trusted_hosts_ = trusted_hosts.clone();
        let session_url = format!("{}/.well-known/jmap", url);
        let session: Session = serde_json::from_slice(
            &Client::handle_error(
                reqwest::blocking::Client::builder()
                    .timeout(Duration::from_millis(self.timeout))
                    .redirect(redirect::Policy::custom(move |attempt| {
                        if attempt.previous().len() > 5 {
                            attempt.error("Too many redirects.")
                        } else if matches!( attempt.url().host_str(), Some(host) if trusted_hosts_.contains(host) )
                        {
                            attempt.follow_trusted()
                        } else {
                            let message = format!(
                                "Aborting redirect request to unknown host '{}'.",
                                attempt.url().host_str().unwrap_or("")
                            );
                            attempt.error(message)
                        }
                    }))
                    .default_headers(headers.clone())
                    .build()?
                    .get(&session_url)
                    .send()
                    ?,
            )?
            .bytes()?,
        )?;

        let default_account_id = session
            .primary_accounts()
            .next()
            .map(|a| a.1.to_string())
            .unwrap_or_default();

        headers.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        );

        Ok(Client {
            download_url: URLPart::parse(session.download_url())?,
            upload_url: URLPart::parse(session.upload_url())?,
            #[cfg(feature = "async")]
            event_source_url: URLPart::parse(session.event_source_url())?,
            api_url: session.api_url().to_string(),
            session: parking_lot::Mutex::new(Arc::new(session)),
            session_url,
            session_updated: true.into(),
            trusted_hosts,
            #[cfg(feature = "websockets")]
            authorization,
            timeout: self.timeout,
            headers,
            default_account_id,
            #[cfg(feature = "websockets")]
            ws: None.into(),
        })
    }
}

impl Client {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> ClientBuilder {
        ClientBuilder::new()
    }

    pub fn set_timeout(&mut self, timeout: u64) -> &mut Self {
        self.timeout = timeout;
        self
    }

    pub fn set_follow_redirects(
        &mut self,
        trusted_hosts: impl IntoIterator<Item = impl Into<String>>,
    ) -> &mut Self {
        self.trusted_hosts = Arc::new(trusted_hosts.into_iter().map(|h| h.into()).collect());
        self
    }

    pub fn timeout(&self) -> u64 {
        self.timeout
    }

    pub fn session(&self) -> Arc<Session> {
        self.session.lock().clone()
    }

    pub fn session_url(&self) -> &str {
        &self.session_url
    }

    pub fn headers(&self) -> &header::HeaderMap {
        &self.headers
    }

    pub(crate) fn redirect_policy(&self) -> redirect::Policy {
        let trusted_hosts = self.trusted_hosts.clone();
        redirect::Policy::custom(move |attempt| {
            if attempt.previous().len() > 5 {
                attempt.error("Too many redirects.")
            } else if matches!( attempt.url().host_str(), Some(host) if trusted_hosts.contains(host) )
            {
                #[cfg(feature = "follow-trusted")]
                {
                    attempt.follow_trusted()
                }
                
                #[cfg(not(feature = "follow-trusted"))]
                {
                    attempt.follow()
                }
            } else {
                let message = format!(
                    "Aborting redirect request to unknown host '{}'.",
                    attempt.url().host_str().unwrap_or("")
                );
                attempt.error(message)
            }
        })
    }

    #[cfg(feature = "async")]
    pub async fn send<R>(
        &self,
        request: &request::Request<'_>,
    ) -> crate::Result<response::Response<R>>
    where
        R: DeserializeOwned,
    {
        let response: response::Response<R> = serde_json::from_slice(
            &Client::handle_error(
                reqwest::Client::builder()
                    .redirect(self.redirect_policy())
                    .timeout(Duration::from_millis(self.timeout))
                    .default_headers(self.headers.clone())
                    .build()?
                    .post(&self.api_url)
                    .body(serde_json::to_string(&request)?)
                    .send()
                    .await?,
            )
            .await?
            .bytes()
            .await?,
        )?;

        if response.session_state() != self.session.lock().state() {
            self.session_updated.store(false, Ordering::Relaxed);
        }

        Ok(response)
    }

    #[cfg(feature = "blocking")]
    pub fn send<R>(&self, request: &request::Request<'_>) -> crate::Result<response::Response<R>>
    where
        R: DeserializeOwned,
    {
        let response: response::Response<R> = serde_json::from_slice(
            &Client::handle_error(
                reqwest::blocking::Client::builder()
                    .redirect(self.redirect_policy())
                    .timeout(Duration::from_millis(self.timeout))
                    .default_headers(self.headers.clone())
                    .build()?
                    .post(&self.api_url)
                    .body(serde_json::to_string(&request)?)
                    .send()?,
            )?
            .bytes()?,
        )?;

        if response.session_state() != self.session.lock().state() {
            self.session_updated.store(false, Ordering::Relaxed);
        }

        Ok(response)
    }

    #[cfg(feature = "async")]
    pub async fn refresh_session(&self) -> crate::Result<()> {
        let session: Session = serde_json::from_slice(
            &Client::handle_error(
                reqwest::Client::builder()
                    .timeout(Duration::from_millis(DEFAULT_TIMEOUT_MS))
                    .redirect(self.redirect_policy())
                    .default_headers(self.headers.clone())
                    .build()?
                    .get(&self.session_url)
                    .send()
                    .await?,
            )
            .await?
            .bytes()
            .await?,
        )?;
        *self.session.lock() = Arc::new(session);
        self.session_updated.store(true, Ordering::Relaxed);
        Ok(())
    }

    #[cfg(feature = "blocking")]
    pub async fn refresh_session(&self) -> crate::Result<()> {
        let session: Session = serde_json::from_slice(
            &Client::handle_error(
                reqwest::blocking::Client::builder()
                    .timeout(Duration::from_millis(DEFAULT_TIMEOUT_MS))
                    .redirect(self.redirect_policy())
                    .default_headers(self.headers.clone())
                    .build()?
                    .get(&self.session_url)
                    .send()?,
            )?
            .bytes()?,
        )?;
        *self.session.lock() = Arc::new(session);
        self.session_updated.store(true, Ordering::Relaxed);
        Ok(())
    }

    pub fn is_session_updated(&self) -> bool {
        self.session_updated.load(Ordering::Relaxed)
    }

    pub fn set_default_account_id(&mut self, defaul_account_id: impl Into<String>) -> &mut Self {
        self.default_account_id = defaul_account_id.into();
        self
    }

    pub fn default_account_id(&self) -> &str {
        &self.default_account_id
    }

    pub fn build(&self) -> Request<'_> {
        Request::new(self)
    }

    pub fn download_url(&self) -> &[URLPart<blob::URLParameter>] {
        &self.download_url
    }

    pub fn upload_url(&self) -> &[URLPart<blob::URLParameter>] {
        &self.upload_url
    }

    #[cfg(feature = "async")]
    pub fn event_source_url(&self) -> &[URLPart<crate::event_source::URLParameter>] {
        &self.event_source_url
    }

    #[cfg(feature = "async")]
    pub async fn handle_error(response: reqwest::Response) -> crate::Result<reqwest::Response> {
        if response.status().is_success() {
            Ok(response)
        } else if let Some(b"application/problem+json") = response
            .headers()
            .get(header::CONTENT_TYPE)
            .map(|h| h.as_bytes())
        {
            Err(Error::Problem(serde_json::from_slice(
                &response.bytes().await?,
            )?))
        } else {
            Err(Error::Server(format!("{}", response.status())))
        }
    }

    #[cfg(feature = "blocking")]
    pub fn handle_error(
        response: reqwest::blocking::Response,
    ) -> crate::Result<reqwest::blocking::Response> {
        if response.status().is_success() {
            Ok(response)
        } else if let Some(b"application/problem+json") = response
            .headers()
            .get(header::CONTENT_TYPE)
            .map(|h| h.as_bytes())
        {
            Err(Error::Problem(serde_json::from_slice(&response.bytes()?)?))
        } else {
            Err(Error::Server(format!("{}", response.status())))
        }
    }
}

impl Credentials {
    pub fn basic(username: &str, password: &str) -> Self {
        Credentials::Basic(base64::encode(format!("{}:{}", username, password)))
    }

    pub fn bearer(token: impl Into<String>) -> Self {
        Credentials::Bearer(token.into())
    }
}

impl From<&str> for Credentials {
    fn from(s: &str) -> Self {
        Credentials::bearer(s.to_string())
    }
}

impl From<String> for Credentials {
    fn from(s: String) -> Self {
        Credentials::bearer(s)
    }
}

impl From<(&str, &str)> for Credentials {
    fn from((username, password): (&str, &str)) -> Self {
        Credentials::basic(username, password)
    }
}

impl From<(String, String)> for Credentials {
    fn from((username, password): (String, String)) -> Self {
        Credentials::basic(&username, &password)
    }
}

#[cfg(test)]
mod tests {
    use crate::core::{response::{Response, TaggedMethodResponse}};

    #[test]
    fn test_deserialize() {
        let _r: Response<TaggedMethodResponse> = serde_json::from_slice(
            br#"{"sessionState": "123", "methodResponses": [[ "Email/query", {
                "accountId": "A1",
                "queryState": "abcdefg",
                "canCalculateChanges": true,
                "position": 0,
                "total": 101,
                "ids": [ "msg1023", "msg223", "msg110", "msg93", "msg91",
                    "msg38", "msg36", "msg33", "msg11", "msg1" ]
            }, "t0" ],
            [ "Email/get", {
                "accountId": "A1",
                "state": "123456",
                "list": [{
                    "id": "msg1023",
                    "threadId": "trd194"
                }, {
                    "id": "msg223",
                    "threadId": "trd114"
                }
                ],
                "notFound": []
            }, "t1" ],
            [ "Thread/get", {
                "accountId": "A1",
                "state": "123456",
                "list": [{
                    "id": "trd194",
                    "emailIds": [ "msg1020", "msg1021", "msg1023" ]
                }, {
                    "id": "trd114",
                    "emailIds": [ "msg201", "msg223" ]
                }
                ],
                "notFound": []
            }, "t2" ]]}"#,
        )
        .unwrap();
    }
}
