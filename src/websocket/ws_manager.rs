use std::collections::HashMap;
use tokio::sync::{mpsc, RwLock};
use warp::ws::Message;

use super::WebSocketMessage;

// Type for message sender channel
pub type Tx = mpsc::UnboundedSender<Message>;

// Shared state for connected clients
#[derive(Debug, Clone)]
pub struct Client {
    // WebSocket is only accessible to logged in users!
    pub user_id: String,
    pub sender: Tx,
}

// Connection manager for tracking all clients
#[derive(Debug)]
pub struct ClientsManager {
    pub clients: RwLock<HashMap<String, Client>>,
}

impl ClientsManager {
    pub fn new() -> Self {
        ClientsManager {
            clients: RwLock::new(HashMap::new()),
        }
    }

    // Register a new client connection
    pub async fn register(&self, tx: Tx, user_id: String) -> Client {
        let client = Client {
            sender: tx,
            // Initially not associated with any user
            user_id,
        };

        self.clients
            .write()
            .await
            .insert(client.user_id.clone(), client.clone());

        // Send a welcome message to the client
        // let welcome_msg = WebSocketMessage::new(
        //     MessageType::SimpleText,
        //     json!({"text": format!("Welcome client {}", id)}),
        // );

        // if let Ok(welcome_json) = serde_json::to_string(&welcome_msg) {
        //     if let Some(client) = self.clients.read().await.get(&id) {
        //         let _ = client.sender.send(Message::text(welcome_json));
        //     }
        // }

        client
    }

    // Unregister a client when they disconnect
    pub async fn unregister(&self, id: String) {
        self.clients.write().await.remove(&id);
    }

    // Broadcast a structured message to all connected clients
    pub async fn broadcast_structured(&self, msg: WebSocketMessage) {
        if let Ok(json_str) = serde_json::to_string(&msg) {
            let clients = self.clients.read().await;
            for (_, client) in clients.iter() {
                if let Err(e) = client.sender.send(Message::text(json_str.clone())) {
                    eprintln!("Failed to send message to client: {}", e);
                    // Client disconnected, but we'll handle cleanup elsewhere
                }
            }
        } else {
            eprintln!("Failed to serialize WebSocketMessage to JSON");
        }
    }

    // // Associate a client connection with a user ID
    // // This should be the id of the user as stored in the database users' table!
    // pub async fn associate_with_user(&self, client_id: ClientId, user_id: String) -> bool {
    //     let mut clients = self.clients.write().await;

    //     if let Some(client) = clients.get_mut(&client_id) {
    //         client.user_id = Some(user_id);
    //         true
    //     } else {
    //         false
    //     }
    // }

    // Find a client by user ID
    // pub async fn find_by_user_id(&self, user_id: &str) -> Option<String> {
    //     let clients = self.clients.read().await;

    //     for (id, client) in clients.iter() {
    //         if client.user_id == user_id {
    //             return Some(id.clone());
    //         }
    //     }

    //     None
    // }
    // Send a structured message to a specific user

    // Send a structured message to a specific client
    // pub async fn send_to_client_structured(
    //     &self,
    //     client_id: String,
    //     msg: WebSocketMessage,
    // ) -> bool {
    //     if let Ok(json_str) = serde_json::to_string(&msg) {
    //         let clients = self.clients.read().await;

    //         if let Some(client) = clients.get(&client_id) {
    //             match client.sender.send(Message::text(json_str)) {
    //                 Ok(_) => return true,
    //                 Err(e) => {
    //                     eprintln!("Failed to send message to client {}: {}", client_id, e);
    //                     return false;
    //                 }
    //             }
    //         }
    //     }
    //     false
    // }

    // // Helper method for sending messages
    // pub async fn send_message(
    //     &self,
    //     user_id: String,
    //     message_type: MessageType,
    //     payload: serde_json::Value,
    // ) -> bool {
    //     let message = WebSocketMessage::new(message_type, payload);

    //     if let Some(cid) = client_id {
    //         self.send_to_client_structured(cid, message).await
    //     } else {
    //         self.broadcast_structured(message).await;
    //         true
    //     }
    // }

    // // Specific message sending methods using the reusable method
    // pub async fn send_user_notification(&self, user_id: &str, payload: serde_json::Value) -> bool {
    //     self.send_message(Some(user_id), None, MessageType::Notification, payload)
    //         .await
    // }

    // pub async fn send_user_text_message(&self, user_id: &str, text: &str) -> bool {
    //     let payload = json!({"text": text});
    //     self.send_message(Some(user_id), None, MessageType::SimpleText, payload)
    //         .await
    // }

    // pub async fn send_notification(&self, client_id: Option<ClientId>, payload: serde_json::Value) {
    //     self.send_message(None, client_id, MessageType::Notification, payload)
    //         .await;
    // }

    // pub async fn notify_waste_scanned(&self, payload: serde_json::Value) {
    //     self.send_message(None, None, MessageType::WasteScanned, payload)
    //         .await;
    // }

    // pub async fn notify_electricity_record_added(&self, payload: serde_json::Value) {
    //     self.send_message(None, None, MessageType::ElectricityRecordAdded, payload)
    //         .await;
    // }

    // pub async fn notify_water_record_added(&self, payload: serde_json::Value) {
    //     self.send_message(None, None, MessageType::WaterRecordAdded, payload)
    //         .await;
    // }

    // pub async fn notify_transportation_trip_added(&self, payload: serde_json::Value) {
    //     self.send_message(None, None, MessageType::TransportationsTripAdded, payload)
    //         .await;
    // }

    // pub async fn send_text_message(&self, client_id: Option<ClientId>, text: &str) {
    //     let payload = json!({"text": text});
    //     self.send_message(None, client_id, MessageType::SimpleText, payload)
    //         .await;
    // }
}
