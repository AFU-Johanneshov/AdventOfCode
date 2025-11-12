use std::collections::HashMap;
use std::io;

mod operations;

mod vector;
use vector::VectorI16;

mod reader;
use reader::get_lines;
mod testing_debug;

#[derive(Debug)]
enum AdventError {
    IoError(io::Error),
    CorruptedData(String),
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
enum DirectionalButtonPress {
    Up,
    Down,
    Left,
    Right,
    Accept,
}

impl DirectionalButtonPress {
    fn button_location(&self) -> VectorI16 {
        match self {
            Self::Up => VectorI16 { x: 1, y: 0 },
            Self::Left => VectorI16 { x: 0, y: 1 },
            Self::Right => VectorI16 { x: 2, y: 1 },
            Self::Down => VectorI16 { x: 1, y: 1 },
            Self::Accept => VectorI16 { x: 2, y: 0 },
        }
    }

    fn copy_and_insert<T: Copy>(vec: &mut Vec<T>, value: T, copies: u64) {
        for _ in 0..copies {
            vec.push(value);
        }
    }

    fn get_pointer_operations(
        &self,
        other: &DirectionalButtonPress,
    ) -> Vec<DirectionalButtonPress> {
        let mut result: Vec<DirectionalButtonPress> = Vec::new();

        let difference = other.button_location() - self.button_location();
        let abs_diff_x = difference.x.unsigned_abs() as u64;
        let abs_diff_y = difference.y.unsigned_abs() as u64;
        let vertical_axis = if difference.y <= 0 {
            DirectionalButtonPress::Up
        } else {
            DirectionalButtonPress::Down
        };
        let horizontal_axis = if difference.x <= 0 {
            DirectionalButtonPress::Left
        } else {
            DirectionalButtonPress::Right
        };

        if difference.x < 0 && (other.button_location().x != 0 || self.button_location().y != 0) {
            Self::copy_and_insert(&mut result, horizontal_axis, abs_diff_x);
            Self::copy_and_insert(&mut result, vertical_axis, abs_diff_y);
        } else if difference.y > 0
            || difference.y < 0 && (other.button_location().y != 0 || self.button_location().x != 0)
        {
            Self::copy_and_insert(&mut result, vertical_axis, abs_diff_y);
            Self::copy_and_insert(&mut result, horizontal_axis, abs_diff_x);
        } else {
            Self::copy_and_insert(&mut result, horizontal_axis, abs_diff_x);
            Self::copy_and_insert(&mut result, vertical_axis, abs_diff_y);
        }

        result
    }
}

struct DirectionalCode(HashMap<(DirectionalButtonPress, DirectionalButtonPress), u64>);

impl DirectionalCode {
    fn pointer_steps(&self) -> DirectionalCode {
        let mut new_map: HashMap<(DirectionalButtonPress, DirectionalButtonPress), u64> =
            HashMap::new();

        for ((current_button, new_button), occurances) in self.0.iter() {
            let mut button_sequence = current_button.get_pointer_operations(new_button);
            button_sequence.push(DirectionalButtonPress::Accept);

            let mut current = DirectionalButtonPress::Accept;
            for next in button_sequence {
                *new_map.entry((current, next)).or_default() += occurances;
                current = next;
            }
        }

        DirectionalCode(new_map)
    }

    fn from_sequence(sequence: Vec<DirectionalButtonPress>) -> Self {
        let mut new_map: HashMap<(DirectionalButtonPress, DirectionalButtonPress), u64> =
            HashMap::new();
        let mut current = DirectionalButtonPress::Accept;
        for next in sequence {
            *new_map.entry((current, next)).or_default() += 1;
            current = next;
        }
        DirectionalCode(new_map)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum NumericButtonPress {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Accept,
}

impl NumericButtonPress {
    fn parse(char: char) -> Result<NumericButtonPress, AdventError> {
        match char {
            'A' => Ok(Self::Accept),
            '0' => Ok(Self::Zero),
            '1' => Ok(Self::One),
            '2' => Ok(Self::Two),
            '3' => Ok(Self::Three),
            '4' => Ok(Self::Four),
            '5' => Ok(Self::Five),
            '6' => Ok(Self::Six),
            '7' => Ok(Self::Seven),
            '8' => Ok(Self::Eight),
            '9' => Ok(Self::Nine),
            _ => Err(AdventError::CorruptedData(format!("NumericButtonPress::parse | Received char: {char} did not match any numberic buttons!"))),
        }
    }

    fn to_char(self) -> char {
        match self {
            Self::Accept => 'A',
            Self::Zero => '0',
            Self::One => '1',
            Self::Two => '2',
            Self::Three => '3',
            Self::Four => '4',
            Self::Five => '5',
            Self::Six => '6',
            Self::Seven => '7',
            Self::Eight => '8',
            Self::Nine => '9',
        }
    }

    fn button_location(&self) -> VectorI16 {
        match self {
            Self::Zero => VectorI16 { x: 1, y: 3 },
            Self::One => VectorI16 { x: 0, y: 2 },
            Self::Two => VectorI16 { x: 1, y: 2 },
            Self::Three => VectorI16 { x: 2, y: 2 },
            Self::Four => VectorI16 { x: 0, y: 1 },
            Self::Five => VectorI16 { x: 1, y: 1 },
            Self::Six => VectorI16 { x: 2, y: 1 },
            Self::Seven => VectorI16 { x: 0, y: 0 },
            Self::Eight => VectorI16 { x: 1, y: 0 },
            Self::Nine => VectorI16 { x: 2, y: 0 },
            Self::Accept => VectorI16 { x: 2, y: 3 },
        }
    }

    fn copy_and_insert<T: Copy>(vec: &mut Vec<T>, value: T, copies: u64) {
        for _ in 0..copies {
            vec.push(value);
        }
    }

    fn get_pointer_operations(&self, other: &NumericButtonPress) -> Vec<DirectionalButtonPress> {
        let mut result: Vec<DirectionalButtonPress> = Vec::new();

        let difference = other.button_location() - self.button_location();
        let abs_diff_x = difference.x.unsigned_abs() as u64;
        let abs_diff_y = difference.y.unsigned_abs() as u64;

        let vertical_axis = if difference.y <= 0 {
            DirectionalButtonPress::Up
        } else {
            DirectionalButtonPress::Down
        };
        let horizontal_axis = if difference.x <= 0 {
            DirectionalButtonPress::Left
        } else {
            DirectionalButtonPress::Right
        };

        if difference.x < 0 && (other.button_location().x != 0 || self.button_location().y != 3) {
            Self::copy_and_insert(&mut result, horizontal_axis, abs_diff_x);
            Self::copy_and_insert(&mut result, vertical_axis, abs_diff_y);
        } else if difference.y < 0
            || difference.y > 0 && (other.button_location().y != 3 || self.button_location().x != 0)
        {
            Self::copy_and_insert(&mut result, vertical_axis, abs_diff_y);
            Self::copy_and_insert(&mut result, horizontal_axis, abs_diff_x);
        } else {
            Self::copy_and_insert(&mut result, horizontal_axis, abs_diff_x);
            Self::copy_and_insert(&mut result, vertical_axis, abs_diff_y);
        }

        result
    }
}

const CODELENGTH: usize = 4;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct NumericCode([NumericButtonPress; CODELENGTH]);

impl NumericCode {
    fn from_line(line: String) -> Result<NumericCode, AdventError> {
        let mut code = [NumericButtonPress::Zero; CODELENGTH];
        let line_length = line.chars().count();
        if line_length != CODELENGTH {
            return Err(AdventError::CorruptedData(format!("Code::from_line | Line length did not match the expected length! Expected: {} but received: {}", CODELENGTH, line_length)));
        }
        for (index, char) in line.chars().enumerate() {
            code[index] = NumericButtonPress::parse(char)?;
        }
        Ok(NumericCode(code))
    }

    fn steps(&self) -> DirectionalCode {
        let mut result: Vec<DirectionalButtonPress> = Vec::new();
        let mut current_button = NumericButtonPress::Accept;
        for new_button in self.0 {
            result.extend(current_button.get_pointer_operations(&new_button));
            result.push(DirectionalButtonPress::Accept);
            current_button = new_button;
        }

        let mut directional_code_cache = DirectionalCode::from_sequence(result);
        for _ in 0..25 {
            directional_code_cache = directional_code_cache.pointer_steps();
        }

        directional_code_cache
    }

    fn number(&self) -> u64 {
        let mut cache = String::new();
        for c in self.0.iter().take(3).map(|nbp| nbp.to_char()) {
            cache.push(c);
        }
        cache.parse().expect("The three first button_presses in a numberic code should ALWAYS be numbers and not Accept!")
    }

    fn complexity(&self) -> u64 {
        let total_sequence_length = {
            let mut t = 0;
            for value in self.steps().0.values() {
                t += value
            }
            t
        };

        self.number() * total_sequence_length
    }
}

impl From<io::Error> for AdventError {
    fn from(err: io::Error) -> Self {
        Self::IoError(err)
    }
}

fn calculate(path: &str) -> Result<u64, AdventError> {
    let lines = get_lines(path)?;

    let mut result: u64 = 0;

    for line in lines {
        let code = NumericCode::from_line(line)?;
        result += code.complexity();
    }

    Ok(result)
}

fn main() {
    match calculate("data.txt") {
        Err(err) => println!("An error occured: {err:?}"),
        Ok(value) => println!("Result is: {}", value),
    }
}

#[test]
fn calculate_test() {
    match calculate("data.txt") {
        Err(err) => panic!("An error occured: {err:?}"),
        Ok(value) => assert_eq!(value, 271397390297138),
    }
}

/* Sudo code:

Challenge part 1:

Might be worth to just brute force it with a pathfinder instead. Something is wrong, and I feel a new approach is needed.

219990 ---
222102 is too low
222670 is ---
226562 ---
228642 ---
229246 ---
230142 is too high
235218 is too high

Challenge part 2:

1794422083 is too low

*/

#[test]
fn test_numeric_button_press_parse_success() {
    assert_eq!(
        NumericButtonPress::Two,
        NumericButtonPress::parse('2').expect("Parse(2) returned a CorruptedData error!")
    );
}

#[test]
fn test_numeric_button_press_parse_error() {
    if let Ok(button_press) = NumericButtonPress::parse('x') {
        panic!(
            "Parse(x) unexpectedly returned a NumericButtonPress::{:?} instead of an error!",
            button_press,
        );
    }
}

#[test]
fn test_code_from_line_success() {
    type Button = NumericButtonPress;

    let target: [NumericButtonPress; CODELENGTH] =
        [Button::One, Button::Two, Button::Eight, Button::Accept];

    let Ok(code) = NumericCode::from_line("128A".to_string()) else {
        panic!("Code::from_line(128A) returned a CorruptedData error!");
    };
    assert_eq!(code, NumericCode(target));
}

#[test]
fn test_consts_untouched() {
    assert_eq!(
        4, CODELENGTH,
        "Const: CODELENGTH has been altered from 4 to {CODELENGTH}! Is this intentional?"
    );
}

#[test]
fn test_code_from_line_errors() {
    if let Ok(code) = NumericCode::from_line("4264A".to_string()) {
        panic!(
            "Code::from_line(4264A) returned a {:?} instead of a error! Expected behaviour is to receive a error because of it not matching the CODELENGTH of 4",
            code
        );
    };

    if let Ok(code) = NumericCode::from_line("42A".to_string()) {
        panic!(
            "Code::from_line(42A) returned a {:?} instead of a error! Expected behaviour is to receive a error because of it not matching the CODELENGTH of 4",
            code
        );
    };

    if let Ok(code) = NumericCode::from_line("12Xl".to_string()) {
        panic!(
            "Code::from_line(12) returned a {:?} instead of a error! Expected behaviour is to receive a error because of the line containing invalid characters!",
            code
        );
    };
}
