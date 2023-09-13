use bincode::*;
use bincode::enc::Encoder;
use bincode::error::EncodeError;
use crate::Capability::{Capability, CapabilityType, FULL_NODE, FullCapability, TCP_SERVER, WS_SERVER};
use crate::neoi64::NEOi64;
use crate::user_agent::UserAgent;
use std::time::*;
use tokio::time;
#[derive(Debug)]
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
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default();
        println!("stamp {:?}",now.as_secs());
        Self{network: 5943216, version: 0, timestamp: 1694557199, nonce: 2100635172, user_agent: "/Neo:3.6.0/".into(), capabilities: vec![
            FULL_NODE,
            TCP_SERVER,
            WS_SERVER
        ]}
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