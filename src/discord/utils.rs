use super::*;

use serenity::all::ChannelId;
use serenity::builder::GetMessages;

pub async fn clear_messages(ctx: &Context, channel_id: ChannelId) {
    let mut retriever = GetMessages::new().limit(100);
    let mut messages = channel_id.messages(&ctx.http, retriever).await.unwrap();

    while !messages.is_empty() {
        let message_ids: Vec<_> = messages.iter().map(|m| m.id).collect();

        if let Err(why) = channel_id.delete_messages(&ctx.http, message_ids).await {
            eprintln!("[ERROR]: could not delete mesages: {why:?}");
        }

        retriever = GetMessages::new().before(messages.last().unwrap().id).limit(100);
        messages = channel_id.messages(&ctx.http, retriever).await.unwrap();
    }
}
