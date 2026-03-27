use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

enum Operation {
    None,
    Mul(char, String, String),
    Do(char),
    DoNot(char),
}

enum OperationResult {
    Number(u32),
    Do,
    DoNot,
}

/*


*/

impl Operation {
    fn next(self, c: char) -> (Operation, Option<OperationResult>) {
        match self {
            Operation::None => match c {
                'm' => (Operation::Mul('m', String::new(), String::new()), None),
                'd' => (Operation::Do('d'), None),
                _ => (Operation::None, None),
            },
            Operation::Mul(t, nr_str1, nr_str2) => mul_get_next(t, nr_str1, nr_str2, c),
            Operation::Do(t) => do_get_next(t, c),
            Operation::DoNot(t) => do_not_get_next(t, c),
        }
    }
}

fn mul_get_next(
    t: char,
    mut nr_str1: String,
    mut nr_str2: String,
    c: char,
) -> (Operation, Option<OperationResult>) {
    match (t, c) {
        ('m', 'u') => (Operation::Mul('u', nr_str1, nr_str2), None),
        ('u', 'l') => (Operation::Mul('l', nr_str1, nr_str2), None),
        ('l', '(') => (Operation::Mul('(', nr_str1, nr_str2), None),
        ('0' | '(', c) if c.is_digit(10) => {
            nr_str1.push(c);
            (Operation::Mul('0', nr_str1, nr_str2), None)
        }
        ('0', ',') => (Operation::Mul(',', nr_str1, nr_str2), None),
        (',' | '1', c) if c.is_digit(10) => {
            nr_str2.push(c);
            (Operation::Mul('1', nr_str1, nr_str2), None)
        }
        ('1', ')') => {
            let i: u32 = nr_str1.parse::<u32>().unwrap() * nr_str2.parse::<u32>().unwrap();
            (Operation::None, Some(OperationResult::Number(i)))
        }
        (_, _) => (Operation::None, None),
    }
}

fn do_get_next(t: char, c: char) -> (Operation, Option<OperationResult>) {
    match (t, c) {
        ('d', 'o') => (Operation::Do('o'), None),
        ('o', '(') => (Operation::Do('('), None),
        ('(', ')') => (Operation::None, Some(OperationResult::Do)),
        ('o', 'n') => (Operation::DoNot('n'), None),
        (_, _) => (Operation::None, None),
    }
}

fn do_not_get_next(t: char, c: char) -> (Operation, Option<OperationResult>) {
    match (t, c) {
        ('n', '\'') => (Operation::DoNot('\''), None),
        ('\'', 't') => (Operation::DoNot('t'), None),
        ('t', '(') => (Operation::DoNot('('), None),
        ('(', ')') => (Operation::None, Some(OperationResult::DoNot)),
        (_, _) => (Operation::None, None),
    }
}

/*



*/

fn get_reader<P>(path: P) -> io::Result<io::BufReader<File>>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    Ok(io::BufReader::new(file))
}

fn main() {
    let mut operation: Operation = Operation::None;
    let mut total: u32 = 0;
    let mut allow_mul: bool = true;

    let Ok(reader) = get_reader("./data.txt") else {
        panic!(); // Panic if file not found. Not viable for production code but for prototying it
                  // is fine.
    };

    for line in reader.lines().flatten() {
        for char in line.chars() {
            let (next_op, output) = operation.next(char);
            operation = next_op;
            let Some(op_result) = output else {
                continue;
            };

            match op_result {
                OperationResult::Number(i) => {
                    if allow_mul {
                        total += i;
                    }
                }
                OperationResult::Do => allow_mul = true,
                OperationResult::DoNot => allow_mul = false,
            };
        }
    }

    println!("Total is: {total}");
}

/*

Todo list:

int total
smarttype: operation
Read one char at a time from file.
    (operation, result) = operation.Next(char)
    if result != null: total += result

once finished total is the answer.


smarttype: Enum: operation
::None
::Mul(char, string, string)

operation: Methods:
.Next(char) -> operation and potential integer (Option<int>)
    if ::None
        if char = 'm' return ::Mul(m, "", "");
    if ::Mul
        if (mul.char = m && char = u) return ::Mul(u, "", "")
        if (mul.char = u && char = l) return ::Mul(l, "", "")
        if (mul.char = l && char = '(') return ::Mul('(', "", "")
        if (mul.char = '(' && char.isnumber) return ::Mul('0', +"char", "")
        if (mul.char = 0 && char.isnumber) return ::Mul('0', +"char", "")
        if (mul.char = 0 && char = ',') return ::Mult(',', +"", "");
        if (mul.char = ',' && char.isnumber) return ::Mul('1', +"", +"char")
        if (mul.char = 1 && char.isnumber) return ::Mul('1', +"", +"char")
        if (mul.char = 0 && char = ')')
            int i = mul.string1 as int * mul.string2 as int
            return ::None and i
        return ::None


*/
