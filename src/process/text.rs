use anyhow::Result;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use ed25519_dalek::{Signature, VerifyingKey};
use ed25519_dalek::{Signer, SigningKey};
use rand::rngs::OsRng;
use std::{fs, io::Read, path::Path};

use crate::{get_reader, TextSignFormat};

use super::process_genpass;

pub trait TextSign {
    fn sign(&self, input: &mut dyn Read) -> Result<Vec<u8>>;
}

pub trait TextVerify {
    fn verify(&self, input: impl Read, sign: &[u8]) -> Result<bool>;
}

pub trait KeyLoader {
    fn load(path: impl AsRef<Path>) -> Result<Self>
    where
        Self: Sized;
}

pub trait KeyGenerator {
    fn generate() -> Result<Vec<Vec<u8>>>;
}

pub struct Blake3 {
    key: [u8; 32],
}

pub struct Ed25519Signer {
    key: SigningKey,
}

pub struct Ed25519Verifier {
    key: VerifyingKey,
}

pub fn process_text_sign(input: &str, key: &str, format: TextSignFormat) -> Result<String> {
    let mut reader = get_reader(input)?;
    let signed = match format {
        TextSignFormat::Blake3 => {
            let signer = Blake3::load(key)?;
            signer.sign(&mut reader)?
        }
        TextSignFormat::Ed25519 => {
            let signer = Ed25519Signer::load(key)?;
            signer.sign(&mut reader)?
        }
    };
    let signed = URL_SAFE_NO_PAD.encode(&signed);
    Ok(signed)
}

pub fn process_text_verify(
    input: &str,
    key: &str,
    format: TextSignFormat,
    sign: &str,
) -> Result<bool> {
    let mut reader = get_reader(input)?;

    let signed = URL_SAFE_NO_PAD.decode(sign)?;

    match format {
        TextSignFormat::Blake3 => {
            let verifier = Blake3::load(key)?;
            verifier.verify(&mut reader, &signed)
        }
        TextSignFormat::Ed25519 => {
            let verifier = Ed25519Verifier::load(key)?;
            verifier.verify(&mut reader, &signed)
        }
    }
}

pub fn process_text_generate(format: TextSignFormat) -> Result<Vec<Vec<u8>>> {
    match format {
        TextSignFormat::Blake3 => Blake3::generate(),
        TextSignFormat::Ed25519 => Ed25519Signer::generate(),
    }
}

impl Blake3 {
    pub fn new(key: [u8; 32]) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = &key[..32];
        let key = key.try_into()?;
        let signer = Blake3::new(key);
        Ok(signer)
    }
}

impl TextSign for Blake3 {
    fn sign(&self, input: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        input.read_to_end(&mut buf)?;
        Ok(blake3::keyed_hash(&self.key, &buf).as_bytes().to_vec())
    }
}

impl TextVerify for Blake3 {
    fn verify(&self, mut input: impl Read, sign: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        input.read_to_end(&mut buf)?;
        Ok(blake3::keyed_hash(&self.key, &buf).as_bytes().to_vec() == sign)
    }
}

impl KeyLoader for Blake3 {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}
impl KeyGenerator for Blake3 {
    fn generate() -> Result<Vec<Vec<u8>>> {
        let key = process_genpass(32, false, false, false, false)?;
        let key = key.as_bytes();
        Ok(vec![key.to_vec()])
    }
}

impl Ed25519Signer {
    pub fn new(key: SigningKey) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = SigningKey::from_bytes(key.try_into()?);
        let signer = Ed25519Signer::new(key);
        Ok(signer)
    }
}

impl KeyLoader for Ed25519Signer {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

impl KeyGenerator for Ed25519Signer {
    fn generate() -> Result<Vec<Vec<u8>>> {
        let mut csprng = OsRng;
        let sk = SigningKey::generate(&mut csprng);
        let pk = sk.verifying_key().to_bytes().to_vec();
        let sk = sk.to_bytes().to_vec();
        Ok(vec![sk, pk])
    }
}
impl TextSign for Ed25519Signer {
    fn sign(&self, input: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        input.read_to_end(&mut buf)?;
        let signature = self.key.sign(&buf);
        Ok(signature.to_bytes().to_vec())
    }
}

impl Ed25519Verifier {
    pub fn new(key: VerifyingKey) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = VerifyingKey::from_bytes(key.try_into()?)?;
        let verifier = Ed25519Verifier::new(key);
        Ok(verifier)
    }
}

impl KeyLoader for Ed25519Verifier {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

impl TextVerify for Ed25519Verifier {
    fn verify(&self, mut input: impl Read, sign: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        input.read_to_end(&mut buf)?;
        let signature = Signature::from_bytes(sign.try_into()?);
        Ok(self.key.verify_strict(&buf, &signature).is_ok())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blake3_sign_verify() -> Result<()> {
        let key = process_text_generate(TextSignFormat::Blake3)?;
        let signer = Blake3::try_new(&key[0])?;
        let test_data = b"hello";

        // 签名 mut 允许通过 reader 修改读取位置，但不能修改底层数据
        let mut reader = &test_data[..];
        let sign = signer.sign(&mut reader)?;

        // 验证 - 使用新的 reader
        let mut reader = &test_data[..];
        let verify = signer.verify(&mut reader, &sign)?;

        assert!(verify);
        Ok(())
    }

    #[test]
    fn test_ed25519_sign_verify() -> Result<()> {
        let key = process_text_generate(TextSignFormat::Ed25519)?;
        let signer = Ed25519Signer::try_new(&key[0])?;
        let test_data = b"hello";
        let mut reader = &test_data[..];
        let sign = signer.sign(&mut reader)?;

        let mut reader = &test_data[..];
        let verifier = Ed25519Verifier::try_new(&key[1])?;
        let verify = verifier.verify(&mut reader, &sign)?;

        assert!(verify);
        Ok(())
    }
}
