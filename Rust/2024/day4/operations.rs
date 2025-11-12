pub enum Operation {
    None,
    Mas(char),
    Sam(char),
}

pub enum OperationResult {
    Success,
}

impl Operation {
    pub fn next(self, c: char) -> (Operation, Option<OperationResult>) {
        match self {
            Operation::None => match c {
                'M' => (Operation::Mas('M'), None),
                'S' => (Operation::Sam('S'), None),
                _ => (Operation::None, None),
            },
            Operation::Mas(t) => mas_get_next(t, c),
            Operation::Sam(t) => amx_get_next(t, c),
        }
    }
}

fn mas_get_next(t: char, c: char) -> (Operation, Option<OperationResult>) {
    match (t, c) {
        ('M', 'A') => (Operation::Mas('A'), None),
        ('A', 'S') => (Operation::Sam('S'), Some(OperationResult::Success)),
        (_, 'M') => (Operation::Mas('M'), None),
        (_, 'S') => (Operation::Sam('S'), None),
        (_, _) => (Operation::None, None),
    }
}

fn amx_get_next(t: char, c: char) -> (Operation, Option<OperationResult>) {
    match (t, c) {
        ('S', 'A') => (Operation::Sam('A'), None),
        ('A', 'M') => (Operation::Mas('M'), Some(OperationResult::Success)),
        (_, 'M') => (Operation::Mas('M'), None),
        (_, 'S') => (Operation::Sam('S'), None),
        (_, _) => (Operation::None, None),
    }
}
