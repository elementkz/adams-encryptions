use std::string::FromUtf8Error;


/*
0-25 -> a-z
26-51 -> A-Z
52 -> whitespace
 */

#[derive(Debug)]
struct CeasarChipher;
impl CeasarChipher {
    fn return_value_in_range(ch: char) {
        todo!();
    }
    fn encode(value: String, shift: u8) -> Result<String, FromUtf8Error> {
        if shift == 0 {
            return Ok(value);
        }

        let mut bytes: Vec<u8> = value.as_bytes().to_vec();

        for i in &mut bytes {
            *i += shift;
        }

        return String::from_utf8(bytes);
    }

    fn decode(value: String, shift: u8) -> Result<String, FromUtf8Error> {
        if shift == 0 {
            return Ok(value);
        }

        let mut bytes: Vec<u8> = value.as_bytes().to_vec();

        for i in &mut bytes {
            *i -= shift;
        }

        return String::from_utf8(bytes);
    }
}

fn main() {
    let x = String::from("Hello There");

    let x = CeasarChipher::encode(x, 8).unwrap();
    println!("{}", x);
    let x = CeasarChipher::decode(x, 8).unwrap();
    println!("{}", x);
}
