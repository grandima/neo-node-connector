use bincode::Encode;
use bincode::enc::Encoder;
use bincode::error::EncodeError;
use crate::neoi64::NEOi64;
#[derive(Debug)]
pub struct UserAgent(String);

impl Encode for UserAgent {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        let len = self.0.len() as i64;
        Encode::encode(&NEOi64::from(len), encoder)?;
        self.0.bytes().for_each(|byte|{_ = Encode::encode(&byte, encoder);});
        Ok(())
    }
}

impl From<&str> for UserAgent {
    fn from(value: &str) -> Self {
        Self::from(value.to_string())
    }
}

impl From<String> for UserAgent {
    fn from(value: String) -> Self {
        Self(value)
    }
}
