use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::{model::channel::Message, prelude::*};

use crate::gpt::gpt::Gpt;

#[command]
pub async fn gpt(ctx: &Context, msg: &Message) -> CommandResult {
    let gpt = Gpt::new().await;
    gpt.ask(msg, ctx).await;

    Ok(())
}
