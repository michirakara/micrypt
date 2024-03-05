use crate::internal::cmd_mode_enum::CmdMode;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
/// encryption & decryption tool
pub enum Opt {
    /// caeser cipher
    Caesar {
        /// available modes: enc, dec
        mode: CmdMode,
        /// text to encrypt/decrypt
        text: String,
    },
    /// ROT-n cipher
    Rot {
        /// available modes: enc, dec
        mode: CmdMode,
        /// text to encrypt/decrypt
        text: String,
        /// key for cipher. automatically generates random key when not specified
        key: Option<u8>,
    },
    /// vigenère cipher
    Vigenere {
        /// available modes: enc, dec
        mode: CmdMode,
        /// text to encrypt/decrypt
        text: String,
        /// key for cipher. automatically generates random key when not specified
        key: Option<String>,
    },
}
