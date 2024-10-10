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

    pub async fn ask(&self, msg: &Message, ctx: &Context) {
        msg.reply(ctx, "Espera ai, deixa eu pensar! 🤔").await;
        let result = self.client.send_message(msg.content.clone()).await;

        match result {
            Ok(response) => {
                msg.reply(ctx, response.message().content.to_string()).await;
            }
            Err(error) => {
                println!("Error: {}", error);
            }
        }
    }
}
