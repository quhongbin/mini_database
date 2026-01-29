pub mod init {
    use clap::{Parser, Subcommand, command};

    #[derive(Parser, Debug)]
    #[command(version = "1.0", author = "qhb@gmail.com")]
    pub struct Cli {
        ///Action for db
        #[arg(short, long)]
        pub operator: Option<String>,

        ///patterns for db act
        #[arg(short, long)]
        pub pattern: Option<String>,

        #[command(subcommand)]
        pub action: Option<Operator>,
    }
    #[derive(Debug, Subcommand)]
    pub enum Operator {
        #[command(about = "set data for mini_database ( default 10M UTF-8 )")]
        Set {
            #[arg(long)]
            memory: u16,
            #[arg(long, default_value = "UTF-8")]
            charactor: String,
        },
        /// set data for mini_database
        Get {
            #[arg(short, long)]
            get_key: String,
        },
        /// set data for mini_database
        Remove {
            #[arg(short, long)]
            remove_key: String,
            #[arg(short, long)]
            remove_value: String,
        },
    }
}
// impl<K, T> Operator<K, T> {
//     fn set(self: &Self, key: K, value: T) {}
// }
// fn handle(argmatches: &ArgMatches) {
//     match argmatches.subcommand() {
//         Some(("set", a)) => Operator::set(&self, key, value),
//         None => (),
//     }
// } }
