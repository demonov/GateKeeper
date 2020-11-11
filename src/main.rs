use teloxide::prelude::*;
use teloxide::BotBuilder;
use teloxide::types::UpdateKind;

mod user_profile;
mod kv_store;
mod file_store;

#[tokio::main]
async fn main() {
    teloxide::enable_logging!();
    log::info!("Starting...");

    let token = std::env::var("TOKEN").unwrap_or_else(|_| panic!("Cannot get the TOKEN env variable"));
    let bot = BotBuilder::new().token(token).build();

    // for i in 1..=10 {
    //     let chat_id = chat_id.clone();
    //     let text = format!("This is message #{}", i);
    //     let response = bot.send_message(chat_id, text).send().await;
    //
    //     if response.is_err() {
    //         dbg!(response.unwrap_err());
    //         break;
    //     }

    let mut offset = 0; //todo: optimize offset in loop
    'global_update: loop  {
        log::trace!("getting updates using offset {}", offset);
        let updates = bot.get_updates().limit(1).offset(offset).send().await;
        if updates.is_err() {
            dbg!(updates.unwrap_err());
            continue;
        } else {
            let mut updates = updates.unwrap();
            if updates.len() > 1 {
                panic!("too many updates!")
            }
            if updates.len() == 0 {
                log::trace!("no updates");
                continue;
            }

            let update = updates.pop().unwrap();
            if update.is_err() {
                dbg!(update.unwrap_err());
            } else {
                let update = update.unwrap();
                offset = update.id + 1;

                match update.kind {
                    UpdateKind::Message(m) => {
                        if m.chat.id == 197333640 {
                            match m.text() {
                                Some("sd") => {
                                    log::warn!("shutting down...");

                                    break 'global_update;
                                },
                                _ => {
                                    let _ = bot.send_message(m.chat.id, "Unknown Command").reply_to_message_id(m.id).send().await;
                                },
                            }
                        }
                    },

                    _ => {
                        dbg!(update.kind);
                    },
                }
            }
        }
    }
}