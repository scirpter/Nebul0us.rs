use crate::game::enums;
use crate::models;
use crate::net;

/// ## ?
/// When the bot dies or a round ends,
/// this packet is received.
/// This is more used to determine whether
/// the bot has died.
pub struct SessionStats<'a> {
    bot: &'a mut models::Bot<'a>,
    data: &'a [u8],
}

impl<'a> SessionStats<'a> {
    pub fn new(bot: &'a mut models::Bot<'a>, data: &'a [u8]) -> Self {
        SessionStats { bot, data }
    }

    pub fn parse(&self) {
        todo!("Add bot event handler");
    }
}
