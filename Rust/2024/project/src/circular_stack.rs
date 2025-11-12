pub struct CircularStack<T: Default + Copy> {
    values: [T; 4],
    index: usize,
}

impl<T: Default + Copy> CircularStack<T> {
    pub fn new() -> CircularStack<T> {
        CircularStack {
            values: [T::default(); 4],
            index: 0,
        }
    }

    pub fn with_default(value: T) -> CircularStack<T> {
        CircularStack {
            values: [value; 4],
            index: 0,
        }
    }

    pub fn push(&mut self, value: T) {
        self.values[self.index] = value;
        self.index = (self.index + 1) % 4;
    }

    pub fn peek(&self) -> T {
        self.values[(self.index + 3) % 4]
    }

    pub fn get_stack(&self) -> [T; 4] {
        let mut result = [T::default(); 4];
        for (i, item) in result.iter_mut().enumerate() {
            *item = self.values[3 - ((self.index + i) % 4)];
        }
        result
    }

    pub fn get_queue(&self) -> [T; 4] {
        let mut result = [T::default(); 4];
        for (i, item) in result.iter_mut().enumerate() {
            *item = self.values[(self.index + i) % 4];
        }
        result
    }
}

#[test]
fn test_circular_stack_push() {
    let mut cs: CircularStack<i8> = CircularStack::new();
    let sequence = [1, 2, 3, 4, 5, 6];
    for value in sequence {
        cs.push(value);
    }
    assert_eq!(cs.get_stack(), [6, 5, 4, 3])
}

#[test]
fn test_circular_stack_get_stack() {
    let cs: CircularStack<i8> = CircularStack {
        values: [6, 7, 8, 9],
        index: 2,
    };
    assert_eq!(cs.get_stack(), [7, 6, 9, 8])
}

#[test]
fn test_circular_stack_get_queue() {
    let cs: CircularStack<i8> = CircularStack {
        values: [6, 7, 8, 9],
        index: 2,
    };
    assert_eq!(cs.get_queue(), [8, 9, 6, 7])
}

#[test]
fn test_circular_stack_peek() {
    let mut cs: CircularStack<i8> = CircularStack::new();
    let sequence = [1, 2, 3, 4, 5, 6];
    for value in sequence {
        cs.push(value);
        assert_eq!(cs.peek(), value)
    }
}
