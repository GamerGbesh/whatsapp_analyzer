use chrono::NaiveDate;
use std::collections::HashMap;
use regex::Regex;
use std::io::{Read, Seek};
use zip::ZipArchive;

use crate::{AppResult, models::{errors::MyError, user::User}};


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
pub fn get_chat_content_from_reader<R: Read + Seek>(reader: R) -> AppResult<String>{
    let mut archive = ZipArchive::new(reader)?;
    for i in 0..archive.len(){
        let mut file = archive.by_index(i)?;

        if file.name().ends_with(".txt") {
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            return Ok(contents)
        }
    };

    // If no .txt file found
    Err(MyError::NotFound)    
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
        let line = line.trim();
        if let Some((name, content, date)) = get_chat_name_and_content(line) {
            names.entry(name)
                .and_modify(|user| user.add_chat(content, date))
                .or_insert_with(|| User::new(name, content, date));
        }
        
}

    names
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Cursor, Write};
    use zip::write::FileOptions;
    use zip::ZipWriter;

    fn create_test_zip() -> Cursor<Vec<u8>> {
        let mut buffer = Cursor::new(Vec::new());

        {
            let mut zip = ZipWriter::new(&mut buffer);
            let options: FileOptions<()> = FileOptions::default()
                .compression_method(zip::CompressionMethod::Stored);

            zip.start_file("chat.txt", options).unwrap();
            zip.write_all(b"Hello from chat!").unwrap();

            zip.start_file("other.bin", options).unwrap();
            zip.write_all(b"binary stuff").unwrap();

            zip.finish().unwrap();
        }

        buffer.set_position(0); // 🔥 IMPORTANT
        buffer
    }


    #[test]
    pub fn test_get_chat_content(){
        let archive = create_test_zip();
        let result = get_chat_content_from_reader(archive)
            .expect("Resource not found");

        assert_eq!(result, "Hello from chat!")
    }


    #[test]
    pub fn test_get_name(){
        let chat1 = "01/01/2025, 09:05 - Alice: Hey there";
        let chat2 = "12/12/2024, 23:59 - John Doe: This is a test message";
        let chat3 = "one:two";

        assert_eq!(
            get_chat_name_and_content(chat1), 
            Some((
                "Alice", 
                "Hey there", 
                NaiveDate::parse_from_str("01/01/2025", "%d/%m/%Y").unwrap()
            ))
        );

        assert_eq!(
            get_chat_name_and_content(chat2),
            Some((
                "John Doe", 
                "This is a test message", 
                NaiveDate::parse_from_str("12/12/2024", "%d/%m/%Y").unwrap()
            ))
        );

        assert_eq!(get_chat_name_and_content(chat3), None);
    }


    #[test]
    pub fn test_parse_chat(){
        let data = r#"
        01/01/2025, 09:05 - Alice: Hey there
        12/12/2024, 23:59 - John Doe: Hello
        "#;
        
        let result = parse_chat(data);
        let expected : HashMap<&str, User> = [
            ("Alice", User::new(
                "Alice", "Hey there", NaiveDate::parse_from_str("01/01/2025", "%d/%m/%Y").unwrap())
            ),
            ("John Doe", User::new(
                "John Doe", "Hello", NaiveDate::parse_from_str("12/12/2024", "%d/%m/%Y").unwrap())
            )
            ]
            .into_iter()
            .collect();

        assert_eq!(result, expected);
    }
}