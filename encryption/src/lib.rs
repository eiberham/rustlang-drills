extern crate base64;
extern crate console_error_panic_hook;
extern crate web_sys;

use wasm_bindgen::prelude::*;
use rsa::{PublicKey, RSAPrivateKey, RSAPublicKey, PaddingScheme};
use rand::{rngs::OsRng};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, NewAead};
use std::str;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RSAKeyPair {
  private_key: RSAPrivateKey,
  public_key: RSAPublicKey
}


pub fn generate_rsa_key (bits: i32) -> RSAKeyPair {
  let mut rng = OsRng;
  let private_key = RSAPrivateKey::new(&mut rng, bits as usize)
    .expect("failed to generate a key");
  let public_key = RSAPublicKey::from(&private_key);
  RSAKeyPair {private_key: private_key, public_key: public_key}
}

#[wasm_bindgen]
pub fn asymmetric_encrypt(pk: &str, plaintext: String) -> Vec<u8> {
    console_error_panic_hook::set_once();
    /* web_sys::console::log_1(&pk.into());
    web_sys::console::log_1(JsValue::from_str(&plaintext).as_ref()); */

    let mut rng = OsRng;
    // the public key string comes as a base64 encoded string so we should decode 
    // it to a Vec<u8> as it's a sequence of 8-bit bytes.
    let bytes = base64::decode(&pk).unwrap();
    let public_key: RSAPublicKey = RSAPublicKey::from_pkcs1(&bytes)
        .expect("failed to parse key");

    // let public_key: RSAPublicKey = serde_json::from_str(&pk).unwrap();
    let padding = PaddingScheme::new_oaep::<sha2::Sha256>();
    return public_key.encrypt(&mut rng, padding, plaintext.as_bytes())
        .expect("failed to encrypt");
}

#[wasm_bindgen]
pub fn asymmetric_decrypt(pk: &str, ciphertext: Vec<u8>) -> Vec<u8> {
    console_error_panic_hook::set_once();
    let bytes = base64::decode(&pk).unwrap();
    
    let private_key = RSAPrivateKey::from_pkcs8(&bytes)
        .expect("failed to parse key");
    
    let padding = PaddingScheme::new_oaep::<sha2::Sha256>();
    return private_key.decrypt(padding, &ciphertext)
        .expect("failed to decrypt");
}

#[wasm_bindgen]
pub fn symmetric_encrypt(plaintext: &str, key: &str) -> Vec<u8> {
    console_error_panic_hook::set_once();
    let cipher_key = Key::from_slice(key.as_bytes()); // key must be 32 bytes length

    let cipher = Aes256Gcm::new(&cipher_key);
    // In cryptography, a nonce is an arbitrary number that can be used just once in a 
    // cryptographic communication.
    let nonce = Nonce::from_slice(b"unique nonce");
    cipher.encrypt(nonce, plaintext.as_ref()).expect("failed to encrypt")
}

#[wasm_bindgen]
pub fn symmetric_decrypt(ciphertext: Vec<u8>, key: &str) -> Vec<u8> {
    console_error_panic_hook::set_once();
    let cipher_key = Key::from_slice(key.as_bytes());
    let cipher = Aes256Gcm::new(&cipher_key);
    let nonce = Nonce::from_slice(b"unique nonce");
    cipher.decrypt(nonce, ciphertext.as_ref()).expect("failed to decrypt")
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::*;
    use super::*;

    #[test]
    #[wasm_bindgen_test]
    fn test_asymmetric_encrypt_and_decrypt() {
        let mut rng = OsRng;
        let bits = 2048;
        let private_key = RSAPrivateKey::new(&mut rng, bits).unwrap();
        let public_key = RSAPublicKey::from(&private_key);

        let encrypted = asymmetric_encrypt(&base64::encode(public_key), "hello".to_string());
        let decrypted = asymmetric_decrypt(private_key, encrypted);

        assert_eq!(
            String::from("hello"), 
            String::from_utf8(decrypted).unwrap()
        );
    }
}