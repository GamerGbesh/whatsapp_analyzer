use chrono::NaiveDate;
use std::collections::HashMap;
use regex::Regex;
use std::fs::File;
use std::io::{Read, Result};
use zip::ZipArchive;

use crate::models::user::User;


/// Takes a filepath to the zip file of the whatsapp chat to extract the chat content
///
/// The function opens the zip file and reads through the files and looks for the .txt file which contains the chat contents
/// It reads the content into a variable and returns the chat content
///
/// # Arguments
///
/// * `filename` - The file path of the zip file
///
/// # Returns
///
/// Returns `Ok(String)` containing the chat content
///
/// # Errors
/// Returns an error if:
/// - The ZIP archive cannot be read
/// - No `.txt` file is found
/// - Reading the file contents fails
pub fn get_chat_content_from_zip(filename: &str) -> Result<String>{
    
    let file = File::open(filename)?;
    let mut archive = ZipArchive::new(file)?;

    for i in 0..archive.len(){
        let mut file = archive.by_index(i)?;

        if file.name().ends_with(".txt") {
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            return Ok(contents)
        }
    };

    // If no .txt file found
    Err(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "No .txt file found in zip",
    ))    
}


pub fn get_chat_name_and_content<'a>(line: &'a str) -> Option<(&'a str, &'a str, NaiveDate)> {
    let re = Regex::new(r"(?m)^(?<date>\d{2}/\d{2}/\d{4}), \d{2}:\d{2} - (?P<name>[^:]+): (?P<content>.*)").unwrap();
    
    if let Some(caps) = re.captures(line) {
        let name = caps.name("name")?.as_str();
        let content = caps.name("content")?.as_str();
        let date = caps.name("date")
                       .and_then(|m| NaiveDate::parse_from_str(m.as_str(), "%d/%m/%Y").ok())
                       .unwrap_or_else(|| NaiveDate::default());

        Some((name, content, date))
    } else {
        None
    }
}

/// Iterate through the chat and create users alongside the chats that they entered into the chat
/// 
/// # Arguments
/// `chat` The full whatsapp chat
/// 
/// # Returns
/// `names` Hashmap of `User` structs
pub fn parse_chat<'a>(chat: &'a str) -> HashMap<&'a str, User>{
    let mut names: HashMap<&str, User> = HashMap::new();
    for line in chat.lines() {
        if let Some((name, content, date)) = get_chat_name_and_content(line) {
            names.entry(name)
                .and_modify(|user| user.add_chat(content, date))
                .or_insert_with(|| User::new(name, content, date));
        }
        
}

    names
}


// pub fn write_users_to_file(users: &Vec<&User>, filename: &str) -> Result<()> {
//     let mut file = File::create(filename)?; // Create or overwrite

//     for user in users {
//         writeln!(file, "User: {}", user.name)?; // Write the username

//         for (i, chat) in user.chats.iter().enumerate() {
//             let date = user.chats.dates.get(i).unwrap();
//             writeln!(file, "[{}] {}", date, chat)?;
//         }

//         writeln!(file, "\n--------------------\n")?; // Separator between users
//     }

//     Ok(())
// }