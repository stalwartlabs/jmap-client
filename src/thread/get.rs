use super::Thread;

impl Thread {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn email_ids(&self) -> &[String] {
        &self.email_ids
    }
}