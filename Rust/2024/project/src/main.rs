use std::io;

mod operations;
use operations::Operation;
use operations::OperationResult;

mod reader;
use reader::get_lines;
mod testing_debug;

#[derive(Debug)]
enum AdventError {
    IoError(io::Error),
}

impl From<io::Error> for AdventError {
    fn from(err: io::Error) -> Self {
        Self::IoError(err)
    }
}

fn get_...(path: &str) -> Result<..., AdventError> {
    let lines = get_lines(path)?;

    todo!()
}

fn calculate(path: &str) -> Result<..., AdventError> {
    let ... = get_...(path)?;

    todo!()
}

fn main() {
    match calculate("data.txt") {
        Err(err) => println!("An error occured: {err:?}"),
        Ok(value) => println!("Result is: {}", value),
    }
}

#[test]
fn calculate_test() {
    match calculate("testdata.txt") {
        Err(err) => panic!("An error occured: {err:?}"),
        Ok(value) => assert_eq!(value, 0),
    }
}

/* Sudo code:

Challenge part 1:



Challenge part 2:



*/
