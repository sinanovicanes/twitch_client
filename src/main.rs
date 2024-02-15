use dotenv::dotenv;
use twitch_client::{ChatClient, ChatConfig};

#[tokio::main]
async fn main() {
    dotenv().expect("Failed to parse env variables");

    let token = std::env::var("TWITCH_TOKEN").expect("TWITCH TOKEN not found");

    let cfg = ChatConfig {
        nickname: "sinanovicanes".to_string(),
        token,
        channels: vec!["#sinanovicanes".to_string()],
    };

    let mut chat_client = ChatClient::from_config(cfg)
        .await
        .expect("Failed to connect");

    chat_client.on_message(|m| {
        println!("{}", m);
    });

    chat_client.add_command("test", |m| {
        let _ = m.reply("good".to_string());
    });

    chat_client.connect().await;

    println!("Hello, world!");
}
