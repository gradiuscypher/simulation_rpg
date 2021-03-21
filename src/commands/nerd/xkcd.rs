use crate::infrastructure::util::nerd::*;

use serenity::prelude::*;
use serenity::{
    framework::standard::{
        macros::command, CommandResult
    }, model::channel::Message
};

#[command]
#[description = "Fetch the most recent xkcd comic."]
#[num_args(0)]
pub async fn xkcd(ctx: &Context, msg: &Message) -> CommandResult {
    let xkcd_info = get_xkcd_info().await;
    
    msg.channel_id.send_message(ctx, |m| {
        m.embed(|e| {
            e.title(format!("#{} - {}", &xkcd_info.num, &xkcd_info.title))
            .url("https://xkcd.com/")
            .image(&xkcd_info.img)
            .description(&xkcd_info.date)
            .color((0, 120, 220))
            .footer(|f| {
                f.text(&xkcd_info.alt) 
            })
        })
    })
    .await?;
    
    Ok(())
}
