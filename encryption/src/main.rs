use rsa::{PublicKey, RSAPrivateKey, RSAPublicKey, PaddingScheme};
use rand::{rngs::OsRng};
use std::str;
use serde::{Serialize, Deserialize};


fn main() {
    println!("Hello, encryption!");

    let mut rng = OsRng;
    let bits = 2048;
    let private_key = RSAPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let public_key = RSAPublicKey::from(&private_key);

    let encrypted = encrypt(rng, public_key, String::from("hello"));
    println!("encrypted: {}", encrypted);
    /* let decrypted = decrypt(private_key, encrypted.into_bytes());
    println!("decrypted: {}", decrypted); */
}

fn encrypt(mut rng: OsRng, pk: &str, text: String) -> Vec<u8> {
    // let data = b"hello world";
    let public_key: RSAPublicKey = serde_json::from_str(pk).unwrap();
    let padding = PaddingScheme::new_oaep::<sha2::Sha256>();
    return public_key.encrypt(&mut rng, padding, text.as_bytes()).expect("failed to encrypt");
    
    /* match str::from_utf8(&enc_data.into) {
        Ok(string) => String::from(string),
        Err(err) => panic!("something went bad: {}", err)
    } */
}

fn decrypt(private_key: RSAPrivateKey, enc_data: Vec<u8>) -> String {
    let padding = PaddingScheme::new_oaep::<sha2::Sha256>();
    let dec_data = private_key.decrypt(padding, &enc_data).expect("failed to decrypt");
    match str::from_utf8(&dec_data) {
        Ok(string) => String::from(string),
        Err(_) => panic!("something went wrong")
    }
}

