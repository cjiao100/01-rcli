use std::{fs, io::Read, path::Path};

use crate::{get_reader, TextSignFormat};
use anyhow::{Ok, Result};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;

use super::process_gen_pass;

pub trait TextSign {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>>;
}

pub trait TextVerify {
    // 下面两种写法相同，和上面 reader: &dyn Read 对比性能要更好，因为不需要动态分发，但打包后体积要更大些
    // fn verify<R: Read>(&self, reader: R, sig: &str) -> Result<bool>;
    fn verify(&self, reader: impl Read, sig: &[u8]) -> Result<bool>;
}

pub trait KeyLoader {
    fn load(path: impl AsRef<Path>) -> Result<Self>
    where
        Self: Sized; // Sized要求Self的数据结构必须是要有一个固定长度的，str/[u8]不符合要求
}

pub trait KeyGenerator {
    fn generate() -> Result<Vec<Vec<u8>>>; // 返回多个key
}

struct Black3 {
    key: [u8; 32],
}

struct Ed25519Signer {
    key: SigningKey,
}

struct Ed25519Verifier {
    key: VerifyingKey,
}

pub fn process_text_sign(input: &str, key: &str, format: TextSignFormat) -> Result<String> {
    let mut reader = get_reader(input)?;

    let signed = match format {
        TextSignFormat::Blake3 => {
            let signer = Black3::load(key)?;
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
    sig: &str,
) -> Result<bool> {
    let reader = get_reader(input)?;
    let sig = URL_SAFE_NO_PAD.decode(sig)?;
    let verified = match format {
        TextSignFormat::Blake3 => {
            let verifier = Black3::load(key)?;
            verifier.verify(reader, &sig)?
        }
        TextSignFormat::Ed25519 => {
            let verifier = Ed25519Verifier::load(key)?;
            verifier.verify(reader, &sig)?
        }
    };

    Ok(verified)
}

pub fn process_text_generate(format: TextSignFormat) -> Result<Vec<Vec<u8>>> {
    match format {
        TextSignFormat::Blake3 => Black3::generate(),
        TextSignFormat::Ed25519 => Ed25519Signer::generate(),
    }
}

impl TextSign for Black3 {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        Ok(blake3::keyed_hash(&self.key, &buf).as_bytes().to_vec())
    }
}

impl TextVerify for Black3 {
    fn verify(&self, mut reader: impl Read, sig: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let binding = blake3::keyed_hash(&self.key, &buf);
        let hash = binding.as_bytes();
        Ok(hash == sig)
    }
}

impl TextSign for Ed25519Signer {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let signature = self.key.sign(&buf);
        Ok(signature.to_bytes().to_vec())
    }
}

impl TextVerify for Ed25519Verifier {
    fn verify(&self, mut reader: impl Read, sig: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let sig = Signature::from_bytes(sig.try_into()?);
        let ret = self.key.verify(&buf, &sig).is_ok();
        Ok(ret)
    }
}

impl KeyLoader for Black3 {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}
impl KeyLoader for Ed25519Signer {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}
impl KeyLoader for Ed25519Verifier {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

impl KeyGenerator for Black3 {
    fn generate() -> Result<Vec<Vec<u8>>> {
        let key = process_gen_pass(32, true, true, true, true)?;
        let key = key.as_bytes().to_vec();
        Ok(vec![key])
    }
}

impl KeyGenerator for Ed25519Signer {
    fn generate() -> Result<Vec<Vec<u8>>> {
        let mut csprng = OsRng;
        let signing_key: SigningKey = SigningKey::generate(&mut csprng);
        let key = signing_key.to_bytes().to_vec();
        let v_key = signing_key.verifying_key().to_bytes().to_vec();
        Ok(vec![key, v_key])
    }
}

impl Black3 {
    pub fn new(key: [u8; 32]) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = &key[..32];
        let key = key.try_into().unwrap();
        let signer = Black3::new(key);

        Ok(signer)
    }
}

impl Ed25519Signer {
    pub fn new(key: SigningKey) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key: &[u8; 32] = key.try_into()?;
        let key = SigningKey::from_bytes(key);
        let signer = Ed25519Signer::new(key);

        Ok(signer)
    }
}

impl Ed25519Verifier {
    pub fn new(key: VerifyingKey) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key: &[u8; 32] = key.try_into()?;
        let key = VerifyingKey::from_bytes(key)?;

        let verifier = Ed25519Verifier::new(key);
        Ok(verifier)
    }
}

#[cfg(test)]
mod tests {
    use crate::process::text::{KeyLoader, TextVerify};

    use super::{Black3, TextSign};

    #[test]
    fn test_blake3_sign_verify() -> Result<(), anyhow::Error> {
        let black3 = Black3::load("fixtures/blake3.txt")?;

        let data = b"Hello World";
        let sig = black3.sign(&mut &data[..])?;

        assert!(black3.verify(&data[..], &sig).unwrap());
        Ok(())
    }

    #[test]
    fn test_ed25519_sign_verify() -> Result<(), anyhow::Error> {
        let signer = super::Ed25519Signer::load("fixtures/ed25519_signer.txt")?;
        let verifier = super::Ed25519Verifier::load("fixtures/ed25519_verifier.txt")?;

        let data = b"Hello World";
        let sig = signer.sign(&mut &data[..])?;

        assert!(verifier.verify(&data[..], &sig).unwrap());
        Ok(())
    }
}
