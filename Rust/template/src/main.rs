use std::error::Error;

mod reader;
use reader::get_lines;
mod data_parser;
mod operations;
use data_parser::{DataParser, ParserRules};

fn load_data(path: &str) -> Result<(), Box<dyn Error>> {
    let lines = reader::get_lines(path)?;

    DataParser::new(ParserRules {}, Box::new(lines));
    todo!();
}

fn calculate(data_path: &str) -> Result<u64, Box<dyn Error>> {
    let data = load_data(data_path)?;
    todo!();
}

fn main() {
    match calculate("data.txt") {
        Ok(value) => println!("Result:\n{}", value),
        Err(err) => println!("Error occured:\n{}", err),
    }
}

#[test]
fn calculate_test() {
    let expected_value = 0;
    match calculate("tesdata.txt") {
        Ok(value) => assert_eq!(
            value, expected_value,
            "Program finished but result was wrong! Expected: {} but received: {}",
            expected_value, value
        ),
        Err(err) => panic!("Error occured:\n{}", err),
    }
}
