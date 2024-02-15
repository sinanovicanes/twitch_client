use irc::proto::{Command, Message as IRCMessage};

use crate::ChatClient;

#[derive(Clone)]
pub struct Message<'a> {
    pub client: &'a ChatClient<'a>,
    pub message_id: String,
    pub channel: String,
    pub username: String,
    pub text: String,
}

impl<'a> Message<'a> {
    pub fn new(
        client: &'a ChatClient<'a>,
        message_id: String,
        channel: String,
        username: String,
        text: String,
    ) -> Self {
        Self {
            client,
            message_id,
            channel,
            username,
            text,
        }
    }

    pub fn from_raw(client: &'a ChatClient<'a>, message: &mut IRCMessage) -> Option<Self> {
        let username = message.source_nickname()?.to_string();
        let tags = message.tags.clone()?;
        let tag = tags.iter().find(|tag| tag.0 == "id")?;
        let message_id = tag.1.clone()?;
        let (channel, text) = match &message.command {
            Command::PRIVMSG(channel, text) => (Some(channel), Some(text)),
            _ => (None, None),
        };

        let channel = channel?;
        let text = text?;

        Some(Self::new(
            client,
            message_id,
            channel.clone(),
            username,
            text.clone(),
        ))
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

impl std::fmt::Debug for Message<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} | {}: {}",
            self.message_id, self.channel, self.username, self.text
        )
    }
}

impl std::fmt::Display for Message<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}: {}", self.channel, self.username, self.text)
    }
}
