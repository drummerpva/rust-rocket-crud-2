use lettre::{
    message::{header::ContentType, MessageBuilder},
    transport::smtp::authentication::Credentials,
    SmtpTransport, Transport,
};
use tera::{Context, Tera};

pub struct HtmlMailer {
    pub template_engine: Tera,
    pub smtp_host: String,
    pub smtp_username: String,
    pub smtp_password: String,
}

impl HtmlMailer {
    pub fn send(self, to: String, template_name: &str, template_context: Context) {
        let html_body = self
            .template_engine
            .render(template_name, &template_context)
            .expect("Error on generate digest html");
        let message = MessageBuilder::new()
            .subject("Cr8s Digest")
            .from("Cr8s <noreply@cr8s.com>".parse().unwrap())
            .to(to.parse().unwrap())
            .header(ContentType::TEXT_HTML)
            .body(html_body)
            .expect("Error on crate message to email with MessageBuidler");
        let credentials = Credentials::new(self.smtp_username, self.smtp_password);
        let mailer = SmtpTransport::relay(&self.smtp_host)
            .expect("Error on create SMTP Transport")
            .credentials(credentials)
            .build();
        mailer.send(&message).expect("Error on send email");
    }
}
