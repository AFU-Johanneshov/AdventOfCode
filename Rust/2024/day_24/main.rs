use std::io;

mod operations;
use circuit::CircuitBuilder;
use operations::Operation;
use operations::OperationResult;

mod circuit;
use circuit::{Circuit, CircuitComponent};

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

fn get_circuit(path: &str) -> Result<(Circuit, Vec<(char, usize, bool)>), AdventError> {
    let mut lines = get_lines(path)?.into_iter();
    let mut inputs = Vec::new();

    while let Some(line) = lines.next() {
        let mut cache = ['_'; 6];
        if line.is_empty() {
            break;
        }
        for (index, char) in line.chars().enumerate() {
            if char == ' ' && index == 0 {
                break;
            }
            cache[index] = char;
        }
        let mut input_id = String::new();
        input_id.push(cache[1]);
        input_id.push(cache[2]);
        println!("{:?}", cache);
        inputs.push((
            cache[0],
            input_id.parse::<usize>().expect(&format!(
                "Unexpected input id: {}. Only numbers are allowed.",
                input_id
            )),
            cache[5] == '1',
        ));
    }

    let mut circuit_builder = CircuitBuilder::new();
    while let Some(line) = lines.next() {
        let mut operation_state = Operation::None;
        let mut strings: [String; 5] = [const { String::new() }; 5];
        let mut strings_index = 0;
        for char in line.chars() {
            let (next_operation, potential_result) = operation_state.next(char);
            operation_state = next_operation;
            if let Some(result) = potential_result {
                if let OperationResult::String(str) = result {
                    strings[strings_index] = str;
                    strings_index += 1;
                }
            }
        }
        let (next_operation, result) = operation_state.collect_operation();
        operation_state = next_operation;
        if let OperationResult::String(str) = result {
            strings[strings_index] = str;
            strings_index += 1;
        }

        //println!("Added gate: \n{:?}", strings);
        circuit_builder.add_gate(
            strings[0].clone(),
            strings[1].clone(),
            strings[2].clone(),
            strings[4].clone(),
        );
    }

    Ok((circuit_builder.assemble(), inputs))
}

fn calculate(path: &str) -> Result<String, AdventError> {
    let (mut circuit, inputs) = get_circuit(path)?;

    //println!("Circuit: \n{:?}", circuit);

    for (modifier, index, powered) in &inputs {
        circuit.set_input(*modifier, *index, *powered);
    }

    let out = circuit.output();

    compare_binary_numbers(inputs.clone(), out);
    circuit.fix();

    for (modifier, index, powered) in &inputs {
        circuit.set_input(*modifier, *index, *powered);
    }

    circuit.fix();
    //circuit.print_info();
    //circuit.fix_full_adders();
    let out = circuit.output();

    compare_binary_numbers(inputs, out);

    todo!();
}

fn compare_binary_numbers(source: Vec<(char, usize, bool)>, result: Vec<bool>) {
    let result_string = {
        let mut result_string = String::new();
        for bit in result.iter().rev() {
            //   result_string.push(if *bit { '1' } else { '0' });
        }

        for i in (0..64).rev() {
            if i < result.len() {
                result_string.push(if result[i] { '1' } else { '0' });
            } else {
                result_string.push('0');
            }
        }
        result_string
    };

    let expected_string = {
        let mut iter = source.iter();
        let mut result_string = String::new();
        let mut expected_number = {
            let mut x = 0;
            let mut binary_exponent = 1;
            for _ in 0..source.len() / 2 {
                if iter.next().unwrap().2 {
                    x += binary_exponent;
                }
                binary_exponent *= 2;
            }

            let mut y = 0;
            binary_exponent = 1;
            for (_, _, bit) in iter {
                if *bit {
                    y += binary_exponent;
                }
                binary_exponent *= 2;
            }

            x + y
        };

        let mut binary_exponent = u64::MAX;
        while binary_exponent > 0 {
            if expected_number > binary_exponent {
                expected_number -= binary_exponent;
                result_string.push('1');
            } else {
                result_string.push('0');
            }
            binary_exponent /= 2;
        }
        result_string
    };

    let difference_string = {
        let (mut expected_chars, mut result_chars) =
            (expected_string.chars(), result_string.chars());
        let mut result = String::new();
        for _ in 0..64 {
            if expected_chars.next().unwrap() != result_chars.next().unwrap() {
                result.push('X');
            } else {
                result.push(' ');
            }
        }
        result
    };

    println!(
        "{}\n{}\n{}\n{}",
        difference_string, expected_string, result_string, difference_string
    );
}

// vkq z11, mmk z24, qdq pvb, hqh z38
// hqh,mmk,pvb,qdq,vkq,z11,z24,z38

/*

    pub fn output(&self) -> u64 {
        let mut binary = Vec::new();
        for index in &self.output_z {
            if let CircuitComponent::Wire(wire) = self.get_component(*index) {
                binary.push(wire.powered);
            }
        }

        let mut result = 0;
        let mut binary_exponent: u64 = 1;
        for b in binary {
            if b {
                result += binary_exponent;
            }
            binary_exponent *= 2;
            //print!("{}", if b { 1 } else { 0 });
        }

        result
    }

*/

fn main() {
    match calculate("data.txt") {
        Err(err) => println!("An error occured: {err:?}"),
        Ok(value) => println!("Result is: {}", value),
    }
}

/*
#[test]
fn calculate_test() {
    match calculate("testdata.txt") {
        Err(err) => panic!("An error occured: {err:?}"),
        Ok(value) => assert_eq!(value, 2024),
    }
}

#[test]
fn calculate_small_test() {
    match calculate("smalltestdata.txt") {
        Err(err) => panic!("An error occured: {err:?}"),
        Ok(value) => assert_eq!(value, 4),
    }
}
*/

/* Sudo code:

Challenge part 1:



Challenge part 2:

// So now the problem is that there are four groups of two gates where the output wires has been swapped.
// The task is to figure out which gates has switched outputs. The circuit is supposed to add two
// numbers together. Meaning we know what it should produce.
//
// First thought is we simply compare the output vs the expected output. Then process the gates
// connected to the wrong numbers, swapping the output of them with others. Then comparing again.
// Shouldn't be too bad to implement.
// Probably a mistake to say that but we shall see.
//
// It might be "better" to try and do it more optimized, but as I see it there are actually not
// that many gates. And since computers are FAST it would likely not be too bad to swap them all
// out, testing each potential change.
// The problem is that from my understanding there is nothing saying the swaps needed won't impact
// each other. Meaning I can't figure out one at a time since I might need another pair to get the
// correct one here.
//
// Is pathfinding the key?
// A-Star with a heuristic prioritising changing outputs between gates somehow connected to wrong
// numbers?
// The score for each gate could be found by calculating the amount of invalid bits vs valid bits
// impacted by that gate. The more invalid the higher priority to swap.
//
// Of course! The whole cirucit is just a chain of full-adders, or supposed to be. All we need to
// do is figure out which full adders are wrong, and then repair them.
//
// I think what we could do is to sort the components into separate full adder components. When
// building the component from the cirucit, we would save any wrong connection together with the
// expected connection in a list.
//
// Fixing the problem this way would likely be the fastest at finding the answer. BUT, it would be
// locked to this specific problem. And wouldn't even work with the provided examples.
// Using pathfinding might still be best.
//
// After further reading and thinking it is clear that the problem is actually simpler than it
// first looks. There are indeed 4 pairs of wires that needs to be swapped with each other. BUT,
// the pairs themsevles are actually not important. Since the answer is all eight wires sorted
// alphabetically.
// Meaning, we only need to figure out which wires are wrongly connected. Not with who.
// There should be exactly 8 wires in the full adders which connect to the wrong wire. Might be
// best to go the route of building full adders anyway.

*/
