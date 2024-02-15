use command::Command;
use events::{EventCallbackFn, EventHandler};
use futures::StreamExt as _;
use irc::{
    client::{data::Config, Client},
    proto::{
        message::Tag as IRCMessageTag, Capability, Command as IRCCommand, Message as IRCMessage,
    },
};

pub mod command;
pub mod events;
pub mod message;

use crate::message::Message;

const TWITCH_IRC_SERVER: &str = "irc.chat.twitch.tv";
const TWITCH_IRC_PORT: u16 = 6697;

pub struct ChatClient<'a> {
    irc: Client,
    events: EventHandler<'a>,
}

pub struct ChatConfig {
    pub nickname: String,
    pub token: String,
    pub channels: Vec<String>,
}

impl<'a> ChatClient<'a> {
    pub async fn new(
        nickname: String,
        token: String,
        channels: Vec<String>,
    ) -> Result<ChatClient<'a>, &'static str> {
        let irc_config = Config {
            nickname: Some(nickname),
            password: Some(format!("oauth:{}", token)),
            server: Some(TWITCH_IRC_SERVER.to_string()),
            port: Some(TWITCH_IRC_PORT),
            channels,
            ..Config::default()
        };

        let irc_client = Client::from_config(irc_config)
            .await
            .expect("Couldn't create IRC server");

        Ok(ChatClient {
            irc: irc_client,
            events: EventHandler::new(),
        })
    }

    pub async fn from_config(config: ChatConfig) -> Result<ChatClient<'a>, &'static str> {
        Self::new(config.nickname, config.token, config.channels).await
    }

    pub async fn connect(&'a mut self) {
        self.irc.identify().expect("Couldn't identify");
        self.request_capabilities();

        let mut stream = self.irc.stream().unwrap();

        while let Some(message) = stream.next().await.transpose().unwrap() {
            // println!("{:#?}", message);
            self.handle_irc_message(message);
        }
    }

    fn handle_irc_message(&'a self, mut message: IRCMessage) -> Option<()> {
        match &message.command {
            IRCCommand::PRIVMSG(_channel, _msg) => {
                let chat_message = Message::from_raw(&self, &mut message)?;

                if let Some(command) = Command::from_message(chat_message.clone()) {
                    if self.events.has_command(&command.name) {
                        self.events.execute_command(&command);
                    } else {
                        self.events.execute_message(chat_message);
                    }
                } else {
                    self.events.execute_message(chat_message);
                }

                Some(())
            }
            _ => None,
        }
    }

    pub fn send_reply(
        &self,
        channel: String,
        reply_id: String,
        message: String,
    ) -> Result<(), &'static str> {
        let message = IRCMessage::with_tags(
            Some(vec![IRCMessageTag(
                "reply-parent-msg-id".to_string(),
                Some(reply_id),
            )]),
            None,
            "PRIVMSG",
            vec![format!("{channel}").as_str(), message.as_str()],
        )
        .expect("Failed to parse message");

        self.irc.send(message).expect("Failed to send message");

        Ok(())
    }

    pub fn send_message(&self, channel: String, message: String) -> Result<(), irc::error::Error> {
        println!("{}", channel);
        self.irc.send_privmsg(format!("{channel}"), message)
    }

    pub fn on_message(&mut self, cb: EventCallbackFn<Message<'a>>) {
        self.events.on_message(cb)
    }

    pub fn add_command(&mut self, command: &str, cb: EventCallbackFn<Command<'a>>) {
        self.events.on_command(command.to_string(), cb)
    }

    fn request_capabilities(&self) {
        let capabilities: [Capability; 3] = [
            Capability::Custom("twitch.tv/commands"),
            Capability::Custom("twitch.tv/membership"),
            Capability::Custom("twitch.tv/tags"),
        ];

        self.irc
            .send_cap_req(&capabilities)
            .expect("Failed to request capabilities");
    }
}
