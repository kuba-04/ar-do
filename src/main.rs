mod account;
mod encryption;
mod args;

use crate::account::ArdorAccount;
use crate::args::Args;
use crate::encryption::encrypt_to_string;
use clap::Parser;
use dialoguer::{Input, Password};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let account = load_account();

    match args.command {
        Some(command) => match command {
            args::Command::Start {  } => println!("Starting counter"),
            args::Command::Stop {  } => println!("Stopping counter"),
            args::Command::Status {  } => println!("status"),
            args::Command::Info {  } => account_info(&account).await,
        }
        None => {}
    }

    Ok(())
}

fn load_account() -> ArdorAccount {
    match ArdorAccount::load() {
        Some(account) => {
            account
        }
        None => {
            println!("No account found. Please set up your Ardor account.");
            println!("Enter your account details below:");

            let node_url: String = Input::new()
                .with_prompt("Ardor Node URL")
                .interact()
                .expect("Failed to read node url");

            let account_id: String = Input::new()
                .with_prompt("Ardor Account ID")
                .interact()
                .expect("Failed to read account ID");

            let secret_phrase: String = Password::new()
                .with_prompt("Secret Phrase")
                .interact()
                .expect("Failed to read secret phrase");

            let encryption_password: String = Password::new()
                .with_prompt("Encryption Password")
                .interact()
                .expect("Failed to read encryption password");

            let encrypted_payload = encrypt_to_string(
                secret_phrase.as_bytes(),
                Some(encryption_password.as_bytes())
            ).expect("Encryption failed");
            println!("Encrypting password string...");

            let account = ArdorAccount::new(account_id, encrypted_payload, node_url);
            account.save().expect("Failed to save account configuration");
            println!("Account configuration saved successfully!");

            account
        }
    }
}

async fn account_info(account: &ArdorAccount) {
    print!("Connecting to Ardor node...");
    match account.get_account_info().await {
        Ok(_info) => {
            println!("connected!");
        }
        Err(e) => {
            println!("Failed to connect to Ardor node: {}", e);
        }
    }

    match account.get_balance().await {
        Ok(balance) => println!("Your balance: {}", balance.get_balance()),
        Err(e) => {
            println!("Failed to get balance: {}", e);
        }
    }
}
