mod reader;

#[allow(dead_code)]
const PART_ONE_EXPECTED_TEST_VALUE: u64 = 50;
#[allow(dead_code)]
const PART_TWO_EXPECTED_TEST_VALUE: u64 = 0;

//

//

/*
Part One
##################################################################################################

We have a list of 2D coordinates. In part one the task is to figure out which two points form a
rectangle of the largest area. The quick and easy solution would be to loop through the
coordinates, comparing each to any coming after it. If the area between the two is the largest yet
then save it, otherwise ignore.

When the loop has finished the saved area should be the largest possible.
*/
mod part_one {
    use crate::reader;
    use std::error::Error;

    fn get_point(data: &str) -> Result<(i64, i64), Box<dyn Error>> {
        let mut parts = data.split(',');
        Ok((
            parts.next().ok_or("E1: Corrupted input data!")?.parse()?,
            parts.next().ok_or("E2: Corrupted input data!")?.parse()?,
        ))
    }

    pub fn calculate(data_path: &str) -> Result<u64, Box<dyn Error>> {
        let (mut points, mut largest_area) = (Vec::new(), 0);
        for line in reader::get_lines(data_path)? {
            points.push(get_point(&line)?);
        }

        for index in 0..points.len() {
            let (p1_x, p1_y) = points[index];
            for (p2_x, p2_y) in points.iter().skip(index + 1) {
                let area = (((p2_x - p1_x).abs() + 1) * ((p2_y - p1_y).abs() + 1)) as u64;
                if area > largest_area {
                    largest_area = area;
                }
            }
        }

        Ok(largest_area)
    }
}

//

//

/*
Part Two
##################################################################################################

*/
mod part_two {
    use crate::reader;
    use std::error::Error;

    pub fn calculate(data_path: &str) -> Result<u64, Box<dyn Error>> {
        let lines = reader::get_lines(data_path)?;

        Err("NotImplemented: This problem has not been solved yet!".into())
    }
}

//

//

// Default controller code. Is the same between projects.
// ###############################################################################################

fn main() {
    println!("Running Program...");
    print!("\nPart One ");
    match part_one::calculate("data.txt") {
        Ok(value) => println!("Result:\n{}", value),
        Err(err) => println!("FAILED with error:\n{}", err),
    }
    print!("\nPart Two ");
    match part_two::calculate("data.txt") {
        Ok(value) => println!("Result:\n{}", value),
        Err(err) => println!("FAILED with error:\n{}", err),
    }
    println!();
}

#[test]
fn calculate_part_one_test() {
    let expected_value = PART_ONE_EXPECTED_TEST_VALUE;
    match part_one::calculate("testdata.txt") {
        Ok(value) => assert_eq!(
            value, expected_value,
            "Part One calculation completed successfully but the result was wrong! Expected: {} but received: {}",
            expected_value, value
        ),
        Err(err) => panic!("Part One failed with error:\n{}\n", err),
    }
}

#[test]
fn calculate_part_two_test() {
    let expected_value = PART_TWO_EXPECTED_TEST_VALUE;
    match part_two::calculate("testdata.txt") {
        Ok(value) => assert_eq!(
            value, expected_value,
            "Part Two calculation completed successfully but the result was wrong! Expected: {} but received: {}",
            expected_value, value
        ),
        Err(err) => panic!("Part Two failed with error:\n{}\n", err),
    }
}
