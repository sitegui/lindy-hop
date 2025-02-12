use aes_gcm::aead::{Aead, Nonce, OsRng};
use aes_gcm::{AeadCore, Aes256Gcm, Key, KeyInit};
use anyhow::{anyhow, Context};
use pbkdf2::pbkdf2_hmac;
use sha2::Sha256;

#[derive(Debug)]
pub struct Encrypted {
    pub iv: String,
    pub ciphertext: String,
}

pub fn encrypt(
    password: &str,
    salt: &str,
    iterations: u32,
    plaintext: &str,
) -> anyhow::Result<Encrypted> {
    let key = derive_key(password, salt, iterations);
    let cipher = Aes256Gcm::new(&key);
    let iv = Aes256Gcm::generate_nonce(&mut OsRng);
    let ciphertext = cipher.encrypt(&iv, plaintext.as_bytes())?;

    Ok(Encrypted {
        iv: base16ct::lower::encode_string(&iv),
        ciphertext: base16ct::lower::encode_string(&ciphertext),
    })
}

#[allow(unused)]
pub fn decrypt(
    password: &str,
    salt: &str,
    iterations: u32,
    encrypted: &Encrypted,
) -> anyhow::Result<String> {
    let key = derive_key(password, salt, iterations);
    let cipher = Aes256Gcm::new(&key);
    let iv = base16ct::lower::decode_vec(&encrypted.iv)?;
    let iv: [u8; 12] = iv.try_into().map_err(|_| anyhow!("invalid IV"))?;
    let iv: Nonce<Aes256Gcm> = iv.into();
    let plaintext = cipher.decrypt(
        &iv,
        base16ct::lower::decode_vec(&encrypted.ciphertext)?.as_slice(),
    )?;

    String::from_utf8(plaintext).context("invalid utf-8")
}

fn derive_key(password: &str, salt: &str, iterations: u32) -> Key<Aes256Gcm> {
    let mut key = Key::<Aes256Gcm>::default();
    pbkdf2_hmac::<Sha256>(password.as_bytes(), salt.as_bytes(), iterations, &mut key);
    key
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let encrypted = encrypt("1234", "salt", 100_000, "something").unwrap();
        let decrypted = decrypt("1234", "salt", 100_000, &encrypted).unwrap();

        assert_eq!(decrypted, "something");
    }
}
