use crate::account::{AccountInfo, ArdorAccount, BalanceResponse};
use crate::config::Config;
use crate::encryption::decrypt_from_string;
use crate::message::{Message, SendMessageResponse};
use dialoguer::Password;
use reqwest::Client;

pub struct ArdorClient {
    client: Client,
    config: Config,
    account: ArdorAccount,
}

impl ArdorClient {
    pub fn new(client: Client, config: &Config, account: &ArdorAccount) -> ArdorClient {
        ArdorClient {
            client,
            config: config.to_owned(),
            account: account.to_owned(),
        }
    }

    pub async fn send_message(&self, recipient: &str, message: Message) -> anyhow::Result<SendMessageResponse> {
        let encrypted_key = self.account.get_secret();
        let private_key;
        if let Ok(key) = decrypt_from_string(encrypted_key, Some("".as_bytes())) {
            private_key = key;
        } else {
            let password: String = Password::new()
                .with_prompt("Password to decrypt the key:")
                .interact()
                .expect("Failed to read password");
            private_key = decrypt_from_string(encrypted_key, Some(password.as_bytes())).expect("Failed to decrypt private key");
        }

        let response = self
            .client
            .post(self.config.get_node_url())
            .form(&[
                ("requestType", "sendMessage"),
                ("chain", "2"), //todo: chain should be defined somewhere else
                ("privateKey", private_key.as_str()),
                ("recipient", recipient),
                ("message", serde_json::to_string(&message).unwrap().as_str()),
            ])
            .send()
            .await;
        if response.is_ok() {
            let resp_txt = response?.text().await?;
            println!("{}", resp_txt);
            let transaction: SendMessageResponse = serde_json::from_str(&resp_txt)?;
            Ok(transaction)
        } else { anyhow::bail!(response.unwrap_err()) }
    }

    pub async fn get_account_info(&self) -> anyhow::Result<AccountInfo> {
        let response = self
            .client
            .post(self.config.get_node_url())
            .form(&[
                ("requestType", "getAccount"),
                ("account", self.account.get_account_id()),
            ])
            .send()
            .await?;
        let text = response.text().await?;
        let account_info: AccountInfo = serde_json::from_str(&text)?;
        Ok(account_info)
    }

    pub async fn get_balance(&self) -> anyhow::Result<BalanceResponse> {
        let response = self
            .client
            .post(self.config.get_node_url())
            .form(&[
                ("requestType", "getBalance"),
                ("account", self.account.get_account_id()),
                ("chain", "2"),
            ])
            .send()
            .await?;
        let text = response.text().await?;
        let balance: BalanceResponse = serde_json::from_str(&text)?;
        Ok(balance)
    }
}
