mod config;
mod io;
mod lexer;
mod parser;
mod types;
mod utils;

use clap::{Parser, command};
use io::read_file;
use std::error::Error;
use std::sync::OnceLock;

use crate::config::{Config, init_config};
use crate::lexer::tokenize;
use crate::parser::{group_lines_to_blocks, parse_blocks};
use crate::types::Token;

static CONFIG: OnceLock<Config> = OnceLock::new();

#[derive(Parser, Debug)]
#[command(
    author = "Zackary Liel",
    version = "0.1.0",
    about = "A Commonmark compliant markdown parser and static site generator.",
    override_usage = "rust_mark [OPTIONS] <FILE_PATH>"
)]
struct Cli {
    #[arg(value_name = "FILE_PATH")]
    file_path: String,
    #[arg(short, long, default_value = "config.toml")]
    config: String,
    #[arg(short, long, default_value = "./output")]
    output_dir: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let file_path = &cli.file_path;
    let config_path = cli.config;

    // Setup
    init_config(&config_path)?;
    let file_contents = read_file(file_path)?;

    // Tokenizing
    let mut tokenized_lines: Vec<Vec<Token>> = Vec::new();
    for line in file_contents.split('\n') {
        tokenized_lines.push(tokenize(line));
    }

    // Parsing
    let blocks = group_lines_to_blocks(tokenized_lines);

    let parsed_elements = parse_blocks(blocks);
    parsed_elements
        .iter()
        .for_each(|block| println!("{:?}", block));

    Ok(())
}
