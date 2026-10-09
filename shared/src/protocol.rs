use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Envelope {
    pub message_id: String,
    pub from: String,
    pub to: String,
    pub kind: String,
    pub payload: String,
    pub timestamp: String,
    pub signature: String,
}

impl Envelope {
    pub fn new(from: &str, to: &str, kind: &str, payload: &str, secret: &str) -> Self {
        let timestamp = chrono::Utc::now().to_rfc3339();
        let message_id = uuid::Uuid::new_v4().to_string();

        let mut msg = serde_json::json!({
            "message_id": message_id,
            "from": from,
            "to": to,
            "kind": kind,
            "payload": payload,
            "timestamp": timestamp
        });

        let payload_string = msg.to_string();
        let signature = crate::sign_payload(&payload_string, secret);

        Self {
            message_id,
            from: from.to_string(),
            to: to.to_string(),
            kind: kind.to_string(),
            payload: payload.to_string(),
            timestamp,
            signature,
        }
    }

    pub fn verify(&self, secret: &str) -> bool {
        let msg = serde_json::json!({
            "message_id": self.message_id,
            "from": self.from,
            "to": self.to,
            "kind": self.kind,
            "payload": self.payload,
            "timestamp": self.timestamp
        });

        crate::verify_payload(&msg.to_string(), &self.signature, secret)
    }
}