use chrono::DateTime;
use chrono::prelude::*;

pub fn get_date() -> String {
    let local: DateTime<Local> = Local::now();
    return local.format("%A, %e %B %Y").to_string();
}
