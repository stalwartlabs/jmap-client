use std::{collections::HashMap, pin::Pin};

use futures_util::{stream::SplitSink, SinkExt, Stream, StreamExt};
use serde::{Deserialize, Serialize};
use tokio::net::TcpStream;
use tokio_tungstenite::{
    tungstenite::{client::IntoClientRequest, Message},
    MaybeTlsStream, WebSocketStream,
};

use crate::{
    client::Client,
    core::{
        error::{ProblemDetails, ProblemType},
        request::{Arguments, Request},
        response::{Response, TaggedMethodResponse},
    },
    event_source::Changes,
    Method, StateChangeType, TypeState, URI,
};

#[derive(Debug, Serialize)]
struct WebSocketRequest {
    #[serde(rename = "@type")]
    pub _type: WebSocketRequestType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    using: Vec<URI>,

    #[serde(rename = "methodCalls")]
    method_calls: Vec<(Method, Arguments, String)>,

    #[serde(rename = "createdIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    created_ids: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize)]
pub struct WebSocketResponse {
    #[serde(rename = "@type")]
    _type: WebSocketResponseType,

    #[serde(rename = "requestId")]
    request_id: Option<String>,

    #[serde(rename = "methodResponses")]
    method_responses: Vec<TaggedMethodResponse>,

    #[serde(rename = "createdIds")]
    created_ids: Option<HashMap<String, String>>,

    #[serde(rename = "sessionState")]
    session_state: String,
}

#[derive(Debug, Serialize, Deserialize)]
enum WebSocketResponseType {
    Response,
}

#[derive(Debug, Serialize)]
struct WebSocketPushEnable {
    #[serde(rename = "@type")]
    _type: WebSocketPushEnableType,

    #[serde(rename = "dataTypes")]
    data_types: Option<Vec<StateChangeType>>,

    #[serde(rename = "pushState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    push_state: Option<String>,
}

#[derive(Debug, Serialize)]
struct WebSocketPushDisable {
    #[serde(rename = "@type")]
    _type: WebSocketPushDisableType,
}

#[derive(Debug, Serialize)]
enum WebSocketRequestType {
    Request,
}

#[derive(Debug, Serialize)]
enum WebSocketPushEnableType {
    WebSocketPushEnable,
}

#[derive(Debug, Serialize)]
enum WebSocketPushDisableType {
    WebSocketPushDisable,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum WebSocketStateChangeType {
    StateChange,
}

#[derive(Deserialize, Debug)]
pub struct WebSocketStateChange {
    #[serde(rename = "@type")]
    pub type_: WebSocketStateChangeType,

    pub changed: HashMap<String, HashMap<TypeState, String>>,

    #[serde(rename = "pushState")]
    push_state: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct WebSocketProblem {
    #[serde(rename = "@type")]
    pub type_: WebSocketProblemType,

    #[serde(rename = "requestId")]
    pub request_id: Option<String>,

    #[serde(rename = "type")]
    p_type: ProblemType,
    status: Option<u32>,
    title: Option<String>,
    detail: Option<String>,
    limit: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum WebSocketProblemType {
    Problem,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum WebSocketMessage_ {
    Response(WebSocketResponse),
    StateChange(WebSocketStateChange),
    Error(WebSocketProblem),
}

#[derive(Debug)]
pub enum WebSocketMessage {
    Response(Response<TaggedMethodResponse>),
    StateChange(Changes),
}

pub struct WsStream {
    tx: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    req_id: usize,
}

impl Client {
    pub async fn connect_ws(
        &mut self,
    ) -> crate::Result<Pin<Box<impl Stream<Item = crate::Result<WebSocketMessage>>>>> {
        let capabilities = self.session().websocket_capabilities().ok_or_else(|| {
            crate::Error::Internal(
                "JMAP server does not advertise any websocket capabilities.".to_string(),
            )
        })?;

        let mut request = capabilities.url().into_client_request()?;
        request
            .headers_mut()
            .insert("Authorization", "Bearer 123".parse().unwrap()); //TODO implement

        let (stream, _) = tokio_tungstenite::connect_async(request).await?;
        let (tx, mut rx) = stream.split();

        self.set_ws_stream(WsStream { tx, req_id: 0 });

        Ok(Box::pin(async_stream::stream! {
            while let Some(message) = rx.next().await {
                match message {
                    Ok(message) if message.is_text() => {
                        match serde_json::from_slice::<WebSocketMessage_>(&message.into_data()) {
                            Ok(message) => match message {
                                WebSocketMessage_::Response(response) => {
                                    yield Ok(WebSocketMessage::Response(Response::new(
                                        response.method_responses,
                                        response.created_ids,
                                        response.session_state,
                                        response.request_id,
                                    )))
                                }
                                WebSocketMessage_::StateChange(changes) => {
                                    yield Ok(WebSocketMessage::StateChange(Changes::new(
                                        changes.push_state,
                                        changes.changed,
                                    )))
                                }
                                WebSocketMessage_::Error(err) => yield Err(ProblemDetails::from(err).into()),
                            },
                            Err(err) => yield Err(err.into()),
                        }
                    }
                    Ok(_) => (),
                    Err(err) => yield Err(err.into()),
                }
            }
        }))
    }

    pub async fn send_ws(&mut self, request: Request<'_>) -> crate::Result<String> {
        let ws = self
            .ws_stream()
            .ok_or_else(|| crate::Error::Internal("Websocket stream not set.".to_string()))?;

        // Assing request id
        let request_id = ws.req_id.to_string();
        ws.req_id += 1;

        ws.tx
            .send(Message::text(
                serde_json::to_string(&WebSocketRequest {
                    _type: WebSocketRequestType::Request,
                    id: request_id.clone().into(),
                    using: request.using,
                    method_calls: request.method_calls,
                    created_ids: request.created_ids,
                })
                .unwrap_or_default(),
            ))
            .await?;

        Ok(request_id)
    }

    pub async fn enable_push_ws(
        &mut self,
        data_types: Option<impl IntoIterator<Item = StateChangeType>>,
        push_state: Option<impl Into<String>>,
    ) -> crate::Result<()> {
        self.ws_stream()
            .ok_or_else(|| crate::Error::Internal("Websocket stream not set.".to_string()))?
            .tx
            .send(Message::text(
                serde_json::to_string(&WebSocketPushEnable {
                    _type: WebSocketPushEnableType::WebSocketPushEnable,
                    data_types: data_types.map(|it| it.into_iter().collect()),
                    push_state: push_state.map(|it| it.into()),
                })
                .unwrap_or_default(),
            ))
            .await
            .map_err(|err| err.into())
    }

    pub async fn disable_push_ws(&mut self) -> crate::Result<()> {
        self.ws_stream()
            .ok_or_else(|| crate::Error::Internal("Websocket stream not set.".to_string()))?
            .tx
            .send(Message::text(
                serde_json::to_string(&WebSocketPushDisable {
                    _type: WebSocketPushDisableType::WebSocketPushDisable,
                })
                .unwrap_or_default(),
            ))
            .await
            .map_err(|err| err.into())
    }

    pub async fn ws_ping(&mut self) -> crate::Result<()> {
        self.ws_stream()
            .ok_or_else(|| crate::Error::Internal("Websocket stream not set.".to_string()))?
            .tx
            .send(Message::Ping(vec![]))
            .await
            .map_err(|err| err.into())
    }
}

impl From<WebSocketProblem> for ProblemDetails {
    fn from(problem: WebSocketProblem) -> Self {
        ProblemDetails::new(
            problem.p_type,
            problem.status,
            problem.title,
            problem.detail,
            problem.limit,
            problem.request_id,
        )
    }
}
