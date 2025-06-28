use crate::schemes::Configuration;

pub trait Parsable {
    fn parse() -> Self;
}

#[derive(Debug)]
pub struct Config {
    repository: Configuration,
}

impl Config {
    pub fn new() -> Self {
        todo!()
    }
}
