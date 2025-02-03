use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub sender: String,
    pub recipient: String,
    pub amount: f64,
    pub timestamp: String,
}

impl Transaction {
    pub fn new(sender: String, recipient: String, amount: f64) -> Self {
        let timestamp = Utc::now().to_rfc3339();
        Transaction {
            sender,
            recipient,
            amount,
            timestamp,
        }
    }
}
