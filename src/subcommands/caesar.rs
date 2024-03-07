use crate::algorithms;
use crate::internal::{cmd_mode_enum::CmdMode, convert};

pub fn caesar(mode: CmdMode, text: String) {
    let vec_text: Vec<u8> = convert::string_to_vec(text);
    match mode {
        CmdMode::Enc => {
            // encrypt
            let cipher_text: Vec<u8> = algorithms::caesar::encrypt(&vec_text);
            // output
            println!("Result: {}", convert::vec_to_string(cipher_text));
        }
        CmdMode::Dec => {
            // decrypt
            let plain_text: Vec<u8> = algorithms::caesar::decrypt(&vec_text);
            // output
            println!("Result: {}", convert::vec_to_string(plain_text));
        }
    }
}
