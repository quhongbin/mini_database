use clap::Parser;
use mini_database::init;

fn main() {
    let inputs = init::Cli::parse();
    // println!("{:?}", &inputs.pattern);
    match inputs.action {
        Some(cmd) => match cmd {
            init::Operator::Set { memory, charactor } => {
                println!("{},{}", memory, charactor)
            }
            init::Operator::Get { get_key } => {
                println!("{}", get_key)
            }
            init::Operator::Remove {
                remove_key,
                remove_value,
            } => {
                println!("{}:{}", remove_key, remove_value)
            }
        },
        None => println!("error occur"),
    }
}

