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
            init::Operator::Get { get_key: st } => {
                println!("{}", st)
            }
            init::Operator::Remove {
                remove_key: st,
                remove_value: test,
            } => {
                println!("{},{}", st, test)
            }
        },
        None => println!("error occur"),
    }
}
