// Base64 is a group of binary-to-text encoding schemes that represent binary 
// data (more specifically, a sequence of 8-bit bytes) in an ASCII string format 

use wasm_bindgen::prelude::*;
use rsa::{PublicKey, RSAPrivateKey, RSAPublicKey, PaddingScheme};
use rand::{rngs::OsRng};
use aes_gcm::{Aes256Gcm, Key, Nonce}; // Or `Aes128Gcm`
use aes_gcm::aead::{Aead, NewAead};
use std::str;
extern crate base64;

#[wasm_bindgen]
pub fn asymmetric_encrypt(pk: &str, plaintext: String) -> Vec<u8> {
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
    return public_key.encrypt(&mut rng, padding, plaintext.as_bytes())
        .expect("failed to encrypt");
}

#[wasm_bindgen]
pub fn asymmetric_decrypt(pk: &str, ciphertext: Vec<u8>) -> Vec<u8> {
    
    let bytes = base64::decode(&pk).unwrap();
    
    let private_key = RSAPrivateKey::from_pkcs8(&bytes)
        .expect("failed to parse key");
    
    let padding = PaddingScheme::new_oaep::<sha2::Sha256>();
    return private_key.decrypt(padding, &ciphertext)
        .expect("failed to decrypt");
}

// Encrypts the given plaintext with aes-256 (32 bytes).

#[wasm_bindgen]
pub fn symmetric_encrypt(plaintext: &str, key: &str) -> Vec<u8> {
    let cipher_key = Key::from_slice(key.as_bytes());

    let cipher = Aes256Gcm::new(&cipher_key);
    // In cryptography, a nonce is an arbitrary number that can be used just once in a 
    // cryptographic communication.
    let nonce = Nonce::from_slice(b"unique nonce");
    cipher.encrypt(nonce, plaintext.as_ref()).expect("failed to encrypt")
}

#[wasm_bindgen]
pub fn symmetric_decrypt(ciphertext: Vec<u8>, key: &str) -> Vec<u8> {
    let cipher_key = Key::from_slice(key.as_bytes());
    let cipher = Aes256Gcm::new(&cipher_key);
    let nonce = Nonce::from_slice(b"unique nonce");
    cipher.decrypt(nonce, ciphertext.as_ref()).expect("failed to decrypt")
}

