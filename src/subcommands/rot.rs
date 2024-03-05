use crate::algorithms;
use crate::internal::{cmd_mode_enum::CmdMode, convert};
use rand::Rng;

pub fn rot(mode: CmdMode, text: String, key: Option<u8>) {
    let vec_text: Vec<u8> = convert::string_to_vec(text);
    match mode {
        CmdMode::Enc => {
            // generate key when unspecified
            let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
            let final_key: u8 = key.unwrap_or(rng.gen_range(1..26));
            // encrypt
            let cipher_text: Vec<u8> = algorithms::rot::encrypt(final_key, vec_text);
            // output
            println!("Key: {}", final_key);
            println!("Result: {}", convert::vec_to_string(cipher_text));
        }
        CmdMode::Dec => {
            // decrypt
            let plain_text: Vec<u8> = algorithms::rot::decrypt(key.unwrap(), vec_text);
            // output
            println!("Key: {}", key.unwrap());
            println!("Result: {}", convert::vec_to_string(plain_text));
        }
    }
}
