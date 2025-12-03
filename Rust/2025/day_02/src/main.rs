use std::error::Error;

mod data_parser;
mod operations;
mod reader;

/*
Part One:

First step here is to read the string and convert it to a sequnce of number pairs.
Once that is done we can process each number in the range of the pair.

Processing could work like this:
Convert the number into a string.
Split the string in half. If both parts are the same then the ID is invalid.



Part Two:

Part two adds a quite annoying new requirement. Any id with a repeating pattern is invalid as long as it repeats at least once.
The first glance solution I can think of is to check each combination, which will be very slow.

Which would mean checking each pattern from the first digit to half of all digits.
So basically:
Take the first digit and iterate through the string. If at any point a different digit is found then move on to the next.
Take the first two digits and iterate through the string two steps at a time. If at any point the two digits checked doesn't
    match the first then continue to the next.
Repeat this up to the string length/2.
If any iterator completes without a mismatch then the id is invalid.

Unfinished random ideas.
A major point to optimise would be to ignore sequence lengths that would never occur in the range.
Check how similar the lower and upper values are left to right.
    235 - 278 only match with the first digit [2].
    53437 - 53495 match with the first three digits [534]
This can be used to skip checking some patterns. With the second range with the shared first digits of 534 we can figure out that:
- The 1 digit pattern is impossible due to the second digit [3] not matching the first [5].
- The 2 digit pattern is impossible due to the third digit [4] not matching the first [5].



*/

struct IDRange {
    lower: u64,
    upper: u64,
}

impl IDRange {
    fn parse(data_string: &str) -> Result<IDRange, Box<dyn Error>> {
        let mut parts = data_string.split('-');
        let lower = parts.next();
        let upper = parts.next();
        let (Some(lower), Some(upper)) = (lower, upper) else {
            return Err(format!(
                "Could not parse IDRange from data string: [{}]!",
                data_string
            )
            .into());
        };
        Ok(IDRange {
            lower: lower.parse()?,
            upper: upper.parse()?,
        })
    }

    fn invalid_id_sum(&self) -> u64 {
        let mut id_sum = 0;

        for id in self.lower..=self.upper {
            let digit_count = id.ilog10() + 1;
            if Self::is_invalid(&id.to_string(), digit_count as usize) {
                id_sum += id;
            }
        }

        id_sum
    }

    fn is_invalid(id_string: &str, digit_count: usize) -> bool {
        for pattern_len in 1..=digit_count / 2 {
            let pattern = &id_string[0..pattern_len];
            let mut i = pattern_len;
            let mut invalid = true;
            while i < digit_count {
                if i + pattern_len > digit_count || pattern != &id_string[i..i + pattern_len] {
                    invalid = false;
                    break;
                }
                i += pattern_len;
            }
            if invalid {
                return true;
            }
        }
        false
    }
}

fn calculate(data_path: &str) -> Result<u64, Box<dyn Error>> {
    // get_lines returns an iterator over the lines of the file. next() attempts to return the
    // first line, which we then ensure is there with expect().
    let line = reader::get_lines(data_path)?
        .next()
        .expect("The data files for this challenge always only contain 1 line.");

    let mut id_sum = 0;
    for data_string in line.split(',') {
        let id_range = IDRange::parse(data_string)?;
        id_sum += id_range.invalid_id_sum();
    }

    Ok(id_sum)
}

fn main() {
    match calculate("data.txt") {
        Ok(value) => println!("Result:\n{}", value),
        Err(err) => println!("Error occured:\n{}", err),
    }
}

#[test]
fn calculate_test() {
    let expected_value = 4174379265;
    match calculate("testdata.txt") {
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
