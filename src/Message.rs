use bincode::enc::Encoder;
use crate::VersionPayload::VersionPayload;
use bincode::Encode;
use bincode::error::EncodeError;

pub struct Message {
    flags: u8,
    command: u8,
    payload: VersionPayload
}

impl Message {
    pub fn new() -> Self {
        Self {flags: 0, command: 0, payload: VersionPayload::new()}
    }
}

impl Encode for Message {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        Encode::encode(&self.flags, encoder)?;
        Encode::encode(&self.command, encoder)?;
        Encode::encode(&self.payload, encoder)?;
        Ok(())
    }
}
