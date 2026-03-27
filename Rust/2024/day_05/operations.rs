pub enum Operation {
    None,
    //Rule(u8, String, String), // Format: nn|nn
    //UpdatePages,              // Format: nn,nn,nn,nn....
    TwoDigits(u8, u8),
    //ModeSwitch, // Not needed since it only is active for one character and gives a result right away.
}

pub enum OperationResult {
    //Rule(u8, u8),
    //Update([u8; 23]),
    TwoDigitNumber(u8),
    ModeSwitch,
}

impl Operation {
    pub fn next(self, c: char) -> (Operation, Option<OperationResult>) {
        match self {
            Operation::None => match c {
                c if c.is_digit(10) => two_digits_get_next(0, 0, c),
                'S' => (Operation::None, Some(OperationResult::ModeSwitch)),
                _ => (Operation::None, None),
            },
            //Operation::Rule => rule_get_next(t, c),
            //Operation::UpdatePages => {}
            Operation::TwoDigits(index, number) => two_digits_get_next(index, number, c),
        }
    }
}

/*
fn rule_get_next(index: u8, char: char) -> (Operation, Option<OperationResult>) {
    match (index, char) {
        (0, char) if char.is_digit(10) => (
            Operation::Rule(1, char.parse::<u8>().unwrap() * 10, 0),
            None,
        ),
        (_, _) => (Operation::None, None),
    }
}

fn update_pages_get_next(t: char, c: char) -> (Operation, Option<OperationResult>) {
    match (t, c) {
        (_, _) => (Operation::None, None),
    }
}*/

fn two_digits_get_next(index: u8, number: u8, char: char) -> (Operation, Option<OperationResult>) {
    match (index, char) {
        (0, char) if char.is_digit(10) => (
            Operation::TwoDigits(1, char.to_digit(10).unwrap() as u8 * 10),
            None,
        ),
        (1, char) if char.is_digit(10) => (
            Operation::None,
            Some(OperationResult::TwoDigitNumber(
                number + char.to_digit(10).unwrap() as u8,
            )),
        ),
        (_, _) => (Operation::None, None),
    }
}
