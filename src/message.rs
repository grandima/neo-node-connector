
use crate::command::Command;
use crate::neoi64::NEOi64;
use crate::version_payload::VersionPayload;
use bincode::de::Decoder;
use bincode::enc::Encoder;
use bincode::error::{DecodeError, EncodeError};
use bincode::{Decode, Encode};

const PAYLOAD_MAX_SIZE: usize = 0x02000000;
#[derive(Debug)]
pub struct Message {
    flags: u8,
    pub(crate) command: Command,
    pub(crate) payload: VersionPayload,
}

impl Message {
    pub fn new(command: Command) -> Self {
        println!("Sending Message: {:?}", command);
        Self {
            flags: 0,
            command,
            payload: VersionPayload::default(),
        }
    }
    pub fn try_deserialize(data: &[u8]) -> (Option<Message>, i32) {
        if data.len() < 3 {
            return (None, 0);
        }
        let header = &data[0..3];
        let flags = header[0];
        let mut length = header[2] as usize;
        let mut payload_index = 3;

        if length == 0xFD {
            if data.len() < 5 {
                return (None, 0);
            }
            length = u16::from_le_bytes(data[payload_index..payload_index + 2].try_into().unwrap())
                as usize;
            payload_index += 2;
        } else if length == 0xFE {
            if data.len() < 7 {
                return (None, 0);
            }
            length = u32::from_le_bytes(data[payload_index..payload_index + 4].try_into().unwrap())
                as usize;
            payload_index += 4;
        } else if length == 0xFF {
            if data.len() < 11 {
                return (None, 0);
            }
            length = u64::from_le_bytes(data[payload_index..payload_index + 8].try_into().unwrap())
                as usize;
            payload_index += 8;
        }

        if length > PAYLOAD_MAX_SIZE {
            return (None, -1);
        }
        length += payload_index;
        if data.len() < length {
            return (None, 0);
        }
        match Command::try_from(header[1]) {
            Ok(command) => {
                println!("Received message: {:?}", command);
                (
                    Some(Message {
                        flags,
                        command,
                        payload: VersionPayload::default(),
                    }),
                    length as i32,
                )
            }
            _ => (None, 0),
        }
    }
    pub fn command(&self) -> &Command {
        &self.command
    }
}

impl Default for Message {
    fn default() -> Self {
        Self {
            flags: 0,
            command: Command::Version,
            payload: VersionPayload::default(),
        }
    }
}

impl Decode for Message {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        let flags: u8 = Decode::decode(decoder)?;
        let command: Command = Decode::decode(decoder)?;
        Ok(Self {
            flags,
            command,
            payload: VersionPayload::default(),
        })
    }
}

impl Encode for Message {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        Encode::encode(&self.flags, encoder)?;
        Encode::encode(&self.command, encoder)?;
        let mut payload_vec: Vec<u8> = Vec::new();
        _ = bincode::encode_into_std_write(&self.payload, &mut payload_vec, *encoder.config())
            .unwrap();
        Encode::encode(&NEOi64::from(payload_vec.len() as i64), encoder)?;
        payload_vec.iter().for_each(|item| {
            _ = Encode::encode(item, encoder);
        });
        Ok(())
    }
}
