use clap::Parser;
use mini_database::parse;

fn main() {
    let args = parse::Cli::parse();
    println!("{:?}", args);
}
