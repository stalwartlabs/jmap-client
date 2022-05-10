use std::time::Duration;

use crate::core::request::Request;

const DEFAULT_TIMEOUT_MS: u64 = 10 * 1000;

pub struct Client {
    client: reqwest::ClientBuilder,
    default_account_id: String,
}

impl Client {
    pub fn connect(url: &str) -> Self {
        Client {
            client: reqwest::Client::builder().timeout(Duration::from_millis(DEFAULT_TIMEOUT_MS)),
            default_account_id: "co".to_string(),
        }
    }

    pub fn default_account_id(&self) -> &str {
        &self.default_account_id
    }

    pub fn request(&self) -> Request<'_> {
        Request::new(self)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_serialize() {
        /*let mut client = Client::connect("coco");

        client.request().email_set().create(
            "coco",
            Email::new()
                .from(["Pepe"])
                .subject("Hello world!")
                .sent_at(342374),
        );*/

        /*let query: QueryRequest<EmailFilter, EmailComparator, email::QueryArguments> =
            QueryRequest::new("coco".to_string())
                .filter(Filter::or([
                    Filter::and([
                        EmailFilter::in_mailbox("peperino"),
                        EmailFilter::in_mailbox_other_than(["coco", "miel"]),
                        EmailFilter::from("comoro"),
                    ]),
                    Filter::not([EmailFilter::after(428374234)]),
                ]))
                .sort([
                    EmailComparator::has_keyword("cocomiel"),
                    EmailComparator::size(),
                ]);

        println!("{}", serde_json::to_string_pretty(&query).unwrap());*/
    }
}
