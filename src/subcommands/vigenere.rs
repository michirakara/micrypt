use crate::algorithms;
use crate::internal::{cmd_mode_enum::CmdMode, convert};
use rand::Rng;

pub fn vigenere(mode: CmdMode, text: String, key: Option<String>) {
    let vec_text: Vec<u8> = convert::string_to_vec(text);
    match mode {
        CmdMode::Enc => {
            let mut final_key: Vec<u8> = Vec::new();
            if key.is_none() {
                // generate key
                let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
                let key_length: i32 = rng.gen_range(1..=10);
                for _ in 0..key_length {
                    let rand: u8 = rng.gen_range(65..=90);
                    final_key.push(rand);
                }
            } else {
                // when key is specified
                final_key = convert::string_to_vec(key.unwrap().to_uppercase());
            }
            // encrypt
            let cipher_text: Vec<u8> = algorithms::vigenere::encrypt(final_key.clone(), vec_text);
            // output
            println!("Key: {}", convert::vec_to_string(final_key));
            println!("Result: {}", convert::vec_to_string(cipher_text));
        }
        CmdMode::Dec => {
            // decrypt
            let final_key: Vec<u8> = convert::string_to_vec(key.unwrap().to_uppercase());
            let plain_text: Vec<u8> = algorithms::vigenere::decrypt(final_key.clone(), vec_text);
            // output
            println!("Key: {}", convert::vec_to_string(final_key));
            println!("Result: {}", convert::vec_to_string(plain_text));
        }
    }
}
