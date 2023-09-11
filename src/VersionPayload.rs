use serde::{Serialize, Serializer};
use bincode::*;
use bincode::de::Decoder;
use bincode::enc::Encoder;
use bincode::error::{DecodeError, EncodeError};

pub struct NEOi64(i64);

impl Encode for NEOi64 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        let value = self.0;
        if value < 0 {
            return Err(EncodeError::Other("Out of range"))
        }
        if value < 0xFD {
            Encode::encode(&(value as u8), encoder)?;
        } else if value <= 0xFFFF {
            Encode::encode(&(0xFD as u8), encoder)?;
            Encode::encode(&(value as u16), encoder)?;
        } else if value <= 0xFFFFFFFF {
            Encode::encode(&(0xFE as u8), encoder)?;
            Encode::encode(&(value as u32), encoder)?;
        } else {
            Encode::encode(&(0xFF as u8), encoder)?;
            Encode::encode(&value, encoder)?;
        }
        Result::Ok(())
    }
}

impl From<i64> for NEOi64 {
    fn from(value: i64) -> Self {
        Self{0: value}
    }
}
pub struct UserAgent(String);
impl Encode for UserAgent {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        let len = (self.0.len() as i64);
        Encode::encode(&NEOi64::from(len), encoder)?;
        self.0.bytes().for_each(|byte|{_ = Encode::encode(&byte, encoder);});
        Ok(())
    }
}
impl From<String> for UserAgent {
    fn from(value: String) -> Self {
        Self{0: value}
    }
}

struct Capability {
    t: u8,
    start_height: u32
}

impl Encode for Capability {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        Encode::encode(&self.t, encoder)?;
        Encode::encode(&self.start_height, encoder)?;
        Ok(())
    }
}

struct VersionPayload {
    network: u32,
    version: u32,
    timestamp: u32,
    nonce: u32,
    user_agent: UserAgent,
    capabilities: Vec<Capability>
}

impl Encode for VersionPayload {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        Encode::encode(&self.network, encoder);
        Encode::encode(&self.version, encoder);
        Encode::encode(&self.timestamp, encoder);
        Encode::encode(&self.nonce, encoder);
        Encode::encode(&self.user_agent, encoder);
        let len = (self.0.len() as i64);
        Encode::encode(&NEOi64::from(len), encoder)?;
        self.capabilities   .iter().for_each(|item|{_ = Encode::encode(item, encoder);});
        Ok(())
    }
}


