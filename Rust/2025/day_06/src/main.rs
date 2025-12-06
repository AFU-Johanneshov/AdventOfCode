use std::error::Error;

mod data_parser;
mod operations;
mod reader;

/*
Part One:

This is an interesting task. We need to read and then perform multiplication/addition with 3 values.
It would be "easy" to do if each group of 3 values and operator was on separate rows. But here the
first value is on the same row as all other groups first values. The second is on the same row as
all other second values. And so on.

So we need to read the data one column at a time instead of row.
I think the quickes solution is to read the data and collect it into groups first, instead of trying
to process only one column at a time.
This should be quite easy using string.split(" ") and filter.



Part Two:



*/

enum Operator {
    Add(u64),
    Mul(u64),
}

impl Operator {
    fn parse(data_string: &str) -> Result<Operator, Box<dyn Error>> {
        match data_string {
            "+" => Ok(Operator::Add(0)),
            "*" => Ok(Operator::Mul(1)), // 1 since the first combine would otherwise multiply by 0
            _ => Err(format!("Could not parse operator from: [{}]", data_string).into()),
        }
    }

    fn combine(&mut self, other_value: &u16) {
        match self {
            Self::Add(value) => *value += *other_value as u64,
            Self::Mul(value) => *value *= *other_value as u64,
        }
    }

    fn value(&self) -> u64 {
        match self {
            Self::Add(value) => *value,
            Self::Mul(value) => *value,
        }
    }
}

fn calculate(path: &str, value_lines_override: usize) -> Result<u64, Box<dyn Error>> {
    let mut lines = reader::get_lines(path)?;

    let mut value_lines: [Vec<u16>; 4] = [const { Vec::new() }; 4];
    for i in 0..value_lines_override {
        for value_string in lines
            .next()
            .ok_or("Datafile value lines does not match the provided value_lines_override value.")?
            .split(" ")
            .filter(|s| !s.is_empty())
        {
            value_lines[i].push(value_string.parse()?);
        }
    }

    let mut operators: Vec<Operator> = Vec::new();
    for operator_string in lines
        .next()
        .ok_or("There should always be a operators line after the values.")?
        .split(" ")
        .filter(|s| !s.is_empty())
    {
        operators.push(Operator::parse(operator_string)?);
    }

    for i in 0..value_lines_override {
        if value_lines[i].len() != operators.len() {
            return Err(
                "Data file contains lines with different amount of columns! Aborting...".into(),
            );
        }
    }

    println!("Len: {}", operators.len());

    let mut sum: u64 = 0;
    for (i, operator) in operators.iter_mut().enumerate() {
        for y in 0..value_lines_override {
            let other_value = &value_lines[y][i];
            print!(" {other_value} ");
            operator.combine(other_value);
        }
        let value = operator.value();
        println!("Value: {value}");
        sum += operator.value();
    }

    Ok(sum)
}

fn main() {
    match calculate("data.txt", 4) {
        Ok(value) => println!("Result:\n{}", value),
        Err(err) => println!("Error occured:\n{}", err),
    }
}

#[test]
fn calculate_test() {
    let expected_value = 4277556;
    match calculate("testdata.txt", 3) {
        Ok(value) => assert_eq!(
            value, expected_value,
            "Program using testdata.txt finished but result was wrong! Expected: {} but received: {}",
            expected_value, value
        ),
        Err(err) => panic!("Error occured:\n{}", err),
    }
}

/*
#[test]
fn calculate_small_test() {
    let expected_value = 0;
    match calculate("smalltestdata.txt") {
        Ok(value) => assert_eq!(
            value, expected_value,
            "Program using smalltestdata.txt finished but result was wrong! Expected: {} but received: {}",
            expected_value, value
        ),
        Err(err) => panic!("Error occured:\n{}", err),
    }
} // */
