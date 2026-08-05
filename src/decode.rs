use std::error::Error;

pub fn decode(input: Vec<u8>) -> Result<u32, Box<dyn Error>> {
    let decoded = hdlc::decode(&input, hdlc::SpecialChars::default());
    if decoded.is_err() {
        return Err(format!("HDLC decoding failed: {:?}", decoded.err().unwrap()).into());
    }
    let decoded = decoded.unwrap();
    let control_byte = decoded[5];
    let packet_type = decoded[18];
    if control_byte == 0x13 {
        match packet_type {
            0x1 => Ok(u32::from_be_bytes(decoded[30..34].try_into()?)),
            0x9 | 0xC | 0xD | 0x0E | 0x11 | 0x12 => {
                Ok(u32::from_be_bytes(decoded[97..101].try_into()?))
            }
            _ => Err("No valid packed type found")?,
        }
    } else {
        return Err("Control byte 0x13 missing")?;
    }
}
