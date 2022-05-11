pub mod get;
pub mod import;
pub mod parse;
pub mod query;
pub mod search_snippet;
pub mod set;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{core::request::ResultReference, Get};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Email<State = Get> {
    #[serde(skip)]
    _create_id: Option<usize>,

    #[serde(skip)]
    _state: std::marker::PhantomData<State>,

    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,

    #[serde(rename = "blobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    blob_id: Option<String>,

    #[serde(rename = "threadId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    thread_id: Option<String>,

    #[serde(rename = "mailboxIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    mailbox_ids: Option<HashMap<String, bool>>,

    #[serde(rename = "#mailboxIds")]
    #[serde(skip_deserializing)]
    #[serde(skip_serializing_if = "Option::is_none")]
    mailbox_ids_ref: Option<ResultReference>,

    #[serde(rename = "keywords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    keywords: Option<HashMap<String, bool>>,

    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<usize>,

    #[serde(rename = "receivedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    received_at: Option<DateTime<Utc>>,

    #[serde(rename = "messageId", alias = "header:Message-ID:asMessageIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    message_id: Option<Vec<String>>,

    #[serde(rename = "inReplyTo", alias = "header:In-Reply-To:asMessageIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    in_reply_to: Option<Vec<String>>,

    #[serde(rename = "references", alias = "header:References:asMessageIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    references: Option<Vec<String>>,

    #[serde(rename = "sender", alias = "header:Sender:asAddresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    sender: Option<Vec<EmailAddress>>,

    #[serde(rename = "from", alias = "header:From:asAddresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    from: Option<Vec<EmailAddress>>,

    #[serde(rename = "to", alias = "header:To:asAddresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    to: Option<Vec<EmailAddress>>,

    #[serde(rename = "cc", alias = "header:Cc:asAddresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    cc: Option<Vec<EmailAddress>>,

    #[serde(rename = "bcc", alias = "header:Bcc:asAddresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    bcc: Option<Vec<EmailAddress>>,

    #[serde(rename = "replyTo", alias = "header:Reply-To:asAddresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to: Option<Vec<EmailAddress>>,

    #[serde(rename = "subject", alias = "header:Subject:asText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    subject: Option<String>,

    #[serde(rename = "sentAt", alias = "header:Date:asDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    sent_at: Option<DateTime<Utc>>,

    #[serde(rename = "bodyStructure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    body_structure: Option<Box<EmailBodyPart>>,

    #[serde(rename = "bodyValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    body_values: Option<HashMap<String, EmailBodyValue>>,

    #[serde(rename = "textBody")]
    #[serde(skip_serializing_if = "Option::is_none")]
    text_body: Option<Vec<EmailBodyPart>>,

    #[serde(rename = "htmlBody")]
    #[serde(skip_serializing_if = "Option::is_none")]
    html_body: Option<Vec<EmailBodyPart>>,

    #[serde(rename = "attachments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    attachments: Option<Vec<EmailBodyPart>>,

    #[serde(rename = "hasAttachment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    has_attachment: Option<bool>,

    #[serde(rename = "preview")]
    #[serde(skip_serializing_if = "Option::is_none")]
    preview: Option<String>,

    #[serde(flatten)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    others: HashMap<String, Option<Field>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailBodyPart<State = Get> {
    #[serde(skip)]
    _state: std::marker::PhantomData<State>,

    #[serde(rename = "partId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    part_id: Option<String>,

    #[serde(rename = "blobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    blob_id: Option<String>,

    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<usize>,

    #[serde(rename = "headers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    headers: Option<Vec<EmailHeader>>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    type_: Option<String>,

    #[serde(rename = "charset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    charset: Option<String>,

    #[serde(rename = "disposition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    disposition: Option<String>,

    #[serde(rename = "cid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    cid: Option<String>,

    #[serde(rename = "language")]
    #[serde(skip_serializing_if = "Option::is_none")]
    language: Option<Vec<String>>,

    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<String>,

    #[serde(rename = "subParts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    sub_parts: Option<Vec<EmailBodyPart>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailBodyValue<State = Get> {
    #[serde(skip)]
    _state: std::marker::PhantomData<State>,

    #[serde(rename = "value")]
    value: String,

    #[serde(rename = "isEncodingProblem")]
    is_encoding_problem: bool,

    #[serde(rename = "isTruncated")]
    is_truncated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Field {
    Text(String),
    TextList(Vec<String>),
    Date(DateTime<Utc>),
    Addresses(Vec<EmailAddress>),
    GroupedAddresses(Vec<EmailAddressGroup>),
    Bool(bool),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailAddress<State = Get> {
    #[serde(skip)]
    _state: std::marker::PhantomData<State>,

    name: Option<String>,
    email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailAddressGroup<State = Get> {
    #[serde(skip)]
    _state: std::marker::PhantomData<State>,

    name: Option<String>,
    addresses: Vec<EmailAddress>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailHeader<State = Get> {
    #[serde(skip)]
    _state: std::marker::PhantomData<State>,

    name: String,
    value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Property {
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "blobId")]
    BlobId,
    #[serde(rename = "threadId")]
    ThreadId,
    #[serde(rename = "mailboxIds")]
    MailboxIds,
    #[serde(rename = "keywords")]
    Keywords,
    #[serde(rename = "size")]
    Size,
    #[serde(rename = "receivedAt")]
    ReceivedAt,
    #[serde(rename = "messageId")]
    MessageId,
    #[serde(rename = "inReplyTo")]
    InReplyTo,
    #[serde(rename = "references")]
    References,
    #[serde(rename = "sender")]
    Sender,
    #[serde(rename = "from")]
    From,
    #[serde(rename = "to")]
    To,
    #[serde(rename = "cc")]
    Cc,
    #[serde(rename = "bcc")]
    Bcc,
    #[serde(rename = "replyTo")]
    ReplyTo,
    #[serde(rename = "subject")]
    Subject,
    #[serde(rename = "sentAt")]
    SentAt,
    #[serde(rename = "bodyStructure")]
    BodyStructure,
    #[serde(rename = "bodyValues")]
    BodyValues,
    #[serde(rename = "textBody")]
    TextBody,
    #[serde(rename = "htmlBody")]
    HtmlBody,
    #[serde(rename = "attachments")]
    Attachments,
    #[serde(rename = "hasAttachment")]
    HasAttachment,
    #[serde(rename = "preview")]
    Preview,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BodyProperty {
    #[serde(rename = "partId")]
    PartId,
    #[serde(rename = "blobId")]
    BlobId,
    #[serde(rename = "size")]
    Size,
    #[serde(rename = "headers")]
    Headers,
    #[serde(rename = "name")]
    Name,
    #[serde(rename = "type")]
    Type,
    #[serde(rename = "charset")]
    Charset,
    #[serde(rename = "disposition")]
    Disposition,
    #[serde(rename = "cid")]
    Cid,
    #[serde(rename = "language")]
    Language,
    #[serde(rename = "location")]
    Location,
    #[serde(rename = "subParts")]
    SubParts,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MailCapabilities {
    #[serde(rename = "maxMailboxesPerEmail")]
    max_mailboxes_per_email: Option<usize>,

    #[serde(rename = "maxMailboxDepth")]
    max_mailbox_depth: usize,

    #[serde(rename = "maxSizeMailboxName")]
    max_size_mailbox_name: usize,

    #[serde(rename = "maxSizeAttachmentsPerEmail")]
    max_size_attachments_per_email: usize,

    #[serde(rename = "emailQuerySortOptions")]
    email_query_sort_options: Vec<String>,

    #[serde(rename = "mayCreateTopLevelMailbox")]
    may_create_top_level_mailbox: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SubmissionCapabilities {
    #[serde(rename = "maxDelayedSend")]
    max_delayed_send: usize,

    #[serde(rename = "submissionExtensions")]
    submission_extensions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct QueryArguments {
    #[serde(rename = "collapseThreads")]
    collapse_threads: bool,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct GetArguments {
    #[serde(rename = "bodyProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    body_properties: Option<Vec<BodyProperty>>,
    #[serde(rename = "fetchTextBodyValues")]
    fetch_text_body_values: bool,
    #[serde(rename = "fetchHTMLBodyValues")]
    fetch_html_body_values: bool,
    #[serde(rename = "fetchAllBodyValues")]
    fetch_all_body_values: bool,
    #[serde(rename = "maxBodyValueBytes")]
    max_body_value_bytes: usize,
}

impl QueryArguments {
    pub fn collapse_threads(&mut self, collapse_threads: bool) {
        self.collapse_threads = collapse_threads;
    }
}

impl GetArguments {
    pub fn body_properties(
        &mut self,
        body_properties: impl IntoIterator<Item = BodyProperty>,
    ) -> &mut Self {
        self.body_properties = Some(body_properties.into_iter().collect());
        self
    }

    pub fn fetch_text_body_values(&mut self, fetch_text_body_values: bool) -> &mut Self {
        self.fetch_text_body_values = fetch_text_body_values;
        self
    }

    pub fn fetch_html_body_values(&mut self, fetch_html_body_values: bool) -> &mut Self {
        self.fetch_html_body_values = fetch_html_body_values;
        self
    }

    pub fn fetch_all_body_values(&mut self, fetch_all_body_values: bool) -> &mut Self {
        self.fetch_all_body_values = fetch_all_body_values;
        self
    }

    pub fn max_body_value_bytes(&mut self, max_body_value_bytes: usize) -> &mut Self {
        self.max_body_value_bytes = max_body_value_bytes;
        self
    }
}

impl MailCapabilities {
    pub fn max_mailboxes_per_email(&self) -> Option<usize> {
        self.max_mailboxes_per_email
    }

    pub fn max_mailbox_depth(&self) -> usize {
        self.max_mailbox_depth
    }

    pub fn max_size_mailbox_name(&self) -> usize {
        self.max_size_mailbox_name
    }

    pub fn max_size_attachments_per_email(&self) -> usize {
        self.max_size_attachments_per_email
    }

    pub fn email_query_sort_options(&self) -> &[String] {
        &self.email_query_sort_options
    }

    pub fn may_create_top_level_mailbox(&self) -> bool {
        self.may_create_top_level_mailbox
    }
}

impl SubmissionCapabilities {
    pub fn max_delayed_send(&self) -> usize {
        self.max_delayed_send
    }

    pub fn submission_extensions(&self) -> &[String] {
        &self.submission_extensions
    }
}
