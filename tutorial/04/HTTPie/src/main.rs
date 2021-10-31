use clap::{AppSettings,Clap};


struct Opts {
    subcmd: Subcommand,
}

enum Subcommand {

}




fn main() {

    let opts: Opts = Opts::parse();

    println!("Hello, world!");
}
