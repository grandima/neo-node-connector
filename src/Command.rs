use std::backtrace::BacktraceStatus::Captured;
use std::ops::Deref;
use bincode::{Decode, Encode};
use bincode::de::Decoder;
use bincode::error::{DecodeError, EncodeError};
use bincode::enc::Encoder;

#[repr(u8)]
#[derive(Debug, Clone)]
pub enum Command {
    Version,
    Verack
}

impl TryFrom<u8> for Command {
    type Error = String;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Command::Version),
            1 => Ok(Command::Verack),
            _ => Err(format!("Range exceeded: {:?}", value))
        }
    }
}

impl Encode for Command {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        let value = self.clone() as u8;
        Encode::encode(&value, encoder)
    }
}

impl Decode for Command {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        let value: u8 = Decode::decode(decoder)?;
        Command::try_from(value).map_err(|e| DecodeError::OtherString(e))
    }
}
