use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::{model::channel::Message, prelude::*};
use yahoo_finance_api as yahoo;

enum StockCodeOutput {
    InvalidStockCode { message: String },
    InvalidArgument { message: String },
    Result { code: String },
}

fn get_stock_code(msg: &Message) -> Option<StockCodeOutput> {
    let spplited_message = msg.content.split_whitespace().collect::<Vec<&str>>();

    if spplited_message.len() > 2 {
        return Some(StockCodeOutput::InvalidArgument {
            message: "Informe um código por vez sua mula!".into(),
        });
    } else {
        match spplited_message.get(1) {
            Some(result) => {
                return Some(StockCodeOutput::Result {
                    code: format!("{}.SA", result.to_string()),
                });
            }
            None => {
                return Some(StockCodeOutput::InvalidStockCode {
                    message: "Informe um código válido sua mula!".into(),
                });
            }
        }
    }
}

#[command]
pub async fn quotes(ctx: &Context, msg: &Message) -> CommandResult {
    let stock_code: String;
    match get_stock_code(msg) {
        Some(StockCodeOutput::InvalidStockCode { message }) => {
            msg.reply(ctx, message).await?;
            return Ok(());
        }
        Some(StockCodeOutput::InvalidArgument { message }) => {
            msg.reply(ctx, message).await?;
            return Ok(());
        }
        Some(StockCodeOutput::Result { code }) => {
            stock_code = code;
        }
        None => {
            msg.reply(ctx, "Informe um código válido sua mula!").await?;
            return Ok(());
        }
    }

    let provider = yahoo::YahooConnector::new();
    let response = provider.get_latest_quotes(stock_code.as_ref(), "1d").await;

    let last_close_quote;
    let open_quote;
    let last_update;
    match response.unwrap().last_quote() {
        Ok(result) => {
            println!("{:#?}", result);

            last_update = result.timestamp;
            last_close_quote = result.adjclose;
            open_quote = result.open;
        }
        Err(_) => {
            msg.reply(ctx, "Informe um código válido sua mula!").await?;
            return Ok(());
        }
    }

    let open_quote_unavailable = open_quote == 0.0;

    let icon;
    if open_quote_unavailable {
        icon = "";
    } else if last_close_quote < open_quote {
        icon = "📉"
    } else {
        icon = "📈"
    }

    let last_close_quote = format!("{:.2}", last_close_quote);
    let open_quote = if open_quote_unavailable {
        "Não disponível".to_string()
    } else {
        format!("{:.2}", open_quote)
    };

    let last_update = chrono::NaiveDateTime::from_timestamp_opt(last_update as i64, 0);
    let last_update = last_update.unwrap() - chrono::Duration::hours(3);
    let last_update = last_update.format("%d/%m/%Y %H:%M:%S").to_string();

    let response_string = format!(
        "## {} \n Abertura: {} \n Ultimo Fechamento: {} {} \n Ultima Atualização: {} \n -----------------------------",
        stock_code.to_uppercase(),
        open_quote,
        last_close_quote,
        icon,
        last_update
    );

    msg.reply(ctx, response_string).await?;
    Ok(())
}
