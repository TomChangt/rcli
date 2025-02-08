use crate::cli::Base64Format;
use std::io::Read;

use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    Engine as _,
};

pub fn process_encode(reader: &mut Box<dyn Read>, format: Base64Format) -> anyhow::Result<String> {
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    let encoded = match format {
        Base64Format::Standard => STANDARD.encode(&buf),

        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(&buf),
    };

    Ok(encoded)
}

pub fn process_decode(reader: &mut Box<dyn Read>, format: Base64Format) -> anyhow::Result<Vec<u8>> {
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    // 移除读到的空白符（包括换行符）
    let buf = buf.trim();

    let decoded = match format {
        Base64Format::Standard => STANDARD.decode(buf)?,
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(buf)?,
    };

    Ok(decoded)
}
