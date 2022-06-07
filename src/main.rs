use std::{ io::stdin };
use clap::Parser;

//  _____         _       
// |_   _|__  ___| |_ ___ 
//   | |/ _ \/ __| __/ __|
//   | |  __/\__ \ |_\__ \
//   |_|\___||___/\__|___/
#[test]
fn check_result() {
    assert_eq!(
        run(
            Args {
                query: String::from("filename.txt"),
                matches: 0
            }
        ).unwrap(),
        "filename"
    );
}

//  ____  _                   _                       
// / ___|| |_ _ __ _   _  ___| |_ _   _ _ __ ___  ___ 
// \___ \| __| '__| | | |/ __| __| | | | '__/ _ \/ __|
//  ___) | |_| |  | |_| | (__| |_| |_| | | |  __/\__ \
// |____/ \__|_|   \__,_|\___|\__|\__,_|_|  \___||___/
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(short)]
    query: String,
    #[clap(short, default_value_t = 0)]
    matches: u8
}

impl Args {
    pub fn new(query: String, matches: u8) -> Self {
        Args { query, matches }
    }
}

//  ____  _        _                 
// / ___|| |_ _ __(_)_ __   __ _ ___ 
// \___ \| __| '__| | '_ \ / _` / __|
//  ___) | |_| |  | | | | | (_| \__ \
// |____/ \__|_|  |_|_| |_|\__, |___/
//                         |___/     
pub fn truncate(arg: String, matches: u8) -> String {
    match arg.rmatch_indices('.').nth(matches as usize) {
        Some(index) => {
            arg[..index.0].to_string()
        },
        None => {
            match arg.match_indices('.').next() {
                Some(index) => {
                    arg[..index.0].to_string()
                },
                None => {
                    arg.to_string()
                }
            }
       }
   }
}

pub fn run(args: Args) -> Result<String, Box<dyn std::error::Error>> {
    Ok(truncate(args.query, args.matches))
}
pub fn load_stdin() -> Result<String, Box<dyn std::error::Error>> {
    let mut buffer = String::new();
    let stdin = stdin();
    stdin.read_line(&mut buffer)?;
    Ok(buffer.trim().to_string())
}

fn main() {
    let argc = std::env::args().skip(1).count();
    if argc == 0 {
        match load_stdin() {
            Ok(stdin) => {
                println!("{}", run(Args::new(stdin, 1)).unwrap());
                std::process::exit(0);
            },
            Err(e) => {
                eprintln!("Unknown error occurred {:?}", e);
                std::process::exit(1);
            }
        }
    } else if argc == 1 {
        let arg = std::env::args().skip(1).collect();
        match run(Args::new(arg, 1)) {
            Ok(val) => {
                println!("{}", val);
                std::process::exit(0);
            },
            Err(_) =>  {
                // Never happens, but maybe for later
                std::process::exit(1);
            }
        }
    }

    let args = Args::parse();
    match run(args) {
        Ok(val) => {
            println!("{}", val);
        },
        Err(_) =>  {
            // Never happens, but maybe for later
            std::process::exit(1);
        }
    }
    std::process::exit(0);
}
