use bincode::Encode;
use bincode::enc::Encoder;
use bincode::error::EncodeError;
#[derive(Debug)]
pub struct Capability {
    type_num: u8,
    type_val: CapabilityType
}
pub const FULL_NODE: Capability = Capability{type_num: 16, type_val: CapabilityType::Full(27691)};
pub const TCP_SERVER: Capability = Capability{type_num: 1, type_val: CapabilityType::Server(21333)};
pub const WS_SERVER: Capability = Capability{type_num: 2, type_val: CapabilityType::Server(21334)};


impl Encode for Capability {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        Encode::encode(&self.type_num, encoder)?;
        Encode::encode(&self.type_val, encoder)?;
        Ok(())
    }
}
#[derive(Debug)]
pub enum CapabilityType {
    Server(u16),
    Full(u32)
}

impl Encode for CapabilityType {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        match self {
            CapabilityType::Server(port) => Encode::encode(port, encoder)?,
            CapabilityType::Full(height) => Encode::encode(height, encoder)?
        }
        Ok(())
    }
}
