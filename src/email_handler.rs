use std::fs;
use serde::{Deserialize, Serialize};
use lettre::smtp::authentication::IntoCredentials;
use lettre::{SmtpClient, Transport};
use lettre_email::EmailBuilder;

// Struct to hold my secret in for the gmail account
#[derive(Serialize, Deserialize)]
struct Secret {
    email: String,
    password: String,
}

pub struct EmailHandler {
    client: SmtpClient,
    from: String,
    recipient_addr: String,
    recipient_name: String,
}

impl EmailHandler {
    pub fn new() -> Self {
        let data: String = fs::read_to_string("secret.json")
            .expect("Something went wrong reading secret!");
        let secret: Secret = serde_json::from_str(&data)
            .expect("Couldn't create the secret");
        let credentials = (secret.email.clone(), secret.password).into_credentials();
        let smtp_address = "smtp.gmail.com";
        let client = SmtpClient::new_simple(smtp_address)
            .unwrap()
            .credentials(credentials);
        return EmailHandler {
            client: client,
            from: secret.email,
            recipient_addr: "jaretscrews@gmail.com".to_string(),
            recipient_name: "Jaret Screws".to_string(),
        }
    }
    
    pub fn test_email(&self) {
        let email = EmailBuilder::new()
            .to((self.recipient_addr.clone(), self.recipient_name.clone()))
            .from(self.from.clone())
            .subject("TEST EMAIL JARET")
            .text("hey there loser")
            .build()
            .unwrap()
            .into();
        let mut transport = self.client.clone().transport();
        let result = transport.send(email);
        match result {
            Ok(_) => println!("email sent"),
            Err(err) => println!("failed to send email alert: {}", err)
        }
        
    }
}
