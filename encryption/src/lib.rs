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
struct RSAKeyPair {
  private_key: RSAPrivateKey,
  public_key: RSAPublicKey
}

#[wasm_bindgen]
pub fn generate_rsa_key (bits: i32) -> String {
  let mut rng = OsRng;
  let private_key = RSAPrivateKey::new(&mut rng, bits as usize)
    .expect("failed to generate a key");
  let public_key = private_key.to_public_key();
  let key_pair = RSAKeyPair {private_key: private_key, public_key: public_key};
  return serde_json::to_string(&key_pair).unwrap();
}

#[wasm_bindgen]
pub fn asymmetric_encrypt(pk: &str, plaintext: String) -> Vec<u8> {
    console_error_panic_hook::set_once();
    web_sys::console::log_1(&pk.into());
    web_sys::console::log_1(JsValue::from_str(&plaintext).as_ref());

    let mut rng = OsRng;
    // the public key string comes as a base64 encoded string so we should decode it to a Vec<u8>
    // as it's a sequence of 8-bit bytes.
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
    use super::*;

    #[test]
    fn test_asymmetric_encrypt_and_decrypt() {

        // `pkcs1`
        let public_key = "MIIBCgKCAQEAyMEMAk5/cwdB7t54djd8oX1tRo3i/mo/lJNDSFef8D7 \
        12SDDV3oLxVlYLLP6qwME6L/3neUGNRZTL1CZbcZJLSTTpVnHh/JBAODZoOM9aHUfd+LsWdbe \
        CNq/yQPzO8vAye2QXmu2cGABFh3nkXPGmQO59MCJ1j/wFHICZmLtQPljYdRgSE97X7M/9UlY4 \
        tPLwU3Ec/yTVgeGrtMCmHfZrYpB+2+ErYnwZ9JjC5vKFBD5nJkpCz6j24yzYiNsEhJq9dZ55h \
        Btmm02WVUyLy4VjPt1ms0iJfsNeOSWWhAYYzUx8WhRFfG7+KtwOn4ARCwLDJNivHmLHqICK9X \
        0TGsg+wIDAQAB";

        // `pkcs8`
        let private_key = "MIIEvAIBADANBgkqhkiG9w0BAQEFAASCBKYwggSiAgEAAoIBAQDIwQ \
        wCTn9zB0Hu3nh2N3yhfW1GjeL+aj+Uk0NIV5/wPvXZIMNXegvFWVgss/qrAwTov/ed5QY1FlM \
        vUJltxkktJNOlWceH8kEA4Nmg4z1odR934uxZ1t4I2r/JA/M7y8DJ7ZBea7ZwYAEWHeeRc8aZ \
        A7n0wInWP/AUcgJmYu1A+WNh1GBIT3tfsz/1SVji08vBTcRz/JNWB4au0wKYd9mtikH7b4Sti \
        fBn0mMLm8oUEPmcmSkLPqPbjLNiI2wSEmr11nnmEG2abTZZVTIvLhWM+3WazSIl+w145JZaEB \
        hjNTHxaFEV8bv4q3A6fgBELAsMk2K8eYseogIr1fRMayD7AgMBAAECggEATMeQeT1efzbyX5J \
        UjhoQn47iAU741wb5xDBCVmO09uSNkHp4W3eBEvxlvcsj/s3LzdzFrmzOX/Vm4Ty5QYPJrutb \
        yuy4ZGUhuV1FvsPKU64f1/FBJecIymgnW5EYMPFvjYs/YO1fPEFriPLtAvM9rH4BW/tbfEa94 \
        oPsKQ6C4QsSUl7kybiEpadOdNWOsppI3MdeWa5E+bJyMnUt24xbAMJiy+g3jBwgdTnCylJlWO \
        TGin4OQun6J67yxh85cT4K89zy+FvDYQvKlOwmc6iIlnXF1/gDW7xKZSnLnW53pIMm2W/CD6d \
        DZK4AcVZz+iHJRuhblCdMbRdgud3KlFloMQKBgQDNJ5qREjQjccZl96gFqMuc/UjVLu1Pkh9s \
        opd5O2sqN6W3t/k/FVfwj+R8KD9WtGdBe5t3TK7yWm/aLr6kMbOhpaJK2kUht0ZZMF7drqXhs \
        oO5BNAI3/eINk5rbGfINhC94n/AgAEQqIt8xFZEasDjbDvnRnHmh3uxfouLftTyGQKBgQD6gj \
        0WGLrVN0nyKh6rViEb4/7GTZoZff3GmD6t2UNnRU2r7yuYwl61QuR5iCxwzqF7l07CzRe0grR \
        2XVHj7ZI/+/l9IorNHBmpZrSKnWfVaOP5xF6P8jISdNMX2OnCm6TNUVc47gHg3Nd0abyaDAnw \
        ZSiWXkZOhIrzOK2wkk7WMwKBgHoT9QAaivgEfgCzTEKB3RfACJUC5agR1a05W1WCrBRlJLGcQ \
        i7trnSxy06uTHJ1y3PF8A8fjIMemayFYGAoSzIHc/mCxNx5SN1N0UWs5XzWU1fHq1t8mrltR2 \
        DZEzCqK3Ay1b5g7UFSah6GXqc7tygdgc/vtVtuh4ZpPPR3NpQRAoGAHJOGzGJ9zZzE/q73WKw \
        2kqvrgEuR7J6GuITdgY9rNPie56Ic4wwpKZw72/FAwI2RtjuIbIhgz+KpG2eB0/Jh4JKp3zB1 \
        jPhLpRtmNPnHwnx4pSFVeC40sA5LlkjUQbLnldRVktTzueb3XPxC23/MEnQ3sQ+IDaTpBN3OU \
        noeLCkCgYBq+rolBYcTBXV2ketipM7Rzsp8HbCCVVSYZrJmJPGPZI0MmlMNTkO6mQJsEePOOm \
        5OnbvzX3GNUbjNRv7EXAZPduvYMfz1acGo75L+R/qPz9N64nB3b/oVhZhG9Oxo136vWWS5Ikq \
        FKmfx16UaH6rz6pjq9Lj9RP5xeddOCqq+AA==";
        
        let encrypted = asymmetric_encrypt(public_key, String::from("hello"));
        let decrypted = asymmetric_decrypt(private_key, encrypted);

        println!("{}", std::str::from_utf8(&decrypted).unwrap());

        assert_eq!("hello", std::str::from_utf8(&decrypted).unwrap());
    }
}