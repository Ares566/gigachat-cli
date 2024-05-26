# About
Работа с публичным API GigaChat от Сбербанка

### Request
```rust
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
```
### Response
context: Вопрос задает ребенок 5 лет
```json
  Answer {
        created: 1816845604,
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
```

Без контекста
```json
  Answer {
        created: 1816844508,
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
```