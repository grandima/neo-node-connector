use bincode::Encode;
use bincode::enc::Encoder;
use bincode::error::EncodeError;

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
            Encode::encode(&(0xFDu8), encoder)?;
            Encode::encode(&(value as u16), encoder)?;
        } else if value <= 0xFFFFFFFF {
            Encode::encode(&(0xFEu8), encoder)?;
            Encode::encode(&(value as u32), encoder)?;
        } else {
            Encode::encode(&(0xFFu8), encoder)?;
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
