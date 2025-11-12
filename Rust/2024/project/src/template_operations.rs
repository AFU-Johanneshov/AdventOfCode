pub enum Operation {
    None,
}

pub enum OperationResult {
    Success,
}

impl Operation {
    pub fn next(self, c: char) -> (Operation, Option<OperationResult>) {
        match self {
            Operation::None => match c {
                _ => (Operation::None, None),
            },
        }
    }
}

fn template_get_next(t: char, c: char) -> (Operation, Option<OperationResult>) {
    match (t, c) {
        (_, _) => (Operation::None, None),
    }
}
