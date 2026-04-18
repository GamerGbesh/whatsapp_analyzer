use chrono::NaiveDate;

#[derive(Debug)]
pub struct Chat {
    pub message: String,
    pub date: NaiveDate
}

#[derive(Debug)]
pub struct User {
    pub name: String,
    pub chats: Vec<Chat>
}

impl User {

    /// Create a new user with `name` and empty chats
    pub fn new(name: &str, chat: &str, date: NaiveDate) -> User {

        let first_chat = Chat {
            message: chat.to_string(),
            date
        };

        User {
            name: name.to_string(),
            chats: vec![first_chat]
        }
    }

    /// Add a new chat to the users vector
        pub fn add_chat(&mut self, chat: &str, date: NaiveDate) {

        if !chat.is_empty() {

            let new_chat = Chat {
                message: chat.to_string(),
                date
            };

            self.chats.push(new_chat);
        }
    }
}

