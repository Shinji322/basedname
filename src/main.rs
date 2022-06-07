use std::env::args;
use std::process::exit;
pub mod lib;
pub mod cli;
use lib::truncate;
use cli::Args;

pub fn run(args: Args) -> Result<String, Box<dyn std::error::Error>> {
    Ok(truncate(args.query, args.matches))
}

fn main() {
    let argc = args().skip(1).count();
    if argc == 0 {
        match Args::stdin() {
            Ok(stdin) => {
                println!("{}", run(stdin).unwrap());
                exit(0);
            },
            Err(e) => {
                eprintln!("Unknown error occurred {:?}", e);
                exit(1);
            }
        }
    } else if argc == 1 {
        let arg = args().skip(1).collect();
        match run(Args::new(arg, 0)) {
            Ok(val) => {
                println!("{}", val);
                exit(0);
            },
            Err(_) =>  {
                // Never happens, but maybe for later
                exit(1);
            }
        }
    }

    // Otherwise, the code is running with multiple arguments and that should be handled by clap
    let args = Args::arg_parse();
    match run(args) {
        Ok(val) => {
            println!("{}", val);
        },
        Err(_) =>  {
            // Never happens, but maybe for later
            exit(1);
        }
    }
    exit(0);
}
