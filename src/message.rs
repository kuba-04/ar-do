use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub sender: String,
    pub command: String,
    pub time: String,
    pub description: String,
}

impl Message {
    pub fn new(sender: String, command: String, time: String, description: String) -> Self {
        Self {
            sender,
            command,
            time,
            description,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Attachment {
    #[serde(rename = "version.Message")]
    pub version_message: i64,
    #[serde(rename = "messageIsText")]
    pub message_is_text: bool,
    pub message: String,
    #[serde(rename = "version.ArbitraryMessage")]
    pub version_arbitrary_message: i64,
}

#[derive(Serialize, Deserialize)]
pub struct TransactionJson {
    #[serde(rename = "senderPublicKey")]
    pub sender_public_key: String,
    pub chain: i64,
    pub signature: String,
    #[serde(rename = "feeNQT")]
    pub fee_nqt: String,
    #[serde(rename = "type")]
    pub r#type: i64,
    #[serde(rename = "fullHash")]
    pub full_hash: String,
    pub version: i64,
    #[serde(rename = "fxtTransaction")]
    pub fxt_transaction: String,
    pub phased: bool,
    #[serde(rename = "ecBlockId")]
    pub ec_block_id: String,
    #[serde(rename = "signatureHash")]
    pub signature_hash: String,
    pub attachment: Attachment,
    #[serde(rename = "senderRS")]
    pub sender_rs: String,
    pub subtype: i64,
    #[serde(rename = "amountNQT")]
    pub amount_nqt: String,
    pub sender: String,
    #[serde(rename = "recipientRS")]
    pub recipient_rs: String,
    pub recipient: String,
    #[serde(rename = "ecBlockHeight")]
    pub ec_block_height: i64,
    pub deadline: i64,
    pub timestamp: i64,
    pub height: i64,
}

#[derive(Serialize, Deserialize)]
pub struct SendMessageResponse {
    #[serde(rename = "minimumFeeFQT")]
    pub minimum_fee_fqt: String,
    #[serde(rename = "signatureHash")]
    pub signature_hash: String,
    #[serde(rename = "transactionJSON")]
    pub transaction_json: TransactionJson,
    #[serde(rename = "unsignedTransactionBytes")]
    pub unsigned_transaction_bytes: String,
    pub broadcasted: bool,
    #[serde(rename = "requestProcessingTime")]
    pub request_processing_time: i64,
    #[serde(rename = "transactionBytes")]
    pub transaction_bytes: String,
    #[serde(rename = "fullHash")]
    pub full_hash: String,
    #[serde(rename = "bundlerRateNQTPerFXT")]
    pub bundler_rate_nqtper_fxt: String,
}