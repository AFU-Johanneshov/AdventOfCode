pub enum Operation {
    None,
    Value(String),
}

pub enum OperationResult {
    TestValue(u64),
    Number(u32),
}

impl Operation {
    pub fn next(self, c: char) -> (Operation, Option<OperationResult>) {
        match self {
            Operation::None => match c {
                c if c.is_numeric() => {
                    let mut str = String::new();
                    str.push(c);
                    (Operation::Value(str), None)
                }
                _ => (Operation::None, None),
            },
            Operation::Value(str) => value_get_next(str, c),
        }
    }
}

fn value_get_next(mut str: String, c: char) -> (Operation, Option<OperationResult>) {
    match c {
        c if c.is_numeric() => {
            str.push(c);
            (Operation::Value(str), None)
        }
        ' ' => {
            let result = str.parse::<u32>().unwrap();
            (Operation::None, Some(OperationResult::Number(result)))
        }
        ':' => {
            let result = str.parse::<u64>().unwrap();
            (Operation::None, Some(OperationResult::TestValue(result)))
        }
        _ => (Operation::None, None),
    }
}
