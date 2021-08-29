extern crate pest;
#[macro_use]
extern crate pest_derive;

mod parser;
use clap::{App, Arg};
use color_eyre::eyre::Result;
use std::{env::current_dir, fs};

use parser::parse;

fn main() -> Result<()> {
    color_eyre::install()?;

    let matches = App::new("yaml-peg")
        .version("0.1.0")
        .author("aofdev <aof.dev@gmail.com>")
        .about("PEG parser for YAML written in Rust")
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .takes_value(true)
                .help("A YAML file"),
        )
        .get_matches();

    let file = matches
        .value_of("file")
        .unwrap_or("example_files/test.yaml");

    let dir = current_dir().unwrap().join(file);
    let filepath = dir.to_str().unwrap();
    let unparsed_file =
        fs::read_to_string(filepath).unwrap_or_else(|_| panic!("Can't read file: {}", filepath));

    match parse(&unparsed_file) {
        Ok(parsed_file) => println!("{:?}", parsed_file),
        Err(e) => eprintln!("{}", e),
    }

    Ok(())
}
