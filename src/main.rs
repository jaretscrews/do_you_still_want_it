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

fn main() {

    // The current method to send emails is a secondary gmail account that I made
    // I simply have the credentials in a secret.json in the root of this project
    // that I extract that info from and pass to lettre so that it can send an email
    let data: String = fs::read_to_string("secret.json")
        .expect("Something went wrong reading secret!");
    let secret: Secret = serde_json::from_str(&data).
        expect("Couldn't create the secret");
    println!("email: {}\npassword: {}", secret.email, secret.password);

    // Lettre setup and a simple test email
    // TODO change this to email actual information
    let smtp_address = "smtp.gmail.com";
    let email = EmailBuilder::new()
        .to(("jaretscrews@gmail.com", "Jaret Screws"))
        .from(secret.email.clone())
        .subject("TEST EMAIL JARET")
        .text("hey there loser")
        .build()
        .unwrap()
        .into();

    let credentials = (secret.email, secret.password).into_credentials();

    let mut client = SmtpClient::new_simple(smtp_address)
        .unwrap()
        .credentials(credentials)
        .transport();

    // Send that boi out
    let result = client.send(email);

    //TODO Log something on failure to send
    match result {
        Ok(_) => println!("email sent"),
        Err(err) => println!("failed to send email alert: {}", err)
    }

}
