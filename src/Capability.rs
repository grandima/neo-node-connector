use bincode::Encode;
use bincode::enc::Encoder;
use bincode::error::EncodeError;

pub struct Capability {
    neo_type: u8,
    start_height: u32
}

impl Capability {
    pub fn new() -> Self {
        Self{neo_type: 16, start_height: 16254}
    }
}

impl Encode for Capability {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        Encode::encode(&self.neo_type, encoder)?;
        Encode::encode(&self.start_height, encoder)?;
        Ok(())
    }
}
