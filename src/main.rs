#[cfg(test)]
mod tests;
mod generator;
mod parser;
mod structures;
mod opts;

use structopt::StructOpt;
use std::fs::read_to_string;

fn main() {
    let opt = opts::Opt::from_args();

    let casefile = read_to_string(opt.filepath).unwrap();

    let case = parser::parse_text(&casefile);

    
}