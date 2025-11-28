mod account;
mod ardor_client;
mod args;
mod encryption;
mod config;
mod message;

use crate::account::ArdorAccount;
use crate::ardor_client::ArdorClient;
use crate::args::Args;
use crate::config::Config;
use crate::encryption::encrypt_to_string;
use crate::message::Message;
use chrono::Local;
use clap::Parser;
use dialoguer::{Input, Password};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let (config, account) = load_up();
    let client = reqwest::Client::new();
    let ardor_client = ArdorClient::new(client, &config, &account);

    if let Some(command) = args.command {
        match command {
            args::Command::Start { comment } => {
                let now = Local::now();
                let time = now.format("%H:%M").to_string();
                let recipient = config.get_recipient_id();
                let project = config.get_project();
                let message = Message::new(
                    account.get_account_id().to_string(),
                    "start".to_string(),
                    time.to_string(),
                    format!("{:?}: {:?}", project, comment),
                );
                ardor_client.send_message(recipient, message).await?;
                println!("Starting counter: {time}")
            },
            args::Command::Stop {} => println!("Stopping counter"),
            args::Command::Status {} => println!("status"),
            args::Command::Info {} => account_info(&ardor_client).await,
        }
    } else {
        account_info(&ardor_client).await;
    }

    Ok(())
}

fn load_up() -> (Config, ArdorAccount) {
    if let Some(account) = ArdorAccount::load() {
        let config = Config::load().expect("Error loading config");
        (config, account)
    } else {
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

        let recipient_id: String = Input::new()
            .with_prompt("Project owner account ID (where you will be reporting to)")
            .interact()
            .expect("Failed to read recipient ID");

        let project: String = Input::new()
            .with_prompt("Project ID/name to identify your work")
            .interact()
            .expect("Failed to read project");

        let encrypted_payload = encrypt_to_string(
            secret_phrase.as_bytes(),
            Some(encryption_password.as_bytes()),
        )
            .expect("Encryption failed");
        println!("Encrypting password string...");

        let account = ArdorAccount::new(account_id, encrypted_payload);
        account
            .save()
            .expect("Failed to save account configuration");

        let config = Config::new(recipient_id, project, node_url);
        config
            .save()
            .expect("Failed to save config");

        println!("Account configuration saved successfully!");

        (config, account)
    }
}

async fn account_info(client: &ArdorClient) {
    print!("Connecting to Ardor node...");
    match client.get_account_info().await {
        Ok(_info) => {
            println!("connected!");
        }
        Err(e) => {
            println!("Failed to connect to Ardor node: {}", e);
        }
    }

    match client.get_balance().await {
        Ok(balance) => println!("Your balance: {}", balance.get_balance()),
        Err(e) => {
            println!("Failed to get balance: {}", e);
        }
    }
}
