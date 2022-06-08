use std::env::args;
use std::process::exit;
pub mod lib;
pub mod cli;
use cli::Args;

pub fn run(args: Args) -> String {
    lib::truncate(args.query, args.matches)
}

fn main() {
    let argc = args().skip(1).count();
    if argc == 0 {
        match Args::stdin() {
            Ok(stdin) => {
                println!("{}", run(stdin));
                exit(0);
            },
            Err(e) => {
                eprintln!("Unknown error occurred {:?}", e);
                exit(1);
            }
        }
    } else if argc == 1 {
        let arg = args().skip(1).collect();
        println!("{}", run(Args::new(arg, 1)));
        exit(0);
    }

    // Otherwise, the code is running with multiple arguments and that should be handled by clap
    let args = Args::arg_parse();
    println!("{}", run(args));
    exit(0);
}
