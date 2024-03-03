mod algorithms;
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
        // key for cipher
        key: u8,
        /// text to encrypt/decrypt
        text: String,
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
        Opt::Rot { mode, key, text } => {
            if mode == "enc" {
                println!("Result: {}", algorithms::rot::encrypt(key, text));
            } else if mode == "dec" {
                println!("Result: {}", algorithms::rot::decrypt(key, text));
            } else {
                unimplemented!("error: unavailable mode");
            }
        }
    }
}
