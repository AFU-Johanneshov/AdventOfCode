pub enum Operation {
    None,
    Number(String),
}

pub enum OperationResult {
    Number(i16),
}

impl Operation {
    pub fn next(self, c: char) -> (Operation, Option<OperationResult>) {
        match self {
            Operation::None => match c {
                i if i.is_digit(10) => (Operation::Number(String::from(c)), None),
                _ => (Operation::None, None),
            },
            Operation::Number(number_string) => number_get_next(c, number_string),
        }
    }
}

fn number_get_next(c: char, number_string: String) -> (Operation, Option<OperationResult>) {
    match c {
        i if i.is_digit(10) => {
            let mut nr = number_string;
            nr.push(c);
            (Operation::Number(nr), None)
        }
        _ => (
            Operation::None,
            Some(OperationResult::Number(number_string.parse().expect(
                "Could not convert string to integer! Check the operation get next functions!",
            ))),
        ),
    }
}

/*
fn template_get_next(t: char, c: char) -> (Operation, Option<OperationResult>) {
    match (t, c) {
        (_, _) => (Operation::None, None),
    }
}*/
