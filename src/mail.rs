use std::fs::File;
use std::io::Read;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use lettre::message::{header, MultiPart, SinglePart};

pub fn send_file_message(message_text: &str, file_path: &str) {
    let smtp_server = "smtp.gmail.com";
    let smtp_username = std::env::var("MAIL").expect("Error getting MAIL env var");
    let smtp_password = std::env::var("MAIL_PASSWORD").expect("Error getting MAIL_PASSWORD env var");

    let creds = Credentials::new(smtp_username.to_string(), smtp_password.to_string());
    let mailer = SmtpTransport::relay(smtp_server)
        .unwrap()
        .credentials(creds)
        .build();

    let mut file = File::open(file_path).expect("Could not open file");
    let mut file_content = Vec::new();
    file.read_to_end(&mut file_content).expect("Could not read file");
    let file_name = file_path.split('/').last().unwrap();

    let email = Message::builder()
        .from(format!("Sender Name <{smtp_username}>").parse().expect("Error parsing email"))
        .to(format!("Recipient Name <{smtp_username}>").parse().expect("Error parsing email"))
        .subject("File from site sent")
        .multipart(
            MultiPart::mixed()
                .singlepart(
                    SinglePart::builder()
                        .header(header::ContentType::TEXT_PLAIN)
                        .body(String::from(message_text)),
                )
                .singlepart(
                    SinglePart::builder()
                        .header(header::ContentType::parse("application/octet-stream").unwrap())
                        .header(header::ContentDisposition::attachment(file_name))
                        .body(file_content),
                ),
        )
        .expect("Error configuring email");

   mailer.send(&email).expect("Error sending the email");
}