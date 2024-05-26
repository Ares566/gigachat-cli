use std::env;
use std::error::Error;
use crate::gigachat::client::GigaChatClient;
use crate::gigachat::types::{Question};

mod gigachat;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let giga_chat_secret = env::var("gigachat-secret")?; // Авторизационные данные
    let mut giga_client = GigaChatClient::new(giga_chat_secret);

    let context = "Вопрос задает ребенок 5 лет";
    let question = "Ты кто?";

    let question = Question::from_string_with_context(
        context.to_string(),
        question.to_string(),
        "GigaChat:latest".to_string(),
        512,
    );

    let response = giga_client.ask(question).await.unwrap();
    println!("{:#?}", response);

    /*Answer with out context {
        created: 1716744508,
        choices: [
            Choice {
                message: Message {
                    role: "assistant",
                    content: "Я — генеративная языковая модель, разработанная компанией Сбером. Могу отвечать на вопросы пользователей, поддерживать беседу и генерировать тексты.",
                },
                index: 0,
                finish_reason: "stop",
            },
        ],
    }
    Answer context: Вопрос задает ребенок 5 лет{
        created: 1716745604,
        choices: [
            Choice {
                message: Message {
                    role: "assistant",
                    content: "Я - компьютерная программа, которая может отвечать на вопросы и помогать в различных задачах.",
                },
                index: 0,
                finish_reason: "stop",
            },
        ],
    }

    */

    Ok(())
}
