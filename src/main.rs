mod algorithms;
mod internal;
mod subcommands;
use internal::cmd_enum::Opt;
use structopt::StructOpt;

fn main() {
    match Opt::from_args() {
        // Caesar cipher
        Opt::Caesar { mode, text } => {
            subcommands::caesar::caesar(mode, text);
        }
        // ROT-n cipher
        Opt::Rot { mode, text, key } => {
            subcommands::rot::rot(mode, text, key);
        }
        // Vigenere cipher
        Opt::Vigenere { mode, text, key } => {
            subcommands::vigenere::vigenere(mode, text, key);
        }
    }
}
