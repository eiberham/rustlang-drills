extern crate base64;
extern crate console_error_panic_hook;
extern crate web_sys;

use wasm_bindgen::prelude::*;
use rsa::{PublicKey, RSAPrivateKey, RSAPublicKey, PaddingScheme};
use rand::{rngs::OsRng};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, NewAead};
use std::str;

#[wasm_bindgen]
pub fn asymmetric_encrypt(pk: &str, plaintext: &str) -> Vec<u8> {
    console_error_panic_hook::set_once();

    let mut rng = OsRng;
    // the public key string comes as a base64 encoded string so we should decode 
    // it to a Vec<u8> as it's a sequence of 8-bit bytes.
    let bytes = base64::decode(&pk).unwrap();
    let public_key: RSAPublicKey = RSAPublicKey::from_pkcs1(&bytes)
        .expect("failed to parse key");

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
pub fn symmetric_encrypt(plaintext: &str, key: &str) -> Vec<u8>  {
    console_error_panic_hook::set_once();
    let cipher_key = Key::from_slice(key.as_bytes()); // key must be 32 bytes length
    let cipher = Aes256Gcm::new(cipher_key);
    
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
        let public_key = "MIIBCgKCAQEAyMEMAk5/cwdB7t54djd8oX1tRo3i/mo/lJNDSFef8D712SDDV3oLxVlYLLP6qwME6L/3neUGNRZTL1CZbcZJLSTTpVnHh/JBAODZoOM9aHUfd+LsWdbeCNq/yQPzO8vAye2QXmu2cGABFh3nkXPGmQO59MCJ1j/wFHICZmLtQPljYdRgSE97X7M/9UlY4tPLwU3Ec/yTVgeGrtMCmHfZrYpB+2+ErYnwZ9JjC5vKFBD5nJkpCz6j24yzYiNsEhJq9dZ55hBtmm02WVUyLy4VjPt1ms0iJfsNeOSWWhAYYzUx8WhRFfG7+KtwOn4ARCwLDJNivHmLHqICK9X0TGsg+wIDAQAB";
        let private_key = "MIIEvAIBADANBgkqhkiG9w0BAQEFAASCBKYwggSiAgEAAoIBAQDIwQwCTn9zB0Hu3nh2N3yhfW1GjeL+aj+Uk0NIV5/wPvXZIMNXegvFWVgss/qrAwTov/ed5QY1FlMvUJltxkktJNOlWceH8kEA4Nmg4z1odR934uxZ1t4I2r/JA/M7y8DJ7ZBea7ZwYAEWHeeRc8aZA7n0wInWP/AUcgJmYu1A+WNh1GBIT3tfsz/1SVji08vBTcRz/JNWB4au0wKYd9mtikH7b4StifBn0mMLm8oUEPmcmSkLPqPbjLNiI2wSEmr11nnmEG2abTZZVTIvLhWM+3WazSIl+w145JZaEBhjNTHxaFEV8bv4q3A6fgBELAsMk2K8eYseogIr1fRMayD7AgMBAAECggEATMeQeT1efzbyX5JUjhoQn47iAU741wb5xDBCVmO09uSNkHp4W3eBEvxlvcsj/s3LzdzFrmzOX/Vm4Ty5QYPJrutbyuy4ZGUhuV1FvsPKU64f1/FBJecIymgnW5EYMPFvjYs/YO1fPEFriPLtAvM9rH4BW/tbfEa94oPsKQ6C4QsSUl7kybiEpadOdNWOsppI3MdeWa5E+bJyMnUt24xbAMJiy+g3jBwgdTnCylJlWOTGin4OQun6J67yxh85cT4K89zy+FvDYQvKlOwmc6iIlnXF1/gDW7xKZSnLnW53pIMm2W/CD6dDZK4AcVZz+iHJRuhblCdMbRdgud3KlFloMQKBgQDNJ5qREjQjccZl96gFqMuc/UjVLu1Pkh9sopd5O2sqN6W3t/k/FVfwj+R8KD9WtGdBe5t3TK7yWm/aLr6kMbOhpaJK2kUht0ZZMF7drqXhsoO5BNAI3/eINk5rbGfINhC94n/AgAEQqIt8xFZEasDjbDvnRnHmh3uxfouLftTyGQKBgQD6gj0WGLrVN0nyKh6rViEb4/7GTZoZff3GmD6t2UNnRU2r7yuYwl61QuR5iCxwzqF7l07CzRe0grR2XVHj7ZI/+/l9IorNHBmpZrSKnWfVaOP5xF6P8jISdNMX2OnCm6TNUVc47gHg3Nd0abyaDAnwZSiWXkZOhIrzOK2wkk7WMwKBgHoT9QAaivgEfgCzTEKB3RfACJUC5agR1a05W1WCrBRlJLGcQi7trnSxy06uTHJ1y3PF8A8fjIMemayFYGAoSzIHc/mCxNx5SN1N0UWs5XzWU1fHq1t8mrltR2DZEzCqK3Ay1b5g7UFSah6GXqc7tygdgc/vtVtuh4ZpPPR3NpQRAoGAHJOGzGJ9zZzE/q73WKw2kqvrgEuR7J6GuITdgY9rNPie56Ic4wwpKZw72/FAwI2RtjuIbIhgz+KpG2eB0/Jh4JKp3zB1jPhLpRtmNPnHwnx4pSFVeC40sA5LlkjUQbLnldRVktTzueb3XPxC23/MEnQ3sQ+IDaTpBN3OUnoeLCkCgYBq+rolBYcTBXV2ketipM7Rzsp8HbCCVVSYZrJmJPGPZI0MmlMNTkO6mQJsEePOOm5OnbvzX3GNUbjNRv7EXAZPduvYMfz1acGo75L+R/qPz9N64nB3b/oVhZhG9Oxo136vWWS5IkqFKmfx16UaH6rz6pjq9Lj9RP5xeddOCqq+AA==";
        let encrypted = asymmetric_encrypt(public_key, "hello");
        let decrypted = asymmetric_decrypt(private_key, encrypted);

        assert_eq!(
            String::from("hello"), 
            String::from_utf8(decrypted).unwrap()
        );
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_symmetric_encrypt_and_decrypt() {
        let key: &str = "13d5e16c29764887a6f9630a23ecb069";
        let encrypted = symmetric_encrypt("hello", key);
        let decrypted = symmetric_decrypt(encrypted, key);

        assert_eq!(
            String::from("hello"), 
            String::from_utf8(decrypted).unwrap()
        );
    }
}