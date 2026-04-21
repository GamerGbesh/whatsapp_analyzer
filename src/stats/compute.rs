use chrono::{Datelike, NaiveDate};

use crate::models::{result::{UserStat, WhatsResult}, user::User};
use std::collections::HashMap;


pub fn total_chats(user: &User) -> usize {
    user.chats.len()
}


pub fn messages_per_day(user: &User) -> HashMap<NaiveDate, usize> {

    let mut map = HashMap::new();

    for chat in &user.chats {
        *map.entry(chat.date).or_insert(0) += 1;
    }

    map
}

pub fn avg_message_length(user: &User) -> f64 {

    let total: usize = user.chats
        .iter()
        .map(|c| c.message.len())
        .sum();

    total as f64 / user.chats.len() as f64
}

pub fn busiest_day(map: &HashMap<NaiveDate, usize>) -> Option<(NaiveDate, usize)> {
    map.iter()
        .max_by_key(|(_, count)| *count)
        .map(|(date, count)| (*date, *count))
}

pub fn first_message_last_message(user: &User) -> Option<(NaiveDate, NaiveDate)> {
    Some((user.chats.first()?.date, user.chats.last()?.date))
}


pub fn messages_per_month(user: &User) -> HashMap<u32, usize> {

    let mut map = HashMap::new();

    for chat in &user.chats {
        *map.entry(chat.date.month()).or_insert(0) += 1;
    }

    map
}

pub fn busiest_month(map: &HashMap<u32, usize>) -> Option<(u32, usize)> {
    map.iter()
        .max_by_key(|(_, count)| *count)
        .map(|(month, count)| (*month, *count))
}

pub fn most_active_user(users: &[User]) -> Option<String> {
    users.into_iter()
        .max_by_key(|u| u.chats.len())
        .map(|u| u.name.clone())
}

pub fn get_result(user: User) -> UserStat{
    let total_chats = total_chats(&user);
    let messages_per_day = messages_per_day(&user);
    let messages_per_month = messages_per_month(&user);
    let avg_message_length = avg_message_length(&user);
    let (first_message, last_message) =
    first_message_last_message(&user).map(|(f, l)| (Some(f), Some(l))).unwrap_or((None, None));    let busiest_day = busiest_day(&messages_per_day);
    let busiest_month = busiest_month(&messages_per_month);
    UserStat { 
        user, 
        messages_per_day, 
        messages_per_month, 
        busiest_day, 
        busiest_month, 
        first_message, 
        last_message, 
        total_chats, 
        avg_message_length 
    }
}


pub fn get_stats(users: Vec<User>) -> WhatsResult{
    let most_active = most_active_user(&users);

    let user_results = users.into_iter()
    .map(get_result)
    .collect();

    WhatsResult {
        most_active_user: most_active,
        user_results
    }

}