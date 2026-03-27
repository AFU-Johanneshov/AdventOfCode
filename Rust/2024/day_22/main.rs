use std::{collections::HashMap, io};

mod operations;
mod secret_number;
use secret_number::SecretNumber;
mod circular_stack;
use circular_stack::CircularStack;

mod reader;
use reader::get_lines;
mod testing_debug;

#[derive(Debug)]
enum AdventError {
    IoError(io::Error),
    CorruptedData(String),
}

impl From<io::Error> for AdventError {
    fn from(err: io::Error) -> Self {
        Self::IoError(err)
    }
}

fn get_sequences(number: i64) -> HashMap<[i8; 4], u64> {
    // We set the default to 16 so we don't need to make logic to only add the stack to the
    // sequence_combinations after the first 3 prices. A pattern containing 16 will never occur
    // due to the max difference in price being 9/-9
    let mut circular_stack: CircularStack<i8> = CircularStack::with_default(16);
    let mut local_sequence_combinations: HashMap<[i8; 4], u64> = HashMap::new();
    let mut secret_number = SecretNumber::new(number);
    let mut previous_price = secret_number.price();
    for _ in 0..2000 {
        secret_number = secret_number.next();
        circular_stack.push(secret_number.price() - previous_price);
        previous_price = secret_number.price();

        local_sequence_combinations
            .entry(circular_stack.get_queue())
            .or_insert_with(|| secret_number.price() as u64);
    }

    local_sequence_combinations
}

fn get_best_sequence_value(sequence_combinations: HashMap<[i8; 4], u64>) -> u64 {
    let mut result_sum = 0;
    for value in sequence_combinations.values() {
        if *value > result_sum {
            result_sum = *value;
        }
    }
    result_sum
}

fn calculate(path: &str) -> Result<u64, AdventError> {
    let mut sequence_combinations: HashMap<[i8; 4], u64> = HashMap::new();
    for line in get_lines(path)? {
        let Ok(number) = line.parse() else {
            return Err(AdventError::CorruptedData(line));
        };

        let local_sequence_combinations = get_sequences(number);
        for (key, value) in local_sequence_combinations.iter() {
            *sequence_combinations.entry(*key).or_default() += *value;
        }
    }

    Ok(get_best_sequence_value(sequence_combinations))
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
        Ok(value) => assert_eq!(value, 23),
    }
}

/* Sudo code:

Challenge part 1:

// Initial thought are that the problem seems to be to create a function which takes in a secret
// number and then outputs the next one.
// It sounds like the only input that matters to get the next number is the current secret number,
// so whatever I end up with should scale linearly with the amount of numbers to calculate.
// Although I feel like that might be a trap which will come back to haunt me in part 2. We shall
// see.


Challenge part 2:

*/
