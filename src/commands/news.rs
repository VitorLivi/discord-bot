use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::{model::channel::Message, prelude::*};

use crate::web_scraper::pages::investing::news::InvestingNews;

#[command]
pub async fn news(ctx: &Context, msg: &Message) -> CommandResult {
    let investing_news = InvestingNews::new().await;
    let news = investing_news.get_news();

    for article in news {
        msg.reply(
            ctx,
            format!(
                "## {}\n{}\n{}\nLink para a matéria: \n{}",
                article.title, article.description, article.date, article.link
            ),
        )
        .await?;

        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    Ok(())
}
