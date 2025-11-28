use crate::account::{AccountInfo, ArdorAccount, BalanceResponse};
use reqwest::Client;

pub struct ArdorClient {
    client: Client,
    account: ArdorAccount,
}

impl ArdorClient {
    pub fn new(account: ArdorAccount) -> ArdorClient {
        ArdorClient {
            client: reqwest::Client::new(),
            account,
        }
    }

    pub async fn get_account_info(&self) -> anyhow::Result<AccountInfo> {
        let response = self
            .client
            .post(self.account.get_node())
            .form(&[
                ("requestType", "getAccount"),
                ("account", self.account.get_account_id().as_str()),
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
            .post(self.account.get_node())
            .form(&[
                ("requestType", "getBalance"),
                ("account", self.account.get_account_id().as_str()),
                ("chain", "2"),
            ])
            .send()
            .await?;
        let text = response.text().await?;
        let balance: BalanceResponse = serde_json::from_str(&text)?;
        Ok(balance)
    }
}
