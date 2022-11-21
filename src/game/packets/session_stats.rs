use crate::models::client;

/// ## ?
/// When the bot dies or a round ends,
/// this packet is received.
/// This is more used to determine whether
/// the bot has died.
pub struct SessionStats<'a> {
    bot: &'a mut client::Bot<'a>,
    data: &'a [u8],
}

impl<'a> SessionStats<'a> {
    pub fn new(bot: &'a mut client::Bot<'a>, data: &'a [u8]) -> Self {
        SessionStats { bot, data }
    }

    pub async fn parse(&self) {
        todo!("Add bot event handler");
    }
}
