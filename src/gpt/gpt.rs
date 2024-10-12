use chatgpt::prelude::*;
use serenity::{model::channel::Message, prelude::*};
use std::env;

pub struct Gpt {
    client: ChatGPT,
}

impl Gpt {
    pub async fn new() -> Gpt {
        let token = env::var("CHAT_GPT_TOKEN").expect("Expected a token in the environment");
        let max_tokens: u32 = 500;

        Gpt {
            client: ChatGPT::new_with_config(
                token,
                ModelConfigurationBuilder::default()
                    .temperature(1.0)
                    .max_tokens(max_tokens)
                    .engine(ChatGPTEngine::Custom("chatgpt-4o-latest"))
                    .build()
                    .unwrap(),
            )
            .unwrap(),
        }
    }

    fn get_pre_context(&self) -> String {
        "Contexto previo: Haja como um bot no discord chamado \"mula-bot\". \n\n".to_string()
    }

    fn split_text_into_chunks(text: &str, chunk_size: usize) -> Vec<String> {
        let mut chunks = Vec::new();
        let mut current_chunk = String::new();

        for paragraph in text.split("\n\n") {
            if current_chunk.len() + paragraph.len() + 2 > chunk_size {
                chunks.push(current_chunk.trim().to_string());
                current_chunk = String::new();
            }

            if !current_chunk.is_empty() {
                current_chunk.push_str("\n\n");
            }
            current_chunk.push_str(paragraph);

            if current_chunk.len() > chunk_size {
                while current_chunk.len() > chunk_size {
                    let split_point = current_chunk[..chunk_size].rfind(' ').unwrap_or(chunk_size);
                    let chunk = current_chunk[..split_point].to_string();
                    chunks.push(chunk.trim().to_string());
                    current_chunk = current_chunk[split_point..].trim().to_string();
                }
            }
        }

        if !current_chunk.is_empty() {
            chunks.push(current_chunk.trim().to_string());
        }

        chunks
    }

    pub async fn ask(&self, msg: &Message, ctx: &Context) {
        msg.reply(ctx, "Hmmmm 🤔").await;
        let pure_message = msg.content.replace("!gpt", "");
        let result = self
            .client
            .send_message(self.get_pre_context() + &pure_message)
            .await;

        match result {
            Ok(response) => {
                let chunks = Gpt::split_text_into_chunks(&response.message().content, 1000);

                for chunk in chunks {
                    msg.reply(ctx, format!("{}", chunk)).await;
                }
            }
            Err(error) => {
                let error_str = error.to_string();
                let spplited_error = error_str.split_whitespace().collect::<Vec<&str>>();

                let mut error_message = String::new();
                for word in spplited_error.iter() {
                    if error_message.len() + word.len() > 500 {
                        break;
                    }

                    error_message.push_str(word);
                    error_message.push_str(" ");
                }

                msg.reply(ctx, error_message).await;
                println!("Error: {}", error);
            }
        }
    }
}
