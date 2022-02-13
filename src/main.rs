mod parser;
mod structures;
#[cfg(test)]
mod tests;

fn main() {
    println!("{:?}", parser::parse_variable("(0,1000)"));
}