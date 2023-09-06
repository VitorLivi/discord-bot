use dotenv::dotenv;
use std::env;

use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{CommandResult, StandardFramework};
use serenity::{async_trait, model::channel::Message, prelude::*};

use discord_bot::commands::gpt::GPT_COMMAND;
use discord_bot::commands::news::NEWS_COMMAND;
use discord_bot::commands::quotes::QUOTES_COMMAND;

const HELP_MESSAGE: &str = "
    Comandos do mula-bot:
    !help - Mostra essa mensagem
    !ping - Pong!
    !quotes - Mostra a cotação de uma ação
    !news - Mostra as últimas notícias do mercado financeiro
    !gpt - Pergunte algo para o mula-bot
";

#[group]
#[commands(ping, help, quotes, news, gpt)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!")) // set the bot's prefix to "!"
        .group(&GENERAL_GROUP);

    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}

#[command]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, HELP_MESSAGE).await?;

    Ok(())
}
