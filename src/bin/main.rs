use whatsapp_analyzer::stats::compute::get_stats;
use whatsapp_analyzer::stats::read::{get_chat_content_from_zip, parse_chat};

fn main() {
    let chat = get_chat_content_from_zip("WhatsApp Chat with Irene 😘.zip").unwrap();
    let users = parse_chat(&chat);
    let user_vec= users.into_values().collect();

    let result = get_stats(user_vec);
    println!("The result's are \n {}", result)

}