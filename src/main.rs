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
            help,
        } => {
            if help{
                todo!("implement help");
            }
            if mode == "enc" {
                println!("Result: {}", algorithms::caesar::encrypt(text));
            } else if mode == "dec" {
                println!("Result: {}", algorithms::caesar::decrypt(text));
            }
        }
    }
}
