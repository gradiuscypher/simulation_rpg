use crate::infrastructure::util::nerd::*;
use crate::infrastructure::util::general::*;

use serenity::prelude::*;
use serenity::{
    framework::standard::{
        macros::command, CommandResult
    }, model::channel::Message
};

#[command]
#[description = "Fetch NASA's Astronomy Picture of the Day."]
#[num_args(0)]
pub async fn apod(ctx: &Context, msg: &Message) -> CommandResult {

    let apod_info = get_apod_info().await;
    
    msg.channel_id.send_message(ctx, |m| {
        m.embed(|e| {
            e.title(apod_info.title)
            .image(apod_info.img)
            .description(apod_info.desc)
            .color((0, 120, 220))
            .footer(|f| {
                f.text(get_date() + " - NASA Astronomy Picture of the Day") 
            })
        })
    })
    .await?;
    
    Ok(())
}
