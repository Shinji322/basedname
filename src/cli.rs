use std::io::stdin;
use std::error::Error;
use clap::Parser;


#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(short)]
    pub query: String, // Simple query
    #[clap(short, default_value_t = 0)]
    pub matches: u8 // [1..query.len()]
}
impl Args {
    pub fn new(query: String, matches: u8) -> Self {
        Args { query, matches }
    }
    pub fn stdin() -> Result<Self, Box<dyn Error>> {
        let mut buffer = String::new();
        let stdin = stdin();
        stdin.read_line(&mut buffer)?;
        Ok(Args::new(buffer, 1))
    }
    // This should be simply called Args::parse()
    // However, the compiler is unsure if it's a recursive function or a derived function
    // As such, a simple rename is in order
    pub fn arg_parse() -> Self {
        Args::parse() // This is a clap related function that just does magic
    }
}
