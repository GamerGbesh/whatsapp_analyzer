use std::{collections::HashMap, str::FromStr};
use std::fmt::{Display, Formatter, Result};


use chrono::NaiveDate;

use crate::models::user::User;

#[derive(Debug)]
pub struct UserStat {
    pub user: User,
    pub messages_per_day: HashMap<NaiveDate, usize>,
    pub messages_per_month: HashMap<u32, usize>,
    pub busiest_day: Option<(NaiveDate, usize)>,
    pub busiest_month: Option<(u32, usize)>,
    pub first_message: Option<NaiveDate>,
    pub last_message: Option<NaiveDate>,
    pub total_chats: usize,
    pub avg_message_length: f64
}

#[derive(Debug)]
pub struct WhatsResult {
    pub most_active_user: Option<String>,
    pub user_results: Vec<UserStat>
}



impl UserStat {
    pub fn new(
        user: User, 
        messages_per_day: HashMap<NaiveDate, usize>,
        messages_per_month: HashMap<u32, usize>,
        busiest_day: Option<(NaiveDate, usize)>,
        busiest_month: Option<(u32, usize)>,
        first_message: Option<NaiveDate>,
        last_message: Option<NaiveDate>,
        total_chats: usize,
        avg_message_length: f64
    ) -> UserStat
    {
        UserStat {
            user, 
            messages_per_day,
            messages_per_month,
            busiest_day,
            busiest_month,
            first_message,
            last_message,
            total_chats,
            avg_message_length,
        }
    }
}


impl WhatsResult {
    pub fn new() -> WhatsResult
    {
        WhatsResult {
            most_active_user: Some(String::new()),
            user_results: Vec::new()
        }
    }

    pub fn set_most_active(&mut self, name: &str){
        let name = String::from_str(name).unwrap();
        self.most_active_user = Some(name);
    }

    pub fn add_user_result(&mut self, user_stat: UserStat){
        self.user_results.push(user_stat);
    }
}


impl Display for WhatsResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let most_active = self.most_active_user.as_deref().unwrap_or("None");

        writeln!(f, "Most Active User: {}", most_active)?;

        for result in &self.user_results {
            writeln!(f, "Name: {}", result.user.name)?;
            writeln!(f, "total_chats: {}", result.total_chats)?;
            writeln!(f, "avg_message_length: {}", result.avg_message_length)?;
            writeln!(f)?;
        }

        Ok(())
    }
}