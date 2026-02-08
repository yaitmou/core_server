use futures::{SinkExt, StreamExt};
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::sync::mpsc;
use warp::ws::{Message, WebSocket};

use crate::api::auth::domain::entities::Claims;

use super::{Client, ClientsManager, MessageType, WebSocketMessage};

// Handle a WebSocket connection
pub async fn handle_ws_client(ws: WebSocket, clients_manager: Arc<ClientsManager>, claims: Claims) {
    // Split the socket into a sender and receiver
    let (mut ws_sender, mut ws_receiver) = ws.split();

    // Use an unbounded channel to handle buffering and flushing of messages
    let (tx, mut rx) = mpsc::unbounded_channel();

    // Spawn a task that forwards messages from rx to the WebSocket sender
    let sender_task = tokio::spawn(async move {
        while let Some(message) = rx.recv().await {
            match ws_sender.send(message).await {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("WebSocket send error: {}", e);
                    break;
                }
            }
        }
    });

    // Register the client
    let client = clients_manager.register(tx, claims.user_id).await;

    println!("Client {} connected", client.user_id);

    // Handle incoming WebSocket messages
    while let Some(result) = ws_receiver.next().await {
        match result {
            Ok(msg) => {
                // Process the message from the client
                handle_client_message(client.clone(), msg, &clients_manager).await;
            }
            Err(e) => {
                // Handle disconnection error more gracefully
                if e.to_string().contains("Connection reset")
                    || e.to_string().contains("protocol error")
                {
                    println!("Client {} disconnected abruptly", client.user_id);
                } else {
                    eprintln!("WebSocket error({}): {}", client.user_id, e);
                }
                break;
            }
        };
    }

    // Client disconnected - cancel the sender task too
    sender_task.abort();

    // Unregister the client
    clients_manager.unregister(client.user_id.clone()).await;
    println!("Client {} disconnected", client.user_id);
}

// Process incoming messages from the client
async fn handle_client_message(client: Client, msg: Message, clients: &ClientsManager) {
    // create a default message type...for testing
    // Skip processing for non-Text messages
    if !msg.is_text() {
        return;
    }

    // Get the message content as a string
    let message = match msg.to_str() {
        Ok(v) => v,
        Err(_) => return,
    };

    // // Try to parse as JSON first
    // if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(message) {
    //     // Check if this is an authentication message
    //     // This is basically the logging process at the ws side!
    //     if json_value.get("type").and_then(|t| t.as_str()) == Some("authenticate") {
    //         if let Some(data) = json_value.get("data") {
    //             // Handling the authentication process (as if it was a basic login!)
    //             handle_authentication(client_id, data.clone(), clients).await;
    //             return;
    //         }
    //     }
    // }

    // Try to parse as a structured message
    match serde_json::from_str::<WebSocketMessage>(message) {
        Ok(structured_msg) => {
            // Handle the structured message based on its type

            handle_structured_message(client.user_id, structured_msg, clients).await;
        }
        Err(_) => {
            // Not a structured message, try to parse as a simple JSON object
            match serde_json::from_str::<Value>(message) {
                Ok(json_value) => {
                    // Create a SimpleText message with the JSON content
                    let structured_msg =
                        WebSocketMessage::new(MessageType::Notification, json_value);
                    handle_structured_message(client.user_id, structured_msg, clients).await;
                }
                Err(_) => {
                    // Not JSON at all, treat as plain text
                    let text_msg =
                        WebSocketMessage::new(MessageType::Notification, json!({"text": message}));
                    handle_structured_message(client.user_id, text_msg, clients).await;
                }
            }
        }
    }
}

// // Handle a new message that contains authentication information
// async fn handle_authentication(
//     client_id: ClientId,
//     auth_data: serde_json::Value,
//     clients: &ClientsManager,
// ) -> bool {
//     // Extract user ID from authentication data
//     if let Some(user_id) = auth_data.get("user_id").and_then(|v| v.as_str()) {
//         // Here you would normally validate the auth token or credentials
//         // For this example, we'll simply trust the provided user_id

//         // Associate this connection with the user ID
//         if clients
//             .associate_with_user(client_id, user_id.to_string())
//             .await
//         {
//             // Send a confirmation message
//             let payload =
//                 json!({"status": "authenticated", "message": "Authentication successful"});
//             let msg = WebSocketMessage::new(MessageType::Notification, payload);
//             clients.send_to_client_structured(client_id, msg).await;
//             return true;
//         }
//     }

//     // Authentication failed
//     let payload = json!({"status": "error", "message": "Authentication failed"});
//     let msg = WebSocketMessage::new(MessageType::Notification, payload);
//     clients.send_to_client_structured(client_id, msg).await;
//     false
// }

// Handle a structured message based on its type
async fn handle_structured_message(
    client_id: String,
    msg: WebSocketMessage,
    clients: &ClientsManager,
) {
    println!(
        "Received structured message from client {}: {:?}",
        client_id, msg
    );

    // Process the message based on its type
    match msg.message_type {
        MessageType::Notification => {
            // Process notification
            println!("Notification from client {}: {:?}", client_id, msg.payload);
            // Forward to other clients if needed
            clients.broadcast_structured(msg).await;
        }
        MessageType::WasteScanned => {
            println!("Waste scanned by client {}: {:?}", client_id, msg);
            // Process waste scanning data
            // You might save this to MongoDB and then broadcast to all clients
            // clients.send_to_client_structured(client_id, msg).await;
            clients.broadcast_structured(msg).await;
        }
        MessageType::ElectricityRecordAdded => {
            println!(
                "Electricity record added by client {}: {:?}",
                client_id, msg.payload
            );
            // Process electricity record data
            clients.broadcast_structured(msg).await;
        }
        MessageType::WaterRecordAdded => {
            println!(
                "Water record added by client {}: {:?}",
                client_id, msg.payload
            );
            // Process water record data
            clients.broadcast_structured(msg).await;
        }
        MessageType::TransportationsTripAdded => {
            println!(
                "Transportation trip added by client {}: {:?}",
                client_id, msg.payload
            );
            // Process transportation trip data
            clients.broadcast_structured(msg).await;
        }
    }
}
