use std::{path::Path, fs::File, error::Error, io::{BufReader, BufRead}, fmt::Display};

pub mod util_imports {
    pub use std::str::FromStr;
    pub use crate::util::{Parser, parse_input_file, ParseError};
    #[cfg(test)]
    pub use crate::util::parse_input_string;
}

pub trait Parser where Self: Sized {
    fn parse_item<'a, I: Iterator<Item = String>>(line_iterator: &mut I) -> Result<Self, Box<dyn Error>>;
}

#[cfg(test)]
pub fn parse_input_string<I: Parser>(input_string: String) -> Result<Vec<I>, Box<dyn Error>> {
    let line_iterator = input_string.lines().map(|s| s.to_string());

    let result = parse_lines(line_iterator)?;

    Ok(result)
}

pub fn parse_input_file<I: Parser>(input_file_path: &str) -> Result<Vec<I>, Box<dyn Error>> {
    let path = Path::new(input_file_path);

    let file = File::open(&path)?;
    let buf_reader = BufReader::new(file);
    let buf_reader_lines = buf_reader.lines()
        .map(|x| x.unwrap());

    let result = parse_lines(buf_reader_lines)?;

    Ok(result)
}

pub fn parse_lines<'a, L, I>(lines: L) -> Result<Vec<I>, Box<dyn Error>> 
    where
        L: IntoIterator<Item = String>,
        I: Parser,
{
    let mut vec = Vec::new();

    let mut peekable_iterator = lines.into_iter().peekable();

    while peekable_iterator.peek().is_some() {
        let item_result = I::parse_item(&mut peekable_iterator);
        match item_result {
            Ok(item) => {vec.push(item)},
            Err(error) => return Err(error.into()),
        }
    }

    Ok(vec)
}

#[derive(Debug)]
pub struct ParseError {
    reason: &'static str,
}

impl ParseError {
    pub fn new(reason: &'static str) -> Self {
        Self { reason }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.reason)
    }
}

impl Error for ParseError {}