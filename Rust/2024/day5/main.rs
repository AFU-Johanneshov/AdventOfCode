mod operations;
use operations::Operation;
use operations::OperationResult;

mod reader;
use reader::get_lines;

fn main() {
    let Ok(reader) = get_lines("./data.txt") else {
        panic!(); // Panic if file not found. Not viable for production code but for prototying it
                  // is fine.
    };

    let mut operation_state: Operation = Operation::None;
    let mut read_rules: bool = true;
    let mut total: u32 = 0;

    let mut rules: [[bool; 100]; 100] = [[false; 100]; 100];

    for line in reader {
        let mut buffer: [u8; 23] = [0; 23];
        let mut index: usize = 0;

        for char in line.chars() {
            let (next_op, output) = operation_state.next(char);
            operation_state = next_op;
            let Some(op_result) = output else {
                continue;
            };
            match op_result {
                OperationResult::ModeSwitch => {
                    read_rules = false;
                    println!("Switched mode!");
                }
                OperationResult::TwoDigitNumber(value) => {
                    buffer[index] = value;
                    index += 1;
                }
            }
        }

        //println!("Buffer: {:?}  |  Lenght: {}", buffer, index);

        if read_rules {
            rules[buffer[0] as usize][buffer[1] as usize] = true;
        } else {
            //if let Some(_) = get_value_if_valid(buffer, index, &rules) {
            if get_value_if_valid(buffer, index, &rules).is_some() {
                continue;
            }

            total += reorder_and_get_value(buffer, index, &rules) as u32;
        }

        operation_state = Operation::None;
    }

    println!("Total is: {total}");
}

fn get_value_if_valid(update: [u8; 23], length: usize, rules: &[[bool; 100]; 100]) -> Option<u8> {
    println!("Checking rules...");

    for page_index in 0..length {
        for i in 0..page_index {
            // If page_index and i is a rule where page_index must be printed before i return none.
            // Since i only loops up to page_index it means that if page_index must be printed
            // before i the order violates the rules.
            if rules[update[page_index] as usize][update[i] as usize] {
                return None;
            }
        }
    }

    Some(update[length / 2])
}

fn reorder_and_get_value(mut update: [u8; 23], length: usize, rules: &[[bool; 100]; 100]) -> u8 {
    for page_index in 0..length {
        for i in 0..page_index {
            if order_is_wrong(update[page_index], update[i], rules) {
                update.swap(page_index, i);
                return reorder_and_get_value(update, length, rules);
            }
        }
    }

    update[length / 2]

    /*
    for i in (0..length) {
        for page_number in next_index..length {

            //
        }
    }*/
}

fn order_is_wrong(first_value: u8, second_value: u8, rules: &[[bool; 100]; 100]) -> bool {
    rules[first_value as usize][second_value as usize]
}

fn swap_index(index_1: usize, index_2: usize, array: &mut [u8; 23]) {
    array.swap(index_1, index_2);
}

/*

RULE FORMAT:
rules[value which must be before][this value]

Part 2 todo:

For each invalid update:





*/
