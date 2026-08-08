use std::error::Error;

fn to_hex(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn decode(input: Vec<u8>) -> Result<u32, Box<dyn Error>> {
    let decoded = hdlc::decode(&input, hdlc::SpecialChars::default());
    if decoded.is_err() {
        // TEMP: dump raw payload for frames the HDLC layer rejects, to spot message types worth investigating
        // eprintln!("HDLC decode failed, raw payload: {}", to_hex(&input));
        // return Err(format!("HDLC decoding failed: {:?}", decoded.err().unwrap()).into());
    }
    let decoded = decoded.unwrap();
    let control_byte = decoded[5];
    let packet_type = decoded[18];
    // TEMP: log every decoded frame's metadata + full bytes to survey message types in the wild
    // eprintln!(
    //     "control_byte=0x{control_byte:02x} packet_type=0x{packet_type:02x} decoded={}",
    //     to_hex(&decoded)
    // );
    if control_byte == 0x13 {
        match packet_type {
            0x1 => Ok(u32::from_be_bytes(decoded[30..34].try_into()?)),
            0x2 | 0x9 | 0xC | 0xD | 0x0E | 0x11 | 0x12 => {
                Ok(u32::from_be_bytes(decoded[97..101].try_into()?))
            }
            _ => Err("No valid packed type found")?,
        }
    } else {
        return Err("Control byte 0x13 missing")?;
    }
}
