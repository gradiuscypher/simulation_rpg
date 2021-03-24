use crate::infrastructure::{database::connect::establish_connection, models::twitter::twitter_account::TwitterAccount};
use crate::infrastructure::database::schema::twitter_accounts::dsl::twitter_accounts;

use diesel::prelude::*;
use serenity::prelude::*;
use serenity::{
    framework::standard::{
        macros::command,
        Args,
        CommandResult
    }, model::channel::Message
};

#[command]
#[description = "Add a twitter account to the database."]
#[num_args(1)]
#[owners_only]
pub async fn add(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let account = args.single::<String>()?;
    let new_acct = TwitterAccount { url: &account };
    let conn = establish_connection();

    diesel::insert_into(twitter_accounts)
        .values(&new_acct)
        .execute(&conn)
        .unwrap();

    msg.reply(ctx, "saved!").await?;

    Ok(())
}
