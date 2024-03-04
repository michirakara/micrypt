mod algorithms;
use structopt::StructOpt;
use rand::Rng;

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
        /// key for cipher
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
            if mode == "enc" {
                // generate key when unspecified
                let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
                let final_key:u8=key.unwrap_or(rng.gen_range(1..26));
                // encrypt
                println!("Key: {}", final_key);
                println!("Result: {}", algorithms::rot::encrypt(final_key, text));
            } else if mode == "dec" {
                // decrypt
                println!("Key: {}", key.unwrap());
                println!("Result: {}", algorithms::rot::decrypt(key.unwrap(), text));
            } else {
                unimplemented!("error: unavailable mode");
            }
        }
    }
}
