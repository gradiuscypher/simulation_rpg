use chrono::{Duration, Utc};
use chrono_humanize::{Accuracy, HumanTime, Tense};
use futures::future::join_all;
use serenity::prelude::*;
use serenity::{
    framework::standard::{
        macros::command, CommandResult
    }, model::channel::Message
};

#[command]
#[description = "Display your discord stats."]
#[num_args(0)]
pub async fn stats(ctx: &Context, msg: &Message) -> CommandResult {
    let user = &msg.author; 

    if let Some(guild_id) = msg.guild_id {
        let member = guild_id.member(ctx, user.id).await?;

        let joined_at = member.joined_at
            .map(|time| time.format("%A, %e %B %Y").to_string())
            .unwrap_or_else(|| "Unknown".to_string());
        
        let joined_at_ago = member.joined_at
            .map(|time| {
                Utc::now()
                    .signed_duration_since(time)
                    .num_days()
                    .to_string()
            })
            .unwrap_or_else(|| "Unknown".to_string());

        let joined_at_ago_human = HumanTime::from(Duration::days(joined_at_ago.parse::<i64>().unwrap()))
            .to_text_en(Accuracy::Rough, Tense::Present);

        let joined_discord = user.created_at()
            .format("%A, %e %B %Y").to_string();

        let joined_discord_ago = Utc::now()
            .signed_duration_since(user.created_at())
            .num_days();
        
        let joined_discord_ago_human = HumanTime::from(Duration::days(joined_discord_ago))
            .to_text_en(Accuracy::Rough, Tense::Present);

        let roles: Vec<String> = join_all(member.roles.iter().map(|r| r.to_role_cached(&ctx.cache)))
            .await
            .into_iter()
            .filter_map(|r| r.map(|r| r.name))
            .collect();

        msg.channel_id.send_message(ctx, |m| {
            m.embed(|e| {
                e.title(format!("{}#{}", user.name, user.discriminator))
                    .thumbnail(&user.static_avatar_url().unwrap_or_default())
                    .color((0, 120, 220))
                    .field("Joined Server", format!("{}\n*{} ago*", joined_at, joined_at_ago_human), true)
                    .field("Joined Discord", format!("{}\n*{} ago*", joined_discord, joined_discord_ago_human), true)
                    .field("Roles", roles.join(", "), false)
            })
        })
        .await?;
    }

    Ok(())
}
