#[allow(non_snake_case)]
#[derive(Debug)]
pub struct ChronoSpatialComputer {
    A: i64,
    B: i64,
    C: i64,
    instruction_pointer: usize,
    program: Vec<u8>,
}

impl Iterator for &mut ChronoSpatialComputer {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        while self.instruction_pointer < self.program.len() {
            if let Some(output) = self.process_next() {
                return Some(output);
            }
        }
        None
    }
}

impl ChronoSpatialComputer {
    #[allow(non_snake_case)]
    pub fn new(A: i64, B: i64, C: i64) -> Self {
        ChronoSpatialComputer {
            A,
            B,
            C,
            instruction_pointer: 0,
            program: Vec::new(),
        }
    }

    pub fn run_program(&mut self) -> Vec<u8> {
        self.into_iter().collect::<Vec<u8>>()
    }

    pub fn regit(&mut self, register: char, value: i64) {
        match register {
            'A' => self.A = value,
            'B' => self.B = value,
            'C' => self.C = value,
            _ => panic!("Register '{}' does not exist!", register),
        }
    }

    pub fn program(&mut self, program: Vec<u8>) {
        self.instruction_pointer = 0;
        self.program = program;
    }

    pub fn reset_program(&mut self) {
        self.instruction_pointer = 0;
    }

    fn process_next(&mut self) -> Option<u8> {
        match self.program[self.instruction_pointer] {
            0 => self.adv(),
            1 => self.bxl(),
            2 => self.bst(),
            3 => self.jnz(),
            4 => self.bxc(),
            5 => {
                return Some(self.out());
            }
            6 => self.bdv(),
            7 => self.cdv(),
            _ => panic!(
                "ChronoSpatial Computer crashed due to a invalid instruction_pointer! {}",
                self.instruction_pointer
            ),
        }
        None
    }

    fn adv(&mut self) {
        self.A /= 2_i64.pow(self.combo_operand() as u32);
        self.instruction_pointer += 2;
    }
    fn bxl(&mut self) {
        self.B ^= self.literal_operand();
        self.instruction_pointer += 2;
    }
    fn bst(&mut self) {
        self.B = self.combo_operand() % 8;
        self.instruction_pointer += 2;
    }
    fn jnz(&mut self) {
        if self.A == 0 {
            self.instruction_pointer += 2;
        } else {
            self.instruction_pointer = self.literal_operand() as usize;
        }
    }
    fn bxc(&mut self) {
        self.B ^= self.C;
        self.instruction_pointer += 2;
    }
    fn out(&mut self) -> u8 {
        let output = (self.combo_operand() % 8) as u8;
        self.instruction_pointer += 2;
        output
    }
    fn bdv(&mut self) {
        self.B = self.A / 2_i64.pow(self.combo_operand() as u32);
        self.instruction_pointer += 2;
    }
    fn cdv(&mut self) {
        self.C = self.A / 2_i64.pow(self.combo_operand() as u32);
        self.instruction_pointer += 2;
    }

    fn literal_operand(&self) -> i64 {
        self.program[self.instruction_pointer + 1] as i64
    }

    fn combo_operand(&self) -> i64 {
        let operand = self.program[self.instruction_pointer + 1];

        match operand {
            0..=3 => operand as i64,
            4 => self.A,
            5 => self.B,
            6 => self.C,
            _ => panic!("Invalid combo operand"),
        }
    }
}

// Chronospatial computer.
//
// Each instruction reads the following 3-bit number after itself as an input.
//
// I'm thinking a struct for the cronospatial computer.
// A: i64,
// B: i64,
// C: i64,
//
//
// Instructions:
// 0: adv:
