mod algorithms;
use structopt::StructOpt;
use rand::Rng;

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
        // key for cipher
        key: Option<u8>,
    },
}

fn main() {
    match Opt::from_args() {
        Opt::Caesar { mode, text } => {
            if mode == "enc" {
                println!("Result: {}", algorithms::caesar::encrypt(text));
            } else if mode == "dec" {
                println!("Result: {}", algorithms::caesar::decrypt(text));
            } else {
                unimplemented!("error: unavailable mode");
            }
        }
        Opt::Rot { mode, text , key} => {
            let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
            let final_key:u8=key.unwrap_or(rng.gen_range(1..26));
            
            if mode == "enc" {
                println!("Result: {}", algorithms::rot::encrypt(final_key, text));
            } else if mode == "dec" {
                println!("Result: {}", algorithms::rot::decrypt(final_key, text));
            } else {
                unimplemented!("error: unavailable mode");
            }
        }
    }
}
