use std::io;

mod operations;
use operations::Operation;
use operations::OperationResult;

mod chronospatial_computer;
use chronospatial_computer::ChronoSpatialComputer;
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

fn save_value(file_data: &mut Vec<i64>, operation_result: OperationResult) {
    if let OperationResult::Integer(value) = operation_result {
        file_data.push(value);
    }
}

fn read_data(path: &str) -> Result<(ChronoSpatialComputer, Vec<u8>), AdventError> {
    let lines = get_lines(path)?;

    let mut file_data: Vec<i64> = Vec::new();
    let mut operation: Operation = Operation::None;
    for line in lines {
        for char in line.chars() {
            let (next_op, potential_result) = operation.next(char);
            operation = next_op;

            let Some(operation_result) = potential_result else {
                continue;
            };

            save_value(&mut file_data, operation_result);
        }

        let (next_op, operation_result) = operation.collect_operation();
        operation = next_op;

        save_value(&mut file_data, operation_result);
    }

    if file_data.len() < 4 {
        panic!(
            "Data file read didn't find enough values for the computer! {}\n{:?}",
            file_data.len(),
            file_data
        );
    }

    Ok((
        ChronoSpatialComputer::new(file_data[0], file_data[1], file_data[2]),
        file_data.iter().skip(3).map(|value| *value as u8).collect(),
    ))
}

fn assemble_result(result_vec: Vec<u8>) -> String {
    let mut output_string = String::with_capacity(result_vec.len() * 2 - 1); // * 2 to compensate for
                                                                             // the , inserted between
                                                                             // numbers.
    for value in result_vec.iter().take(result_vec.len() - 1) {
        output_string.push_str(&value.to_string());
        output_string.push(',');
    }
    output_string.push_str(&result_vec[result_vec.len() - 1].to_string());

    output_string
}

fn program_contains(program: &Vec<u8>, result: &Vec<u8>) -> bool {
    let diff = program.len() - result.len();
    for (index, value) in result.iter().enumerate() {
        if program[index + diff] != *value {
            return false;
        }
    }

    true
}

fn calculate(path: &str) -> Result<i64, AdventError> {
    println!("Working...  ");
    let (mut computer, program) = read_data(path)?;

    let mut a_value = 0;
    computer.program(program.clone());
    for i in (0..program.len()).rev() {
        computer.regit('A', a_value);
        let mut a_cache = 0;
        loop {
            computer.reset_program();
            computer.regit('A', a_cache + (a_value << 3));
            let result = computer.run_program();
            if !result.is_empty() && result[0] == program[i] && program_contains(&program, &result)
            {
                break;
            }
            a_cache += 1;
        }
        a_value = a_cache + (a_value << 3);
    }
    println!("DONE!");

    confirm_result(&mut computer, &program, a_value);

    Ok(a_value)
}

fn confirm_result(computer: &mut ChronoSpatialComputer, expected_result: &Vec<u8>, a_value: i64) {
    computer.reset_program();
    computer.regit('A', a_value);
    let result = computer.run_program();
    if !program_contains(expected_result, &result) {
        panic!(
            "Result did not match program!\np: {}\nr: {}",
            assemble_result(expected_result.to_vec()),
            assemble_result(result)
        );
    }
}

fn view_bits(value: i64) {
    let mut i = 2;
    for n in (0..64).rev() {
        print!("{}", (value >> n) & 1);
        i += 1;
        if i >= 3 {
            i = 0;
            print!(" ");
        }
    }
    println!(" = {}", value);
}

fn main() {
    match calculate("data.txt") {
        Err(err) => println!("An error occured: {err:?}"),
        Ok(value) => println!("Result is: \n{}", value),
    }
}

#[test]
fn calculate_test() {
    match calculate("testdata.txt") {
        Err(err) => panic!("An error occured: {err:?}"),
        Ok(value) => assert_eq!(value, 117440),
    }
}

/* Sudo code:

Challenge part 1:

// Chronospatial computer.
// I'm thinking a struct for the cronospatial computer.
// A: i64,
// B: i64,
// C: i64,
//
//
// Instructions:
// 0: adv:

Result: 1,7,6,5,1,0,5,0,7

Challenge part 2:

So part two of the challenge is kind of reversed.
We no longer try to figure out the program output, but rather use the program output to calculate the original register values.
The question is, how.
We do not know what the registers are at the end, only that the result is supposed to be a clone of the program.
My first thought was to simply calculate everything backwards, but that would require we know the resulting register values.

I think what needs to be done here is to analyze the interactions of the instructions. How does each impact the others?
Start simple, see if we can figure out which values of A that results in a output of the first value of the program. In our case 1.

Note that we need to find the LOWEST possible value, meaining that there are more than one potential solution.

// After getting the test case to work I was hopeful, but quickly realised it is not realistic to
// brute force this. There has to be some other way. But what...

// Notes:
// It seems like both the test data and real data programs end with a jnz instruction where there
// program terminates IF register A is 0, otherwise the program restarts with pointer 0.
//
// I think that with the main program the amound of outputs you get are linked
// Meaning
// 0 - 7 yields 1, x2
// 8 - 63 yields 2 x4
// 64 - 511 yields 3 x4
// 512 - 4095 yields 4 x4
// 4096 - 32 768 yields 5 x4
// And so on.
// If we instead look at the numbers in bits only, we notice that each new result requires 3
// additional bits for the source number.

// I think I have to surrender here.
// Due to my less than perfect knowledge of math I thought I could figure out a smart way to brute
// force it that would be faster to implement than properly calculating the values backwards.
//
//

// First we must analyze both the instructions and program.
//
//
// bitwise xor can be reversed. If the result and one part is known then simply running XOR between
// them will yield the other original part!

// At the end of the program A is 0!!!!
// The last instruction is 3, jnz. Which if A is anything else than 0 will reset the program
// pointer to 0. Meaning that when the program does finish the value of A must be 0.

// That is the last key we needed.
// We simply need to run the program in reverse.
// Set the program pointer to the last instruction, then subtract 2 every time.
// Regarding the jnz instruction, we don't react to it when it is reached, but instead as long as
// there are outputs left to find we loop back to the last jnz instruction once we complete the
// instruction at index 0.
//
// There are problems with that approach though. I feel like it won't be universal for any program,
// but instead be kind of locked down to the specific program.
// There has to be a better way...



Register A: 30118712
Register B: 0
Register C: 0

Program: 2,4, 1,3, 7,5, 4,2, 0,3, 1,5, 5,5, 3,0

Program -> :
i2: A % 8 = x | x -> B          // Only keeps the last 3 bits!
i1: B xor 3 = x | x -> B
i7: A / (2^B) = x | x -> C
i4: B xor C = x | x -> B
i0: A / (2^3) = A / 8 = x | x -> A
i1: B xor 5 = x | x -> B
i5: B % 8 = x | x -> OUT
i3: A == 0 -> reset pointer

Program <- :
i3: 0 -> A;
i5: x % 8 = 0 | x -> B
i1: B xor 5 -> B
i0: x / (2*2) = x / 8 = 0 ->

I have been thinking alot about this, and while I don't like how this is done, I feel like the best way is to make a smarter number iterator.
Instead of adding one, when we find the correct output, we add one multiplied by the output index times 3. That "should" give me the result I want.
WRONG.....

Because of the way the number is processed we have to do it backwards.
Basically we start with 0, add 1 until the LAST output index contains the last value in the program.
Once it matches, we bitshift the number to the left 3 steps. Then once again add one at a time until the next index matches.


216133893759679

*/
