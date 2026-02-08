use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MessageType {
    Notification,
    WasteScanned,
    ElectricityRecordAdded,
    WaterRecordAdded,
    TransportationsTripAdded,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebSocketMessage {
    pub message_type: MessageType,
    pub payload: serde_json::Value,
    pub timestamp: String,
}

impl WebSocketMessage {
    // Helper method to create a new message
    pub fn new(message_type: MessageType, payload: serde_json::Value) -> Self {
        // Format current time in ISO 8601 format
        let now = Utc::now();
        let timestamp = now.to_rfc3339();

        WebSocketMessage {
            message_type,
            payload,
            timestamp,
        }
    }
}
