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

#[cfg(test)]
mod tests {
    use super::*;

    fn d(y: i32, m: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, day).expect("valid date fixture")
    }

    fn build_user(name: &str, chats: &[(&str, NaiveDate)]) -> User {
        let (first_message, first_date) = chats[0];
        let mut user = User::new(name, first_message, first_date);

        for (message, date) in &chats[1..] {
            user.add_chat(message, *date);
        }

        user
    }

    #[test]
    fn total_chats_counts_messages() {
        let user = build_user(
            "Alice",
            &[
                ("hello", d(2024, 1, 1)),
                ("how are you", d(2024, 1, 1)),
                ("bye", d(2024, 1, 2)),
            ],
        );

        assert_eq!(total_chats(&user), 3);
    }

    #[test]
    fn messages_per_day_groups_by_date() {
        let user = build_user(
            "Alice",
            &[
                ("m1", d(2024, 1, 1)),
                ("m2", d(2024, 1, 1)),
                ("m3", d(2024, 1, 2)),
            ],
        );

        let per_day = messages_per_day(&user);
        assert_eq!(per_day.get(&d(2024, 1, 1)), Some(&2));
        assert_eq!(per_day.get(&d(2024, 1, 2)), Some(&1));
    }

    #[test]
    fn avg_message_length_returns_expected_average() {
        let user = build_user(
            "Alice",
            &[("ab", d(2024, 1, 1)), ("abcd", d(2024, 1, 1))],
        );

        let avg = avg_message_length(&user);
        assert!((avg - 3.0).abs() < f64::EPSILON);
    }

    #[test]
    fn busiest_day_picks_highest_count() {
        let mut counts = HashMap::new();
        counts.insert(d(2024, 1, 1), 2);
        counts.insert(d(2024, 1, 2), 5);

        assert_eq!(busiest_day(&counts), Some((d(2024, 1, 2), 5)));
    }

    #[test]
    fn first_message_last_message_uses_chat_order() {
        let user = build_user(
            "Alice",
            &[
                ("first", d(2024, 1, 1)),
                ("middle", d(2024, 1, 4)),
                ("last", d(2024, 1, 9)),
            ],
        );

        assert_eq!(
            first_message_last_message(&user),
            Some((d(2024, 1, 1), d(2024, 1, 9)))
        );
    }

    #[test]
    fn messages_per_month_groups_by_month_number() {
        let user = build_user(
            "Alice",
            &[
                ("jan-1", d(2024, 1, 1)),
                ("jan-2", d(2024, 1, 2)),
                ("feb-1", d(2024, 2, 1)),
            ],
        );

        let per_month = messages_per_month(&user);
        assert_eq!(per_month.get(&1), Some(&2));
        assert_eq!(per_month.get(&2), Some(&1));
    }

    #[test]
    fn busiest_month_picks_highest_count() {
        let mut counts = HashMap::new();
        counts.insert(1, 3);
        counts.insert(2, 8);

        assert_eq!(busiest_month(&counts), Some((2, 8)));
    }

    #[test]
    fn most_active_user_returns_name_with_most_chats() {
        let alice = build_user(
            "Alice",
            &[("a1", d(2024, 1, 1)), ("a2", d(2024, 1, 1))],
        );
        let bob = build_user(
            "Bob",
            &[
                ("b1", d(2024, 1, 1)),
                ("b2", d(2024, 1, 2)),
                ("b3", d(2024, 1, 3)),
            ],
        );

        assert_eq!(most_active_user(&[alice, bob]), Some("Bob".to_string()));
    }

    #[test]
    fn get_result_populates_expected_fields() {
        let user = build_user(
            "Alice",
            &[
                ("one", d(2024, 1, 1)),
                ("two-two", d(2024, 1, 1)),
                ("three", d(2024, 2, 1)),
            ],
        );

        let result = get_result(user);

        assert_eq!(result.user.name, "Alice");
        assert_eq!(result.total_chats, 3);
        assert_eq!(result.busiest_day, Some((d(2024, 1, 1), 2)));
        assert_eq!(result.busiest_month, Some((1, 2)));
        assert_eq!(result.first_message, Some(d(2024, 1, 1)));
        assert_eq!(result.last_message, Some(d(2024, 2, 1)));
        assert!((result.avg_message_length - 5.0).abs() < f64::EPSILON);
    }

    #[test]
    fn get_stats_returns_aggregate_for_all_users() {
        let alice = build_user(
            "Alice",
            &[("a1", d(2024, 1, 1)), ("a2", d(2024, 1, 1))],
        );
        let bob = build_user(
            "Bob",
            &[
                ("b1", d(2024, 1, 1)),
                ("b2", d(2024, 1, 2)),
                ("b3", d(2024, 1, 3)),
            ],
        );

        let stats = get_stats(vec![alice, bob]);

        assert_eq!(stats.most_active_user, Some("Bob".to_string()));
        assert_eq!(stats.user_results.len(), 2);
        assert!(stats.user_results.iter().any(|r| r.user.name == "Alice"));
        assert!(stats.user_results.iter().any(|r| r.user.name == "Bob"));
    }
}