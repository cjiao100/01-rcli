use anyhow::Result;
use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    Engine as _,
};

use crate::{read_data, Base64Format};

pub fn process_encode(input: &str, format: Base64Format) -> Result<String> {
    let buffer = read_data(input)?;

    let encoded = match format {
        Base64Format::Standard => STANDARD.encode(&buffer),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(&buffer),
    };
    Ok(encoded)
}

pub fn process_decode(input: &str, format: Base64Format) -> Result<Vec<u8>> {
    let buffer = read_data(input)?;

    let decoded = match format {
        Base64Format::Standard => STANDARD.decode(buffer)?,
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(buffer)?,
    };

    Ok(decoded)
}

#[cfg(test)]
mod tests {
    use crate::{process_decode, process_encode, Base64Format};

    #[test]
    fn test_process_encode() {
        let input = "Cargo.toml";
        let format = Base64Format::Standard;
        assert!(process_encode(input, format).is_ok());
    }

    #[test]
    fn test_process_decode() {
        let input = "fixtures/b64_encode.txt";
        let format = Base64Format::UrlSafe;
        assert!(process_decode(input, format).is_ok())
    }
}
