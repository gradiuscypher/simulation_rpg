use crate::commands::{owner::*, misc::*, nerd::*};
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
