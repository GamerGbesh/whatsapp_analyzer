use whatsapp_analyzer::analyze_zip_path;

fn main() {
    let filename = "WhatsApp Chat with Irene 😘.zip";
    if let Ok(result) = analyze_zip_path(filename) {
        let json_response = serde_json::to_string(&result).unwrap();
        println!("The result's are \n {}", json_response)
    };

}