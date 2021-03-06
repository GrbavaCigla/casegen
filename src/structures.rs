#[derive(Debug)]
pub enum Entity<'a> {
    Text(&'a str),
    Variable(Variable<'a>),
}

#[derive(Debug, PartialEq)]
pub struct Variable<'a> {
    pub name: Option<&'a str>,
    pub lower: i32,
    pub upper: i32,
}
