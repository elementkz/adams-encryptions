use std::string::FromUtf8Error;

#[derive(Clone, Copy)]
struct CeasarCipherStream(u8); // ranges 0-52

// Type (from) conversions
impl From<u8> for CeasarCipherStream {
    fn from(value: u8) -> Self {
        return match value {
            0x20 => CeasarCipherStream(52), // 52 -> whitespace (32 in ascii)
            0x41..=0x5a => CeasarCipherStream(value - 65), // 0-25 -> A-Z (65-90 in ascii)
            0x61..=0x7A => CeasarCipherStream(value - 71), // 26-51 -> a-z (97-122 in ascii)
            _ => panic!("Value outside of range [A-Z,a-z,whitespace]"),
        };
    }
}

impl From<CeasarCipherStream> for u8 {
    fn from(value: CeasarCipherStream) -> Self {
        return match value {
            CeasarCipherStream(52) => 0x20,
            CeasarCipherStream(0..=25) => value.0 + 65,
            CeasarCipherStream(26..=51) => value.0 + 71,
            _ => panic!("Value outside of range [A-Z,a-z,whitespace]"),
        };
    }
}

// CeasarCipherStream Methods
impl CeasarCipherStream {
    fn shift(&mut self, value: u8) {
        if (value + self.0) > 52 {
            self.0 = (self.0 + value) % 53
        } else {
            self.0 += value
        }
    }
    fn unshift(&mut self, value: u8) {
        let mut ushift = self.0 as i16 - value as i16;

        while ushift < 0 {
            ushift += 53;
        }

        self.0 = (ushift % 53) as u8
    }
}

// General functions
fn u8vec_to_streamvec(u8vec: Vec<u8>) -> Vec<CeasarCipherStream> {
    let mut stream: Vec<CeasarCipherStream> = Vec::with_capacity(u8vec.len());
    for i in u8vec {
        stream.push(CeasarCipherStream::from(i))
    }

    stream
}

fn streamvec_to_u8vec(stream_vec: Vec<CeasarCipherStream>) -> Vec<u8> {
    // css_vec = Ceasar Cipher Stream Vector
    let mut stream: Vec<u8> = Vec::with_capacity(stream_vec.len());
    for i in stream_vec {
        stream.push(u8::from(i));
    }

    stream
}

pub fn encode(value: String, shift_val: u8) -> Result<String, FromUtf8Error> {
    if shift_val == 0 {
        return Ok(value);
    }

    let bytes: Vec<u8> = value.as_bytes().to_vec();

    let mut stream = u8vec_to_streamvec(bytes);

    for i in &mut stream {
        i.shift(shift_val)
    }

    return String::from_utf8(streamvec_to_u8vec(stream));
}

pub fn decode(value: String, shift_val: u8) -> Result<String, FromUtf8Error> {
    if shift_val == 0 {
        return Ok(value);
    }
    let bytes: Vec<u8> = value.as_bytes().to_vec();
    let mut stream = u8vec_to_streamvec(bytes);

    for i in &mut stream {
        i.unshift(shift_val)
    }

    return String::from_utf8(streamvec_to_u8vec(stream));
}
