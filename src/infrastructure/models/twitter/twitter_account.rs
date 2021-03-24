use crate::infrastructure::database::schema::twitter_accounts;

#[derive(Insertable)]
#[table_name = "twitter_accounts"]
pub struct TwitterAccount<'a> {
    pub url: &'a str,
}
