mod algorithms;
use structopt::StructOpt;

//TODO: エラーを実装する
#[derive(StructOpt, Debug)]
/// encryption & decryption tool
enum Opt {
    /// caeser cipher
    Caesar {
        mode: String,
        text: String,
        #[structopt(short = "h", long)]
        help: bool,
    },
}

fn main() {
    match Opt::from_args() {
        Opt::Caesar {
            mode,
            text,
            help: _,
        } => {
            if mode == "enc" {
                println!("Result: {}", algorithms::caesar::encrypt(text));
            } else if mode == "dec" {
                todo!("decryption mode WIP");
            }
        }
    }
}
