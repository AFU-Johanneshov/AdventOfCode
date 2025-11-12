use std::io;

mod operations;
use operations::Operation;
use operations::OperationResult;

mod reader;
use reader::get_lines;
mod testing_debug;
mod vector;
use vector::VectorF64;
use vector::VectorI16;
use vector::VectorI64;

#[derive(Debug)]
enum AdventError {
    IoError(io::Error),
}

impl From<io::Error> for AdventError {
    fn from(err: io::Error) -> Self {
        Self::IoError(err)
    }
}

#[derive(Debug)]
struct ClawMachine {
    button_a: VectorI64,
    button_b: VectorI64,
    prize: VectorI64,
}

impl From<&mut [i64; 6]> for ClawMachine {
    fn from(values: &mut [i64; 6]) -> Self {
        ClawMachine {
            button_a: VectorI64 {
                x: values[0],
                y: values[1],
            },
            button_b: VectorI64 {
                x: values[2],
                y: values[3],
            },
            prize: VectorI64 {
                x: values[4] + 10000000000000,
                y: values[5] + 10000000000000,
            },
        }
    }
}

impl ClawMachine {
    fn calculate_cost(self) -> Option<u64> {
        testonly_println!(
            "ClawMachine: \nButton A: {:?}\nButton B: {:?}\nPrize: {:?}",
            self.button_a,
            self.button_b,
            self.prize
        );

        let b_inverted = self.button_b * -1;

        let (button_a_pressed, button_b_pressed) = VectorF64::line_line_intersection_get_scalars(
            VectorF64 { x: 0.0, y: 0.0 },
            VectorF64::from(self.button_a),
            VectorF64::from(self.prize),
            VectorF64::from(b_inverted),
        );

        //testonly_println!("Result: {:?}\nWhole number: {:?} {:?}", result, a, b);

        if button_a_pressed % 1.0 != 0.0 || button_b_pressed % 1.0 != 0.0 {
            return None;
        }

        Some(button_a_pressed as u64 * 3 + button_b_pressed as u64)
    }
}
/* Sudo code:

Challenge part 1:
I felt like the best way to solve this is to imagine two lines. One going from 0.0 in the direction of the vector of button A.
And one going from the pri<e location in the direction of the vector of button B * -1.
Then we need to figure out where on the coordinate system the two lines intersect.
While it would have been possible to brute force finding that point I felt it would be interesting to see how it could be solved with math instead.

The type of problem to solve then is what is sometimes known as a line-line intersection, or a ray-ray intersection.

Once the intersection point is found we check the following conditions:
1: Is the point somewhere between 0.0 and the prize location? If not then no price can be won.
2: Is the point divided by the vector of button A a whole number with no decimals? If not then the prize can't be won..
2: Is the difference between the prize location and point divided by the vector of button B a whole number with no decimals? If not then the prize can't be won.

If all 3 pass:
multiply the result of condition 2 with 3
add the above result together with the result of condition 3.

Then you have the cost to win that machine.



Challenge part 2:



*/

const CACHESIZE: usize = 6;

fn handle_operation_result(
    operation_result: Option<OperationResult>,
    number_cache: &mut [i64; CACHESIZE],
    index: &mut usize,
) -> Option<ClawMachine> {
    let Some(result) = operation_result else {
        return None;
    };

    match result {
        OperationResult::Number(number) => {
            number_cache[*index] = number as i64;
            *index += 1;

            if *index >= CACHESIZE {
                *index = 0;
                return Some(ClawMachine::from(number_cache));
            }
        }
    }
    None
}

fn calculate(path: &str) -> Result<u64, AdventError> {
    //let machines = get_machines(path)?;

    let mut number_cache: [i64; CACHESIZE] = [0; CACHESIZE];
    let mut index: usize = 0;

    let lines = get_lines(path)?;
    let mut state: Operation = Operation::None;

    let mut tokens: u64 = 0;

    for line in lines {
        for char in line.chars() {
            let (new_state, potential_result) = state.next(char);
            state = new_state;

            let Some(claw_machine) =
                handle_operation_result(potential_result, &mut number_cache, &mut index)
            else {
                continue;
            };

            if let Some(cost) = claw_machine.calculate_cost() {
                tokens += cost;
            }
        }
    }

    let (_, potential_result) = state.next(' ');
    if let Some(claw_machine) =
        handle_operation_result(potential_result, &mut number_cache, &mut index)
    {
        if let Some(cost) = claw_machine.calculate_cost() {
            tokens += cost;
        }
    }

    Ok(tokens)
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
        Ok(value) => assert_eq!(value, 480),
    }
}

#[test]
fn calculate_test_small() {
    match calculate("smalltestdata.txt") {
        Err(err) => panic!("An error occured: {err:?}"),
        Ok(value) => assert_eq!(value, 280),
    }
}

/* Sudo code:

Challenge part 1:
I felt like the best way to solve this is to imagine two lines. One going from 0.0 in the direction of the vector of button A.
And one going from the pri<e location in the direction of the vector of button B * -1.
Then we need to figure out where on the coordinate system the two lines intersect.
While it would have been possible to brute force finding that point I felt it would be interesting to see how it could be solved with math instead.

The type of problem to solve then is what is sometimes known as a line-line intersection, or a ray-ray intersection.

Once the intersection point is found we check the following conditions:
1: Is the point somewhere between 0.0 and the prize location? If not then no price can be won.
2: Is the point divided by the vector of button A a whole number with no decimals? If not then the prize can't be won..
2: Is the difference between the prize location and point divided by the vector of button B a whole number with no decimals? If not then the prize can't be won.

If all 3 pass:
multiply the result of condition 2 with 3
add the above result together with the result of condition 3.

Then you have the cost to win that machine.



Challenge part 2:



*/
