use std::io::{Cursor, Read, Seek};

use crate::{models::result::WhatsResult, stats::{compute::get_stats, read::{get_chat_content_from_reader, parse_chat}}};
use crate::models::errors::MyError;

pub mod models;
pub mod stats;

type AppResult<T> = Result<T, MyError>;

pub fn analyze_zip_reader<R: Read + Seek>(reader: R) -> AppResult<WhatsResult> {
    let chat = get_chat_content_from_reader(reader)?;
    if chat.trim().is_empty() {
        return Err(MyError::EmtpyChat)
    }
    let users = parse_chat(&chat).into_values().collect();
    Ok(get_stats(users))
}

pub fn analyze_zip_bytes(bytes: &[u8]) -> AppResult<WhatsResult>{
    let cursor = Cursor::new(bytes);
    analyze_zip_reader(cursor)
}

pub fn analyze_zip_path(path: &str) -> AppResult<WhatsResult> {
    let file = std::fs::File::open(path)?;
    analyze_zip_reader(file)
}