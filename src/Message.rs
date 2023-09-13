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
        Encode::encode(&self.payload, encoder)?;
        let mut payload_vec: Vec<u8> = Vec::new();
        let data = VersionPayload::new();
        let v = bincode::encode_to_vec(data, bincode::config::standard().with_fixed_int_encoding()).unwrap();
        Encode::encode(&NEOi64::from(v.len() as i64), encoder)?;
        v.iter().for_each(|item|{_ = Encode::encode(item, encoder);});
        Ok(())
    }
}
