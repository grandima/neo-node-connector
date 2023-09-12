use bincode::*;
use bincode::enc::Encoder;
use bincode::error::EncodeError;
use crate::Capability::Capability;
use crate::neoi64::NEOi64;
use crate::user_agent::UserAgent;

pub struct VersionPayload {
    network: u32,
    version: u32,
    timestamp: u32,
    nonce: u32,
    user_agent: UserAgent,
    capabilities: Vec<Capability>
}

impl VersionPayload {
    pub fn new() -> Self {
        Self{network: 860833102, version: 0, timestamp: 1694537347, nonce: 1073020062, user_agent: "/Neo:3.6.0/".into(), capabilities: vec![Capability::new()]}
    }
}

impl Encode for VersionPayload {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        Encode::encode(&self.network, encoder)?;
        Encode::encode(&self.version, encoder)?;
        Encode::encode(&self.timestamp, encoder)?;
        Encode::encode(&self.nonce, encoder)?;
        Encode::encode(&self.user_agent, encoder)?;
        let len = (self.capabilities.len() as i64);
        Encode::encode(&NEOi64::from(len), encoder)?;
        self.capabilities.iter().for_each(|item|{_ = Encode::encode(item, encoder);});
        Ok(())
    }
}