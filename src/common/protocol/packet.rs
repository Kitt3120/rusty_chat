pub mod client;
pub mod server;

use crate::common::protocol::{message::Message, serializable::Serializable};

pub trait Packet: Serializable {
    fn to_message(self) -> Message;
}
