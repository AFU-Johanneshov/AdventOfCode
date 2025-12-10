mod reader;

#[allow(dead_code)]
const PART_ONE_EXPECTED_TEST_VALUE: u64 = 50;
#[allow(dead_code)]
const PART_TWO_EXPECTED_TEST_VALUE: u64 = 24;

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

This time not all rectangles are valid.

The rules are the following:
A rectangle is only valid if all tiles it contain is either part of the coordinates list, OR are
"inside" the coordinate list area.
Imagine you draw a line from point 1 to point 2, then point 2 to 3, 3 to 4 and so on until the
last one. Then finally draw a line from the last point to the first.
Anything inside of this drawn shape is valid. Meaning no part of the rectangle may exist outside
the drawn shape.

This should be solvable using math and without having to actually create a grid to manually check.

Could it be as simple as to just ensure no other point is inside the area between two points?
Not including the edge layer though.
Actually more is needed. In the example a rectangle between point 2,3 and 11,1 would have point
7,3 be in the edge layer of the rectangle, but on the wrong side of the rectangle.

If we can determine which side of a point is "outside" and which is "inside" then we could add a
rule that points may exist on the edge of the area IF that points outside is outside the area.
We should also ensure that the area between the two points are always in the "inside" of both
points.

If we assume the drawn line never crosses itself then we should be able to tell which side is
"outside" from a point based on the direction to the next point. If the line is drawn clockwise
then the left side of the direction is outside and right is inside.
So if we add an additional d_x and d_y (x,y direction from the current to the next point) to the
points data we should be able to tell if a area is valid or not.

In the following example we assume the line is drawn clockwise, meaning the left side of any
direction is the outside.
If we modify the Part One code:

In addition to the two points being checked in the loop we should also save the previous two
points direction. Meaning we set the last_direction to the current points direction at the end of
that loop iteration.
This gives us easy access what is outside and inside of the points.

So we should modify the code and add the following if an Area larger than the largest
is found:
Only save the new largest area if:
The outside of the old_p1 direction is outside the area
The outside of the new_p1 direction is outside the area
The outside of the old_p2 direction is outside the area
The outside of the new_p2 direction is outside the area
The outside of any other points in the area is outside the area.
IF any of these are false then the area is invalid.

*/
mod part_two {
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
        let mut points = Vec::new();
        for line in reader::get_lines(data_path)? {
            points.push(get_point(&line)?);
        }

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
