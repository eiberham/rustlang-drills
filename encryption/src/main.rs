// Base64 is a group of binary-to-text encoding schemes that represent binary 
// data (more specifically, a sequence of 8-bit bytes) in an ASCII string format 

use wasm_bindgen::prelude::*;
use rsa::{PublicKey, RSAPrivateKey, RSAPublicKey, PaddingScheme};
use rand::{rngs::OsRng};
use aes_gcm::{Aes256Gcm, Nonce}; // Or `Aes128Gcm`
use aes_gcm::aead::{Aead, NewAead};
use std::str;
extern crate base64;


fn main() {
    println!("Proof of concept RSA encryption");
    // To generate my own key.
    // let mut rng = OsRng;
    // let bits = 2048;
    // let private_key = RSAPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    // let public_key = RSAPublicKey::from(&private_key);

    // base64 encoded public key previously encoded in `PKCS1`
    let public_key = "MIIBCgKCAQEAyMEMAk5/cwdB7t54djd8oX1tRo3i/mo/lJNDSFef8D712SDDV3oLxVlYLLP6qwME6L/3neUGNRZTL1CZbcZJLSTTpVnHh/JBAODZoOM9aHUfd+LsWdbeCNq/yQPzO8vAye2QXmu2cGABFh3nkXPGmQO59MCJ1j/wFHICZmLtQPljYdRgSE97X7M/9UlY4tPLwU3Ec/yTVgeGrtMCmHfZrYpB+2+ErYnwZ9JjC5vKFBD5nJkpCz6j24yzYiNsEhJq9dZ55hBtmm02WVUyLy4VjPt1ms0iJfsNeOSWWhAYYzUx8WhRFfG7+KtwOn4ARCwLDJNivHmLHqICK9X0TGsg+wIDAQAB";
    // base64 encoded private key previously encoded in `PKCS8`
    let private_key = "MIIEvAIBADANBgkqhkiG9w0BAQEFAASCBKYwggSiAgEAAoIBAQDIwQwCTn9zB0Hu3nh2N3yhfW1GjeL+aj+Uk0NIV5/wPvXZIMNXegvFWVgss/qrAwTov/ed5QY1FlMvUJltxkktJNOlWceH8kEA4Nmg4z1odR934uxZ1t4I2r/JA/M7y8DJ7ZBea7ZwYAEWHeeRc8aZA7n0wInWP/AUcgJmYu1A+WNh1GBIT3tfsz/1SVji08vBTcRz/JNWB4au0wKYd9mtikH7b4StifBn0mMLm8oUEPmcmSkLPqPbjLNiI2wSEmr11nnmEG2abTZZVTIvLhWM+3WazSIl+w145JZaEBhjNTHxaFEV8bv4q3A6fgBELAsMk2K8eYseogIr1fRMayD7AgMBAAECggEATMeQeT1efzbyX5JUjhoQn47iAU741wb5xDBCVmO09uSNkHp4W3eBEvxlvcsj/s3LzdzFrmzOX/Vm4Ty5QYPJrutbyuy4ZGUhuV1FvsPKU64f1/FBJecIymgnW5EYMPFvjYs/YO1fPEFriPLtAvM9rH4BW/tbfEa94oPsKQ6C4QsSUl7kybiEpadOdNWOsppI3MdeWa5E+bJyMnUt24xbAMJiy+g3jBwgdTnCylJlWOTGin4OQun6J67yxh85cT4K89zy+FvDYQvKlOwmc6iIlnXF1/gDW7xKZSnLnW53pIMm2W/CD6dDZK4AcVZz+iHJRuhblCdMbRdgud3KlFloMQKBgQDNJ5qREjQjccZl96gFqMuc/UjVLu1Pkh9sopd5O2sqN6W3t/k/FVfwj+R8KD9WtGdBe5t3TK7yWm/aLr6kMbOhpaJK2kUht0ZZMF7drqXhsoO5BNAI3/eINk5rbGfINhC94n/AgAEQqIt8xFZEasDjbDvnRnHmh3uxfouLftTyGQKBgQD6gj0WGLrVN0nyKh6rViEb4/7GTZoZff3GmD6t2UNnRU2r7yuYwl61QuR5iCxwzqF7l07CzRe0grR2XVHj7ZI/+/l9IorNHBmpZrSKnWfVaOP5xF6P8jISdNMX2OnCm6TNUVc47gHg3Nd0abyaDAnwZSiWXkZOhIrzOK2wkk7WMwKBgHoT9QAaivgEfgCzTEKB3RfACJUC5agR1a05W1WCrBRlJLGcQi7trnSxy06uTHJ1y3PF8A8fjIMemayFYGAoSzIHc/mCxNx5SN1N0UWs5XzWU1fHq1t8mrltR2DZEzCqK3Ay1b5g7UFSah6GXqc7tygdgc/vtVtuh4ZpPPR3NpQRAoGAHJOGzGJ9zZzE/q73WKw2kqvrgEuR7J6GuITdgY9rNPie56Ic4wwpKZw72/FAwI2RtjuIbIhgz+KpG2eB0/Jh4JKp3zB1jPhLpRtmNPnHwnx4pSFVeC40sA5LlkjUQbLnldRVktTzueb3XPxC23/MEnQ3sQ+IDaTpBN3OUnoeLCkCgYBq+rolBYcTBXV2ketipM7Rzsp8HbCCVVSYZrJmJPGPZI0MmlMNTkO6mQJsEePOOm5OnbvzX3GNUbjNRv7EXAZPduvYMfz1acGo75L+R/qPz9N64nB3b/oVhZhG9Oxo136vWWS5IkqFKmfx16UaH6rz6pjq9Lj9RP5xeddOCqq+AA==";

    let encrypted = asymmetric_encrypt(public_key, String::from("hello"));
    let decrypted = asymmetric_decrypt(private_key, encrypted);

    // println!("encrypted: {:?}", String::from_utf8(encrypted));
    println!("decrypted: {:?}", String::from_utf8(decrypted));
}

#[wasm_bindgen]
pub fn asymmetric_encrypt(pk: &str, text: String) -> Vec<u8> {
    let mut rng = OsRng;
    // the public key string comes as a base64 encoded string so we should decode it to a Vec<u8>
    // becase it's a sequence of 8-bit bytes.
    let bytes = base64::decode(&pk).unwrap();

    // PKCS#1 OAEP is an asymmetric cipher based on RSA and the OAEP padding. It is described in 
    // RFC8017 where it is called RSAES-OAEP.
    // It can only encrypt messages slightly shorter than the RSA modulus (a few hundred bytes).

    let public_key: RSAPublicKey = RSAPublicKey::from_pkcs1(&bytes)
        .expect("failed to parse key");

    let padding = PaddingScheme::new_oaep::<sha2::Sha256>();
    return public_key.encrypt(&mut rng, padding, text.as_bytes())
        .expect("failed to encrypt");
}

#[wasm_bindgen]
pub fn asymmetric_decrypt(pk: &str, enc_data: Vec<u8>) -> Vec<u8> {
    
    let bytes = base64::decode(&pk).unwrap();
    
    let private_key = RSAPrivateKey::from_pkcs8(&bytes)
        .expect("failed to parse key");
    
    let padding = PaddingScheme::new_oaep::<sha2::Sha256>();
    return private_key.decrypt(padding, &enc_data)
        .expect("failed to decrypt");
}

#[wasm_bindgen]
pub fn symmetric_encrypt(text: &str, key: &str) -> Vec<u8> {
    let cipher = Aes256Gcm::new(key);
    // In cryptography, a nonce is an arbitrary number that can be used just once in a cryptographic 
    // communication. It is similar in spirit to a nonce word, hence the name. It is often a random 
    // or pseudo-random number issued in an authentication protocol to ensure that old communications 
    // cannot be reused in replay attacks. They can also be useful as initialization vectors and in 
    // cryptographic hash functions.
    let nonce = Nonce::from_slice(b"unique nonce");
    cipher.encrypt(nonce, text).expect("failed to encrypt")
}

#[wasm_bindgen]
pub fn symmetric_decrypt(text: &str, key: &str) -> Vec<u8> {
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(b"unique nonce");
    cipher.decrypt(nonce, text.as_ref()).expect("failed to decrypt")
}

