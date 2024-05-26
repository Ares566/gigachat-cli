use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Token {
    pub access_token: String,
    pub expires_at: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Question {
    pub model: String,
    pub messages: Vec<Message>,
    pub max_tokens: u32,
}

impl Question {
    pub fn from_string(message: String, model: String, max_tokens: u32) -> Self {
        Question {
            model,
            messages: vec![Message {
                role: "user".to_string(),
                content: message,
            }],
            max_tokens,
        }
    }

    pub fn from_string_with_context(
        context: String,
        message: String,
        model: String,
        max_tokens: u32,
    ) -> Self {
        Question {
            model,
            messages: vec![
                Message {
                    role: "system".to_string(),
                    content: context,
                },
                Message {
                    role: "user".to_string(),
                    content: message,
                },
            ],
            max_tokens,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Choice {
    pub message: Message,
    pub index: u32,
    pub finish_reason: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Answer {
    pub created: u64,
    pub choices: Vec<Choice>,
}
