use bincode::enc::Encoder;
use crate::VersionPayload::VersionPayload;
use bincode::Encode;
use bincode::error::EncodeError;
use crate::neoi64::NEOi64;

#[derive(Debug)]
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
        let mut payload_vec: Vec<u8> = Vec::new();
        _ = bincode::encode_into_std_write(&self.payload, &mut payload_vec, *encoder.config()).unwrap();
        Encode::encode(&NEOi64::from(payload_vec.len() as i64), encoder)?;
        payload_vec.iter().for_each(|item|{_ = Encode::encode(item, encoder);});

        Ok(())
    }
}
