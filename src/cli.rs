use std::io::stdin;
use std::error:Error;
use clap::Parser;


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

pub fn load_stdin() -> Result<String, Box<dyn Error>> {
    let mut buffer = String::new();
    let stdin = stdin();
    stdin.read_line(&mut buffer)?;
    Ok(buffer.trim().to_string())
}
