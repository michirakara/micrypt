mod algorithms;
use rand::Rng;
use structopt::StructOpt;

//TODO: エラーを実装する
#[derive(StructOpt, Debug)]
/// encryption & decryption tool
enum Opt {
    /// caeser cipher
    Caesar {
        /// available modes: enc, dec
        mode: String,
        /// text to encrypt/decrypt
        text: String,
    },
    /// ROT-n cipher
    Rot {
        /// available modes: enc, dec
        mode: String,
        /// text to encrypt/decrypt
        text: String,
        /// key for cipher. automatically generates random key when not specified
        key: Option<u8>,
    },
    /// vigenère cipher
    Vigenere {
        /// available modes: enc, dec
        mode: String,
        /// text to encrypt/decrypt
        text: String,
        /// key for cipher. automatically generates random key when not specified
        key: Option<String>,
    },
}

fn string_to_vec(s:String) -> Vec<u8>{
    let mut to_ret:Vec<u8> = Vec::new();
    for i in s.bytes(){
        to_ret.push(i);
    }
    to_ret
}

fn vec_to_string(s:Vec<u8>) -> String{
    let mut to_ret:String = String::new();
    for i in s{
        to_ret+=&((i as char).to_string());
    }
    to_ret
}

fn main() {
    match Opt::from_args() {
        // Caesar cipher
        Opt::Caesar { mode, text } => {
            let vec_text: Vec<u8>=string_to_vec(text);
            if mode == "enc" {
                // encrypt
                let cipher_text: Vec<u8> = algorithms::caesar::encrypt(vec_text);
                // output
                println!("Result: {}", vec_to_string(cipher_text));
            } else if mode == "dec" {
                // decrypt
                let plain_text: Vec<u8> = algorithms::caesar::decrypt(vec_text);
                // output
                println!("Result: {}", vec_to_string(plain_text));
            } else {
                // when mode is not `enc` or `dec`
                unimplemented!("error: unavailable mode");
            }
        }
        // ROT-n cipher
        Opt::Rot { mode, text, key } => {
            let vec_text: Vec<u8>=string_to_vec(text);
            if mode == "enc" {
                // generate key when unspecified
                let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
                let final_key: u8 = key.unwrap_or(rng.gen_range(1..26));
                // encrypt
                let cipher_text: Vec<u8> = algorithms::rot::encrypt(final_key, vec_text);
                // output
                println!("Key: {}", final_key);
                println!("Result: {}", vec_to_string(cipher_text));
            } else if mode == "dec" {
                // decrypt
                let plain_text: Vec<u8> = algorithms::rot::decrypt(key.unwrap(), vec_text);
                // output
                println!("Key: {}", key.unwrap());
                println!("Result: {}", vec_to_string(plain_text));
            } else {
                // when mode is not `enc` or `dec`
                unimplemented!("error: unavailable mode");
            }
        }
        // Vigenere cipher
        Opt::Vigenere { mode, text, key } => {
            let vec_text: Vec<u8>=string_to_vec(text);
            if mode == "enc" {
                let mut final_key: Vec<u8> = Vec::new();
                if key == None {
                    // generate key
                    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();

                    let key_length: i32 = rng.gen_range(1..=10);
                    for _ in 0..key_length {
                        let rand: u8 = rng.gen_range(65..=90);
                        final_key.push(rand);
                    }
                } else {
                    // when key is specified
                    final_key = string_to_vec(key.unwrap().to_uppercase());
                }
                // encrypt
                let cipher_text: Vec<u8> = algorithms::vigenere::encrypt(final_key.clone(), vec_text);
                // output
                println!("Key: {}", vec_to_string(final_key));
                println!("Result: {}", vec_to_string(cipher_text));
            } else if mode == "dec" {
                // decrypt
                let final_key:Vec<u8> = string_to_vec(key.unwrap().to_uppercase());
                let plain_text: Vec<u8> = algorithms::vigenere::decrypt(final_key.clone(), vec_text);
                // output
                println!("Key: {}", vec_to_string(final_key));
                println!("Result: {}", vec_to_string(plain_text));
            } else {
                // when mode is not `enc` or `dec`
                unimplemented!("error: unavailable mode");
            }
        }
    }
}
