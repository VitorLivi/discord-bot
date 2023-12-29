use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::{model::channel::Message, prelude::*};

use crate::tabnews::tabnews::TabNews;

#[command]
pub async fn tabnews(ctx: &Context, msg: &Message) -> CommandResult {
    let tabnews_instance = TabNews::new();
    let news = tabnews_instance.get_relevants().await?;

    for article in news {
        println!("## {}\n{}\n{}\n", article.title, article.body, article.user);

        msg.reply(
            ctx,
            format!("## {}\n{}\n{}\n", article.title, article.body, article.user)
                .chars()
                .take(1700)
                .collect::<String>()
                + format!("\n\n{}", article.url).as_str(),
        )
        .await?;

        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    Ok(())
}
