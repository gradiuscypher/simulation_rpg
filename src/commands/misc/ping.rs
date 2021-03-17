use serenity::prelude::*;
use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};

#[command]
#[description = "Pong!"]
#[num_args(0)]
pub async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}
