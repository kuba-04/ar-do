use dialoguer::{Input, Password};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

const ARDOR_NODE_URL: &str = "https://testardor.jelurida.com/nxt";

#[derive(Debug, Serialize, Deserialize)]
struct ArdorAccount {
    account_id: String,
    secret_phrase: String,
}

impl ArdorAccount {
    fn config_path() -> PathBuf {
        let mut path = PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| ".".to_string()));
        path.push(".todo-ardor");
        path.push("config.json");
        path
    }

    fn load() -> Option<Self> {
        let path = Self::config_path();
        if !path.exists() {
            return None;
        }
        fs::read_to_string(path)
            .ok()
            .and_then(|content| serde_json::from_str(&content).ok())
    }

    fn save(&self) -> std::io::Result<()> {
        let path = Self::config_path();
        fs::create_dir_all(path.parent().unwrap())?;
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)
    }

    async fn get_account_info(&self) -> Result<AccountInfo, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let response = client
            .post(ARDOR_NODE_URL)
            .form(&[
                ("requestType", "getAccount"),
                ("account", &self.account_id),
            ])
            .send()
            .await?;

        let text = response.text().await?;

        if let Ok(error) = serde_json::from_str::<ErrorResponse>(&text) {
            return Err(format!(
                "Ardor API Error: {} (code: {}) for account: {}",
                error.error_description, error.error_code, error.account_rs
            )
            .into());
        }

        // If it's not an error, parse as account info
        let account_info: AccountInfo = serde_json::from_str(&text)?;
        Ok(account_info)
    }

    async fn get_balance(&self) -> Result<BalanceResponse, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let response = client
            .post(ARDOR_NODE_URL)
            .form(&[
                ("requestType", "getBalance"),
                ("account", &self.account_id),
                ("chain", "2"),
            ])
            .send()
            .await?;

        let text = response.text().await?;

        if let Ok(error) = serde_json::from_str::<GeneralErrorResponse>(&text) {
            return Err(format!(
                "Ardor API Error: {} (code: {})",
                error.error_description, error.error_code
            )
            .into());
        }

        let balance: BalanceResponse = serde_json::from_str(&text)?;
        Ok(balance)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct ErrorResponse {
    #[serde(rename = "errorDescription")]
    error_description: String,
    #[serde(rename = "errorCode")]
    error_code: i32,
    #[serde(rename = "accountRS")]
    account_rs: String,
    account: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct GeneralErrorResponse {
    #[serde(rename = "errorDescription")]
    error_description: String,
    #[serde(rename = "errorCode")]
    error_code: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct AccountInfo {
    #[serde(rename = "forgedBalanceFQT")]
    forged_balance: String,
    #[serde(rename = "accountRS")]
    account_rs: String,
    #[serde(rename = "requestProcessingTime")]
    request_processing_time: i32,
    account: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct BalanceResponse {
    #[serde(rename = "unconfirmedBalanceNQT")]
    unconfirmed_balance_nqt: String,
    #[serde(rename = "balanceNQT")]
    balance_nqt: String,
    #[serde(rename = "requestProcessingTime")]
    request_processing_time: i32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to To-do on ardor!");
    println!("==========================");

    let account = match ArdorAccount::load() {
        Some(account) => {
            println!("Loaded existing account: {}", account.account_id);
            account
        }
        None => {
            println!("No account found. Please set up your Ardor account.");
            println!("Enter your account details below:");

            let account_id: String = Input::new()
                .with_prompt("Ardor Account ID")
                .interact()
                .expect("Failed to read account ID");

            let secret_phrase: String = Password::new()
                .with_prompt("Secret Phrase")
                .interact()
                .expect("Failed to read secret phrase");

            let account = ArdorAccount {
                account_id,
                secret_phrase,
            };

            account
                .save()
                .expect("Failed to save account configuration");
            println!("Account configuration saved successfully!");
            account
        }
    };

    print!("Connecting to Ardor node...");
    match account.get_account_info().await {
        Ok(info) => {
            println!("connected!");
        }
        Err(e) => {
            println!("Failed to connect to Ardor node: {}", e);
            println!("Please make sure the node is running at {}", ARDOR_NODE_URL);
        }
    }

    match account.get_balance().await {
        Ok(balance) => println!("Your balance: {}", balance.balance_nqt),
        Err(e) => {
            println!("Failed to get balance: {}", e);
        }
    }

    Ok(())
}
