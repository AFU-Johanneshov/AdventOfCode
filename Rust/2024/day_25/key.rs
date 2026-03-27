use std::fmt::Display;

const KEYLENGTH: usize = 5;
const KEYHEIGTH: usize = 6;
const KEYCHARBLOCKED: char = '#';
const KEYCHAROPEN: char = '.';

pub struct Key {
    value: u32,
    raw: [u8; KEYLENGTH],
}

impl Key {
    pub fn overlaps(&self, other: &Key) -> bool {
        for i in 0..KEYLENGTH {
            if self.raw[i] as usize + other.raw[i] as usize >= KEYHEIGTH {
                return true;
            }
        }
        false
    }
}

impl Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Key {} | -:{:?}:-", self.value, self.raw)
    }
}

pub enum KeyBuilderError {
    LineOverflow(String),
    LineParseFailed(String),
}

impl Display for KeyBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LineOverflow(message) => {
                write!(f, "KeyBuilderError::LineOverflow\n{}\n", message)
            }
            Self::LineParseFailed(message) => {
                write!(f, "KeyBuilderError::LineParseFailed\n{}\n", message)
            }
        }
    }
}

pub struct KeyBuilder {
    raw: [u8; KEYLENGTH],
    height: usize,
}

impl KeyBuilder {
    pub fn new() -> KeyBuilder {
        KeyBuilder {
            raw: [0; KEYLENGTH],
            height: 0,
        }
    }

    pub fn add_line(&mut self, line: &String) -> Result<(), KeyBuilderError> {
        if self.height >= KEYHEIGTH {
            return Err(KeyBuilderError::LineOverflow(format!("KeyBuilder.add_line({}): Attempted to add a line when the keybuilder already is at maximum height {}.", line, self.height)));
        }

        if line.chars().count() != KEYLENGTH {
            return Err(KeyBuilderError::LineParseFailed(format!(
                "KeyBuilder.add_line({}): The provided line is the wrong length! Expected {} characters.",
                line, KEYLENGTH,
            )));
        }

        for (index, char) in line.chars().enumerate() {
            match char {
                '.' => {}
                '#' => self.raw[index] += 1,
                _ => {
                    return Err(KeyBuilderError::LineParseFailed(format!("KeyBuilder.add_line({}): The provided line contained invalid character: {}\nOnly '{}' and '{}' is allowed.",
                                line, char, KEYCHARBLOCKED, KEYCHAROPEN)));
                }
            }
        }

        self.height += 1;
        Ok(())
    }

    pub fn assemble(self) -> Key {
        let key_value: u32 = self.get_key_value();
        Key {
            value: key_value,
            raw: self.raw,
        }
    }

    fn get_key_value(&self) -> u32 {
        let mut result = 0;
        let mut multiplier: u32 = (KEYHEIGTH as u32).pow(KEYLENGTH as u32 - 1);
        for value in self.raw {
            result += value as u32 * multiplier;
            multiplier /= KEYHEIGTH as u32;
        }
        result
    }
}
