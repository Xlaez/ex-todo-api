use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::authentication::Credentials;

pub async fn email_sender(to: &str, subject: &str, body: &str) -> Result<(), Box<dyn std::error::Error>>{
    let smtp_user: String = std::env::var("SMTP_USER").expect("SMTP_USER must have a value");
    let smtp_password: String = std::env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD  must have a value");
    let smtp_service: String = std::env::var("SMTP_SERVICE").expect("SMTP_SERVICE  must have a value");

    let email = Message::builder()
        .to(to.parse()?)
        .from("ex@blog.com".parse()?)
        .subject(subject)
        .header(lettre::message::header::ContentType::TEXT_HTML)
        .body(body.to_string())?;

    let creds = Credentials::new(smtp_user, smtp_password);

    let mailer = SmtpTransport::relay(&smtp_service)?
        .credentials(creds)
        .build();

    mailer.send(&email)?;

    Ok(())
}

pub async fn send_otp_mail(to: &str, otp: &str, username: &str){
    let html_body = format!("<h1>Hello {username}</h1> <br/> <p>Here is your OTP:  <strong>{otp}</strong>.</p>");

    let subject = "Your OTP";

    match email_sender(to, &subject, &html_body).await {
        Ok(_) => println!("Otp email has been sent to {username}"),
        Err(e) => eprintln!("Failed to send email: {:?}", e),
    }
}