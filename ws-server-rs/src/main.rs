use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::{
    net::TcpListener,
    sync::{broadcast, Mutex},
};
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};

mod last_messages;
use last_messages::LastMessages;

#[derive(Serialize, Deserialize, Debug)]
struct ClientMessage {
    r#type: MessageType,
    content: Option<String>,
    user_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
enum MessageType {
    Name,
    Message,
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:9001").await.unwrap();
    let (tx, _rx) = broadcast::channel(10);

    // Shared state for holding messages.
    let messages: VecDeque<(usize, String)> = VecDeque::new();
    let shared_messages = Arc::new(Mutex::new(messages));

    // Atomic counter for unique IDs.
    let id_counter = Arc::new(AtomicUsize::new(0));

    // Create a HashMap to store user names associated with their IDs.
    let user_names = Arc::new(Mutex::new(HashMap::new()));

    // Create LastMessages instance to keep track of every sender's last 5 messages
    let last_messages = Arc::new(Mutex::new(LastMessages::new(5)));

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let tx = tx.clone();
        let shared_messages = Arc::clone(&shared_messages);
        let user_names_for_sender = Arc::clone(&user_names);
        let user_names_for_receiver = Arc::clone(&user_names);
        let last_messages = Arc::clone(&last_messages);

        // Assign a unique ID to this connection.
        let id = id_counter.fetch_add(1, Ordering::Relaxed);

        tokio::spawn(async move {
            let ws_stream = accept_async(stream).await.unwrap();
            let (mut ws_sender, mut ws_receiver) = ws_stream.split();

            // Send the last 10 messages to the client when they connect.
            {
                let unknown_user = "Unknown".to_string();
                let messages = shared_messages.lock().await;
                let user_names = user_names_for_sender.lock().await;
                for (sender_id, msg) in messages.iter() {
                    let user_name = user_names.get(sender_id).unwrap_or(&unknown_user);
                    let _ = ws_sender
                        .send(Message::Text(format!("{user_name} ({sender_id}): {msg}")))
                        .await;
                }
            }

            let mut rx = tx.subscribe();

            // Spawn a task for sending messages
            tokio::spawn(async move {
                while let Ok((sender_id, msg)) = rx.recv().await {
                    let user_names = user_names_for_sender.lock().await;
                    let user_name = match user_names.get(&sender_id) {
                        Some(name) => name,
                        None => "Unknown",
                    };
                    let message = format!("{} ({}): {}", user_name, sender_id, msg);
                    let _ = ws_sender.send(Message::Text(message)).await;
                }
            });

            // Receiving messages
            while let Some(msg) = ws_receiver.next().await {
                let msg = msg.unwrap();
                if msg.is_text() || msg.is_binary() {
                    let msg_text = match msg {
                        Message::Text(txt) => txt,
                        Message::Binary(bin) => String::from_utf8_lossy(&bin).to_string(),
                        _ => continue,
                    };

                    // Deserialize the JSON message.
                    let client_message: ClientMessage = serde_json::from_str(&msg_text).unwrap();

                    match client_message.r#type {
                        MessageType::Name => {
                            let mut user_names = user_names_for_receiver.lock().await;
                            if let Some(user_name) = client_message.user_name {
                                user_names.insert(id, user_name);
                            }
                        }
                        MessageType::Message => {
                            if let Some(content) = client_message.content {
                                // Store the received message.
                                let mut messages = shared_messages.lock().await;
                                if messages.len() >= 10 {
                                    messages.pop_front();
                                }
                                messages.push_back((id, content.clone()));

                                // Add message to sender's last messages
                                let mut last_messages = last_messages.lock().await;
                                last_messages.add_message(id, content.clone());

                                // Send the message to all connected clients.
                                tx.send((id, content)).unwrap();
                            }
                        }
                    }
                }
            }
        });
    }
}
