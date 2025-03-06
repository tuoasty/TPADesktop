use serde::{Deserialize, Serialize};
use reqwest;
use std::error::Error;
use chrono::Utc;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Message {
    pub text: String,
    pub sender: String,
    pub timestamp: i64,
}

pub struct FirebaseHandler {
    base_url: String,
    last_fetch_timestamp: i64,
}

impl FirebaseHandler {
    pub fn new() -> Self {
        FirebaseHandler {
            base_url: "https://tpadesktop-fdf5e-default-rtdb.firebaseio.com/".to_string(),
            last_fetch_timestamp: 0,
        }
    }

    pub async fn send_message(&self, message: Message, group:String) -> Result<(), Box<dyn Error>> {
        let client = reqwest::Client::new();
        let url = format!("{}{}chats.json", self.base_url, group);

        client.post(&url)
            .json(&message)
            .send()
            .await?;

        Ok(())
    }

    pub async fn get_messages(&mut self, group:String) -> Result<Vec<Message>, Box<dyn Error>> {
        let client = reqwest::Client::new();
        let url = format!("{}{}chats.json", self.base_url, group);

        let response = client.get(&url)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        let mut messages: Vec<Message> = Vec::new();

        if let Some(messages_obj) = response.as_object() {
            for (_, msg_value) in messages_obj.iter() {
                if let Ok(message) = serde_json::from_value::<Message>(msg_value.clone()) {
                    if message.timestamp > self.last_fetch_timestamp {
                        messages.push(message.clone());
                    }
                }
            }
        }

        if let Some(latest_message) = messages.iter().max_by_key(|m| m.timestamp) {
            self.last_fetch_timestamp = latest_message.timestamp;
        }


        messages.sort_by_key(|m| m.timestamp);

        Ok(messages)
    }
}

#[tauri::command]
pub async fn send_chat_message(text: String, sender: String, group:String) -> Result<(), String> {
    let handler = FirebaseHandler::new();
    let message = Message {
        text,
        sender,
        timestamp: Utc::now().timestamp(),
    };

    handler.send_message(message, group).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn fetch_new_messages(group:String) -> Result<Vec<Message>, String> {
    let mut handler = FirebaseHandler::new();
    handler.get_messages(group).await.map_err(|e| e.to_string())
}