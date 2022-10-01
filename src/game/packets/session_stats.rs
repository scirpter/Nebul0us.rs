use crate::game::packets;
use std::ops::Deref;

pub struct SessionStats {}

impl Deref for SessionStats {
    type Target = packets::Packet;

    fn deref(&self) -> &Self::Target {
        todo!()
    }
}
