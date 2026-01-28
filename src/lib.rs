pub mod parse {
    use clap::Parser;
    #[derive(Parser, Debug)]
    pub struct Cli {
        #[arg(short, long)]
        pattern: String,
        #[arg(short, long)]
        ospath: std::path::PathBuf,
    }
    // enum Commands<K, V> {
    //     Set(K, V),
    //     Get(K),
    //     Remove(K, V),
    // }
}
