use std::{path::Path, fs::File, error::Error, io::{BufReader, BufRead}};


pub trait Parser where Self: Sized {
    fn parse_item<'a, I: Iterator<Item = String>>(line_iterator: &mut I) -> Result<Self, Box<dyn Error>>;
}

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