use crate::{message::Message, ChatClient};

pub struct Command<'a> {
    pub client: &'a ChatClient<'a>,
    pub channel: String,
    pub username: String,
    pub name: String,
    pub args: Vec<String>,
    message_id: String,
}

fn get_command_name_from_text(text: &String) -> Option<String> {
    let parts: Vec<&str> = text.split_whitespace().collect();

    if let Some(first_part) = parts.get(0) {
        if first_part.starts_with('!') {
            return Some(first_part[1..].to_string());
        }
    }

    None
}

fn get_command_args(text: &String) -> Vec<String> {
    let mut parts: Vec<String> = text.split_whitespace().map(|arg| arg.to_string()).collect();

    parts.remove(0);

    parts
}

impl<'a> Command<'a> {
    pub fn from_message(message: Message<'a>) -> Option<Self> {
        let name = get_command_name_from_text(&message.text)?;

        Some(Self {
            client: message.client,
            channel: message.channel,
            username: message.username,
            name,
            args: get_command_args(&message.text),
            message_id: message.message_id,
        })
    }

    pub fn as_text(&self) -> String {
        let mut text = format!("!{}", self.name);

        for arg in &self.args {
            text.push_str(format!(" {}", arg).as_str())
        }

        text
    }

    pub fn to_message(self) -> Message<'a> {
        let text = self.as_text();

        Message {
            client: self.client,
            channel: self.channel,
            username: self.username,
            message_id: self.message_id,
            text,
        }
    }

    pub fn reply(&self, message: String) -> Result<(), &'static str> {
        self.client
            .send_reply(
                self.channel.to_string(),
                self.message_id.to_string(),
                message,
            )
            .expect("Failed to send reply");

        Ok(())
    }
}
