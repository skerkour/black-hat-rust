use anyhow::Error;
use lettre::{
    transport::smtp::authentication::Credentials, AsyncSmtpTransport, Message, Tokio1Executor,
};
use rusoto_ses::SesClient;

mod ses;
mod smtp;
mod template;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // email data
    let from = "evil@hacker.com".to_string();
    let to = "credule@kerkour.com".to_string();
    let subject = "".to_string();
    let title = subject.clone();
    let content = "".to_string();

    // template things
    let mut templates = tera::Tera::default();
    // don't escape input as it's provided by us
    templates.autoescape_on(Vec::new());
    templates.add_raw_template("email", template::EMAIL_TEMPLATE)?;

    let email_data = tera::Context::from_serialize(template::EmailData { title, content })?;
    let html = templates.render("email", &email_data)?;

    let email = Message::builder()
        .from(from.parse()?)
        .to(to.parse()?)
        .subject(subject)
        .body(html.to_string())?;

    // Using SMTP
    let smtp_credentials =
        Credentials::new("smtp_username".to_string(), "smtp_password".to_string());

    let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay("smtp.email.com")?
        .credentials(smtp_credentials)
        .build();

    smtp::send_email(&mailer, email.clone()).await?;

    // or using SES
    // load credentials from env
    let ses_client = SesClient::new(rusoto_core::Region::UsEast1);
    ses::send_email(&ses_client, email).await?;

    Ok(())
}
