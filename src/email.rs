use lettre::address::Envelope;
use lettre::message::header::ContentType;
use lettre::message::Mailbox;
use lettre::transport::file::AsyncFileTransport;
use lettre::transport::smtp::authentication::{Credentials, Mechanism};
use lettre::transport::smtp::AsyncSmtpTransport;
use lettre::transport::stub::AsyncStubTransport;
use lettre::{Address, AsyncTransport, Message, Tokio1Executor};
use rand::distr::{Alphanumeric, SampleString};
use std::sync::Arc;

use crate::config::{self, Env};

pub trait Email {
    fn subject(&self) -> String;
    fn body(&self) -> String;
}

#[derive(Debug, Clone)]
pub struct Emails {
    backend: EmailBackend,
    pub domain: String,
    from: Address,
}

const DEFAULT_FROM: &str = "noreply@learn.framer.university";

impl Emails {
    /// Create a new instance detecting the backend from the environment. This will either connect
    /// to a SMTP server or store the emails on the local filesystem.
    pub fn from_environment(config: &config::Server) -> Self {
        let config::Server {
            mailgun_smtp_login: login,
            mailgun_smtp_password: password,
            mailgun_smtp_server: server,
            ..
        } = config;

        let backend = match (login, password, server) {
            (login, password, server) => {
                let transport = AsyncSmtpTransport::<Tokio1Executor>::relay(&server)
                    .unwrap()
                    .credentials(Credentials::new(login.to_owned(), password.to_owned()))
                    .authentication(vec![Mechanism::Plain])
                    .build();

                EmailBackend::Smtp(Box::new(transport))
            }
            _ => {
                let transport = AsyncFileTransport::new("/tmp");
                EmailBackend::FileSystem(Arc::new(transport))
            }
        };

        if config.env == Env::Production && !matches!(backend, EmailBackend::Smtp { .. }) {
            panic!("Only the SMTP backend is allowed in production");
        }

        let domain = config.domain_name.clone();

        Self {
            backend,
            domain,
            from: login.parse().unwrap(),
        }
    }

    /// Create a new test backend that stores all the outgoing emails in memory, allowing for tests
    /// to later assert the mails were sent.
    pub fn new_in_memory() -> Self {
        Self {
            backend: EmailBackend::Memory(AsyncStubTransport::new_ok()),
            domain: "learn.framer.university".into(),
            from: DEFAULT_FROM.parse().unwrap(),
        }
    }

    /// This is supposed to be used only during tests, to retrieve the messages stored in the
    /// "memory" backend. It's not cfg'd away because our integration tests need to access this.
    pub async fn mails_in_memory(&self) -> Option<Vec<(Envelope, String)>> {
        if let EmailBackend::Memory(transport) = &self.backend {
            Some(transport.messages().await)
        } else {
            None
        }
    }

    fn build_message(
        &self,
        recipient: &str,
        subject: String,
        body: String,
    ) -> Result<Message, EmailError> {
        // The message ID is normally generated by the SMTP server, but if we let it generate the
        // ID there will be no way for the crates.io application to know the ID of the message it
        // just sent, as it's not included in the SMTP response.
        //
        // We do this to allow for finding misdelivered emails.
        let message_id = format!(
            "<{}@{}>",
            Alphanumeric.sample_string(&mut rand::rng(), 32),
            self.domain,
        );

        let from = Mailbox::new(Some(self.domain.clone()), self.from.clone());

        let message = Message::builder()
            .message_id(Some(message_id.clone()))
            .to(recipient.parse()?)
            .from(from)
            .subject(subject)
            .header(ContentType::TEXT_PLAIN)
            .body(body)?;

        Ok(message)
    }

    pub async fn send<E: Email>(&self, recipient: &str, email: E) -> Result<(), EmailError> {
        let email = self.build_message(recipient, email.subject(), email.body())?;

        self.backend
            .send(email)
            .await
            .map_err(EmailError::TransportError)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum EmailError {
    #[error(transparent)]
    AddressError(#[from] lettre::address::AddressError),
    #[error(transparent)]
    MessageBuilderError(#[from] lettre::error::Error),
    #[error(transparent)]
    TransportError(anyhow::Error),
}

#[derive(Debug, Clone)]
enum EmailBackend {
    /// Backend used in production to send mails using SMTP.
    ///
    /// This is using `Box` to avoid a large size difference between variants.
    Smtp(Box<AsyncSmtpTransport<Tokio1Executor>>),
    /// Backend used locally during development, will store the emails in the provided directory.
    FileSystem(Arc<AsyncFileTransport<Tokio1Executor>>),
    /// Backend used during tests, will keep messages in memory to allow tests to retrieve them.
    Memory(AsyncStubTransport),
}

impl EmailBackend {
    async fn send(&self, message: Message) -> anyhow::Result<()> {
        match self {
            EmailBackend::Smtp(transport) => transport.send(message).await.map(|_| ())?,
            EmailBackend::FileSystem(transport) => transport.send(message).await.map(|_| ())?,
            EmailBackend::Memory(transport) => transport.send(message).await.map(|_| ())?,
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct StoredEmail {
    pub to: String,
    pub subject: String,
    pub body: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestEmail;

    impl Email for TestEmail {
        fn subject(&self) -> String {
            "test".into()
        }

        fn body(&self) -> String {
            "test".into()
        }
    }

    #[tokio::test]
    async fn sending_to_invalid_email_fails() {
        let emails = Emails::new_in_memory();

        let address = "String.Format(\"{0}.{1}@live.com\", FirstName, LastName)";
        assert!(emails.send(address, TestEmail).await.is_err());
    }

    #[tokio::test]
    async fn sending_to_valid_email_succeeds() {
        let emails = Emails::new_in_memory();

        let address = "someone@example.com";
        assert!(emails.send(address, TestEmail).await.is_ok());
    }
}
