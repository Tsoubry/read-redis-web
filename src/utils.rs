use std::{error, io::Error, str::from_utf8};

use serde_json::Value;

fn gzip_decode(bytes: impl AsRef<[u8]>) -> Result<Vec<u8>, Error> {
    use std::io::Read as _;
    let mut decoder = flate2::read::GzDecoder::new(bytes.as_ref());
    let mut buf = Vec::new();
    decoder.read_to_end(&mut buf)?;
    Ok(buf)
}

fn validate_json(bytes: Vec<u8>) -> Result<Vec<u8>, Box<dyn error::Error>> {
    let json_string = from_utf8(&bytes)?;
    let _: Value = serde_json::from_str(json_string)?;

    Ok(bytes)
}

pub fn decide_decompress(decompress: bool, value: Vec<u8>) -> Result<Vec<u8>, Error> {
    if decompress {
        gzip_decode(value)
    } else {
        Ok(value)
    }
}

pub fn decide_validate(validate: bool, value: Vec<u8>) -> Result<Vec<u8>, Box<dyn error::Error>> {
    if validate {
        validate_json(value)
    } else {
        Ok(value)
    }
}
