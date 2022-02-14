#[cfg(test)]
mod tests;
mod generator;
mod parser;
mod structures;
mod opts;

use structopt::StructOpt;
use std::fs::read_to_string;
use generator::generate_case;

fn main() {
    let opt = opts::Opt::from_args();

    let casefile = read_to_string(opt.filepath).unwrap();

    let case = parser::parse_text(&casefile).unwrap().1;

    println!("{}", generate_case(&case));
}