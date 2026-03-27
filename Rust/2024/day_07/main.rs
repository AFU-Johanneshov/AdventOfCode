mod operations;
use operations::Operation;
use operations::OperationResult;

mod reader;
use reader::get_lines;

#[derive(Debug)]
struct Equation {
    target: u64,
    numbers: [u32; 12],
    numbers_length: usize,
}

impl Equation {
    fn is_true(&self) -> bool {
        self.calculate(0, 0)
    }

    fn calculate(&self, depth: usize, result: u64) -> bool {
        if depth >= self.numbers_length {
            return result == self.target;
        }

        if self.calculate(depth + 1, result + self.numbers[depth] as u64) {
            return true;
        }
        if self.calculate(depth + 1, result * self.numbers[depth] as u64) {
            return true;
        }
        if self.calculate(
            depth + 1,
            // Big thanks to deepkrg17 on reddit for this code piece. Link below.
            result * 10u64.pow(self.numbers[depth].ilog10() + 1) + self.numbers[depth] as u64,
            // https://www.reddit.com/r/rust/comments/191l3ot/concatinate_two_numbers/
            // How does this even work? The above acts like the two numbers are strings, adds them
            // together to one string, then parses a u64 from it. But it does it with math instead.
            // Crazy stuff!
        ) {
            return true;
        }
        false
    }
}

fn calculate(path: &str) -> u64 {
    let Ok(lines) = get_lines(path) else {
        panic!("Data file could not be read!");
    };

    let mut operation_status: Operation = Operation::None;
    let mut total: u64 = 0;

    for line in lines {
        let mut equation: Equation = Equation {
            target: 0,
            numbers: [0; 12],
            numbers_length: 0,
        };

        for char in line.chars() {
            let (next_op, output) = operation_status.next(char);

            operation_status = next_op;

            let Some(result) = output else {
                continue;
            };

            match result {
                OperationResult::TestValue(value) => {
                    equation.target = value;
                }
                OperationResult::Number(number) => {
                    equation.numbers[equation.numbers_length] = number;
                    equation.numbers_length += 1;
                }
            };
        }

        match operation_status {
            Operation::Value(str) => {
                let result = str.parse::<u32>().unwrap();
                equation.numbers[equation.numbers_length] = result;
                equation.numbers_length += 1;
            }
            Operation::None => {}
        }

        operation_status = Operation::None;

        print!("{:?} is: ", equation);

        if equation.is_true() {
            total += equation.target;
            println!("True");
        } else {
            println!("False");
        }
    }

    total
}

fn main() {
    // Start here:
    println!("Total is: {}", calculate("./data.txt"));
}

#[test]
fn calculate_test() {
    assert_eq!(calculate("./data2.txt"), 11387);
}

/* Sudo code:

Challenge part 1:
Struct equation
{
    testvalue: u32
    numbers: [u32; 12]
    numbers_length: usize
}

Read file line by line
for each line
    create a equation struct based on the line.


Core plan:
mutliply or add all number together.
Then from the right to the left shift the operators one by one like binary counting. If 0 is add and 1 is multiply:
5 3 12
5+3+12 = 00
5+3*12 = 01
5*3+12 = 10
5*3*12 = 11

Inefficient as hell but should work.


Challenge part 2:



*/
