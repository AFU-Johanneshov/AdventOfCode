use std::{fmt::Display, io};

mod key;
use key::{Key, KeyBuilder, KeyBuilderError};

mod reader;
use reader::get_lines;
mod testing_debug;

enum AdventError {
    Io(io::Error),
    KeyBuilder(KeyBuilderError),
    DataFileLength(String),
    DataFileFormat(String),
}

impl Display for AdventError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(err) => write!(f, "AdventError:\n{}", err),
            Self::KeyBuilder(err) => write!(f, "AdventError:\n{}", err),
            Self::DataFileLength(err) => write!(f, "AdventError:\n{}", err),
            Self::DataFileFormat(err) => write!(f, "AdventError:\n{}", err),
        }
    }
}

impl From<io::Error> for AdventError {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<KeyBuilderError> for AdventError {
    fn from(err: KeyBuilderError) -> Self {
        Self::KeyBuilder(err)
    }
}

fn extract_line(source: Option<(usize, String)>) -> Result<(usize, String), AdventError> {
    match source {
        Some(value) => Ok(value),
        None => Err(AdventError::DataFileLength("Unexpected data file length! Check that the data file follows the correct format as specified on the website.".to_string()))
    }
}

fn get_keys(path: &str) -> Result<(Vec<Key>, Vec<Key>), AdventError> {
    let mut lines = get_lines(path)?.enumerate();
    let mut keys = Vec::new();
    let mut locks = Vec::new();

    while let Some(_) = lines.next() {
        let mut key_builder = KeyBuilder::new();
        for _ in 0..5 {
            let (_, line) = extract_line(lines.next())?;
            key_builder.add_line(&line)?;
        }
        let (identifier_line_nr, identifier_line) = extract_line(lines.next())?;

        let Some(identifier) = identifier_line.chars().next() else {
            return Err(AdventError::DataFileFormat(format!(
                "Identifier line at row {} was empty!",
                identifier_line_nr
            )));
        };

        match identifier {
            '.' => locks.push(key_builder.assemble()),
            '#' => keys.push(key_builder.assemble()),
            _ => {
                return Err(AdventError::DataFileFormat(format!(
                    "Unexpected idenfier: '{}'\nExpected: '#' or '.'",
                    identifier
                )))
            }
        }
        lines.next();
    }

    Ok((keys, locks))
}

fn calculate(path: &str) -> Result<u64, AdventError> {
    let (keys, locks) = get_keys(path)?;

    let mut result = 0;
    for key in keys {
        for lock in &locks {
            if !key.overlaps(lock) {
                result += 1;
            }
        }
    }

    Ok(result)
}

fn main() {
    match calculate("data.txt") {
        Err(err) => println!("An error occured: {err}"),
        Ok(value) => println!("Result is: {}", value),
    }
}

#[test]
fn calculate_test() {
    match calculate("testdata.txt") {
        Err(err) => panic!("An error occured: {err}"),
        Ok(value) => assert_eq!(value, 3),
    }
}

/* Sudo code:

Challenge part 1:

The keys and locks can be represented as base 5 numbers.
// This would grant us a easy way to see if a key and lock is a perfect match.
// But here we also care about keys where the pins doesn't overlap but are shorter than needed to
// perfectly match.
//
// a perfect key match would result in combined id of 7775

Challenge part 2:



*/
