use crate::commands::{owner::*, misc::*, nerd::*, twitter::*};
use serenity::framework::standard::macros::group;

#[group]
#[commands(quit)]
pub struct Owner;

#[group]
#[commands(ping, stats)]
pub struct Misc;

#[group]
#[commands(apod, xkcd)]
pub struct Nerd;

#[group]
#[commands(add)]
pub struct Twitter;
