mod algorithms;
use rand::Rng;
use structopt::StructOpt;

//TODO: エラーを実装する
#[derive(StructOpt, Debug)]
/// encryption & decryption tool
enum Opt {
    /// caeser cipher
    Caesar {
        #[structopt(short)]
        /// available modes: enc, dec
        mode: String,
        #[structopt(short)]
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
    Vigenere{
        /// available modes: enc, dec
        mode: String,
        /// text to encrypt/decrypt
        text: String,
        /// key for cipher. automatically generates random key when not specified
        key: Option<String>
    }
}

fn main() {
    match Opt::from_args() {
        // Caesar cipher
        Opt::Caesar { mode, text } => {
            if mode == "enc" {
                // encrypt
                let cipher_text: String = algorithms::caesar::encrypt(text);
                // output
                println!("Result: {}", cipher_text);
            } else if mode == "dec" {
                // decrypt
                let plain_text: String = algorithms::caesar::decrypt(text);
                // output
                println!("Result: {}", plain_text);
            } else {
                // when mode is not `enc` or `dec`
                unimplemented!("error: unavailable mode");
            }
        }
        // ROT-n cipher
        Opt::Rot { mode, text, key } => {
            if mode == "enc" {
                // generate key when unspecified
                let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
                let final_key: u8 = key.unwrap_or(rng.gen_range(1..26));
                // encrypt
                let cipher_text: String = algorithms::rot::encrypt(final_key, text);
                // output
                println!("Key: {}", final_key);
                println!("Result: {}", cipher_text);
            } else if mode == "dec" {
                // decrypt
                let plain_text: String = algorithms::rot::decrypt(key.unwrap(), text);
                // output
                println!("Key: {}", key.unwrap());
                println!("Result: {}", plain_text);
            } else {
                // when mode is not `enc` or `dec`
                unimplemented!("error: unavailable mode");
            }
        }
        // Vigenere cipher
        Opt::Vigenere { mode, text, key } => {
            if mode=="enc" {
                let mut final_key:String;
                if key==None{
                    // generate key
                    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
                    
                    let key_length:i32=rng.gen_range(1..=10);
                    final_key=String::new();
                    for _ in 0..key_length{
                        let rand:u8=rng.gen_range(65..=90);
                        let next_key:&str=&(rand as char).to_string();
                        final_key+=next_key;
                    }
                }else{
                    // when key is specified
                    final_key=key.unwrap().to_uppercase();
                }
                // encrypt
                let cipher_text: String=algorithms::vigenere::encrypt(final_key.clone(),text);
                // output
                println!("Key: {}",final_key);
                println!("Result: {}", cipher_text);
            } else if mode=="dec"{
                // decrypt
                let final_key=key.unwrap().to_uppercase();
                let plain_text: String = algorithms::vigenere::decrypt(final_key.clone(),text);
                // output
                println!("Key: {}", final_key);
                println!("Result: {}", plain_text);
            }else{
                // when mode is not `enc` or `dec`
                unimplemented!("error: unavailable mode");
            }
        }
    }
}
