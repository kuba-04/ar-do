use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ArdorAccount {
    account_id: String,
    secret_phrase: String,
    node_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountInfo {
    #[serde(rename = "forgedBalanceFQT")]
    forged_balance: String,
    #[serde(rename = "accountRS")]
    account_rs: String,
    #[serde(rename = "requestProcessingTime")]
    request_processing_time: i32,
    account: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BalanceResponse {
    #[serde(rename = "unconfirmedBalanceNQT")]
    unconfirmed_balance_nqt: String,
    #[serde(rename = "balanceNQT")]
    balance_nqt: String,
    #[serde(rename = "requestProcessingTime")]
    request_processing_time: i32,
}

impl BalanceResponse {
    pub fn get_balance(&self) -> &str {
        self.balance_nqt.as_str()
    }
}

impl ArdorAccount {
    pub fn new(account_id: String, secret_phrase: String, node_url: String) -> ArdorAccount {
        ArdorAccount {
            account_id,
            secret_phrase,
            node_url,
        }
    }

    pub fn get_node(&self) -> String {
        self.node_url.to_string()
    }

    pub fn get_account_id(&self) -> String {
        self.account_id.to_string()
    }

    fn config_path() -> PathBuf {
        let mut path = PathBuf::new();
        path.push(std::env::var("HOME").unwrap_or_else(|_| ".".to_string()));
        path.push(".todo-ardor");
        path.push("config.json");
        path
    }

    pub fn load() -> Option<Self> {
        let path = Self::config_path();
        if !path.as_path().exists() {
            return None;
        }
        fs::read_to_string(path)
            .ok()
            .and_then(|content| serde_json::from_str(&content).ok())
    }

    pub fn save(&self) -> std::io::Result<()> {
        let path = Self::config_path();
        fs::create_dir_all(path.as_path().parent().unwrap())?;
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)
    }
}
