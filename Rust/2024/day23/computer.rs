use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub struct Identifier((char, char));
#[derive(Debug, PartialEq)]
pub enum ComputerError {
    IdentifierParseFailed,
}

impl Identifier {
    pub fn parse(chars: (char, char)) -> Result<Identifier, ComputerError> {
        if !Self::valid_char(chars.0) || !Self::valid_char(chars.1) {
            return Err(ComputerError::IdentifierParseFailed);
        }
        Ok(Identifier(chars))
    }

    pub fn as_u16(&self) -> u16 {
        let a = ((self.0 .0 as u16) - 97) * 26;
        let b = (self.0 .1 as u16) - 97;
        a + b
    }

    fn valid_char(char: char) -> bool {
        (char as u16) >= 97 && (char as u16) <= 122
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::with_capacity(2);
        s.push(self.0 .0);
        s.push(self.0 .1);
        write!(f, "{}", s)
    }
}

impl From<u16> for Identifier {
    fn from(value: u16) -> Self {
        let c1 = ((value / 26) as u8 + 97) as char;
        let c2 = ((value % 26) as u8 + 97) as char;
        Identifier((c1, c2))
    }
}

#[derive(Debug, Clone)]
pub struct Computer {
    pub id: u16,
    pub connections: Vec<u16>,
}

impl Computer {
    pub fn new(id: u16) -> Self {
        Computer {
            id,
            connections: Vec::new(),
        }
    }

    pub fn starts_with_t(&self) -> bool {
        self.id >= 494 && self.id <= 519
    }
}

impl From<Identifier> for Computer {
    fn from(identifier: Identifier) -> Self {
        Computer::new(identifier.as_u16())
    }
}

#[cfg(test)]
mod tests {
    use crate::computer::Identifier;

    use super::{Computer, ComputerError};
    const TESTID: u16 = 42;
    const TESTIDENTIFIER: Identifier = Identifier(('b', 'q')); // bq
    const TESTCOMPUTER: Computer = Computer {
        id: TESTID,
        connections: Vec::new(),
    };

    #[test]
    fn new_creates_expected_computer() {
        let result = Computer::new(TESTID);
        assert_eq!(
            result.id, TESTID,
            "Computer::new({}) returned a computer with ID: {} instead of the expected: {}",
            TESTID, result.id, TESTID
        );
    }

    #[test]
    fn starts_with_t_success() {
        assert!(
            Computer::new(Identifier::parse(('t', 'f')).unwrap().as_u16()).starts_with_t(),
            "Computer::starts_with_t() returned false even though the identifier is <tf>!"
        );
        assert!(
            Computer::new(Identifier::parse(('t', 'a')).unwrap().as_u16()).starts_with_t(),
            "Computer::starts_with_t() returned false even though the identifier is <ta>!"
        );
        assert!(
            Computer::new(Identifier::parse(('t', 'z')).unwrap().as_u16()).starts_with_t(),
            "Computer::starts_with_t() returned false even though the identifier is <tz>!"
        );
    }

    #[test]
    fn starts_with_t_fail() {
        assert!(
            !Computer::new(Identifier::parse(('u', 'a')).unwrap().as_u16()).starts_with_t(),
            "Computer::starts_with_t() returned true even though the identifier is <ua>!"
        );
        assert!(
            !Computer::new(Identifier::parse(('s', 'z')).unwrap().as_u16()).starts_with_t(),
            "Computer::starts_with_t() returned true even though the identifier is <sz>!"
        );
    }

    #[test]
    fn identifier_valid_char_success() {
        assert!(
            Identifier::valid_char('a'),
            "Identifier::valid_char('a') returned: false instead of the expected: true"
        );
        assert!(
            Identifier::valid_char('z'),
            "Identifier::valid_char('<') returned: false instead of the expected: true"
        );
        assert!(
            Identifier::valid_char('k'),
            "Identifier::valid_char('k') returned: false instead of the expected: true"
        );
    }

    #[test]
    fn identifier_valid_char_fail() {
        assert!(
            !Identifier::valid_char('0'),
            "Identifier::valid_char('0') returned: true instead of the expected: false"
        );
        assert!(
            !Identifier::valid_char('A'),
            "Identifier::valid_char('A') returned: true instead of the expected: false"
        );
        assert!(
            !Identifier::valid_char('9'),
            "Identifier::valid_char('9') returned: true instead of the expected: false"
        );
    }

    #[test]
    fn identifier_parse_success() {
        let chars = ('t', 'e');
        let Ok(identifier) = Identifier::parse(chars) else {
            panic!("Idenifier::parse({:?}) returned: ComputerError::IdentifierParseFailed instead of the expected Identifier!", chars);
        };
        assert_eq!(identifier.0, chars, "Identifier::parse({:?}) returned a identifier with value: {:?} instead of the expected: {:?}", chars, identifier.0, chars);
    }

    #[test]
    fn identifier_parse_fail() {
        let chars = ('t', 'A');
        match Identifier::parse(chars) {
            Ok(identifier) => panic!("Identifier::parse({:?}) returned as identifier with value: {:?} instead of the expected ComputerError::IdentifierParseFailed!", chars, identifier.0),
            Err(error) => assert_eq!(error, ComputerError::IdentifierParseFailed, "identifier::parse({:?}) returned {:?} instead of the expected: {:?}",chars, error, ComputerError::IdentifierParseFailed),
        }
    }

    #[test]
    fn identifier_as_u16() {
        let result = TESTIDENTIFIER.as_u16();
        assert_eq!(
            result, TESTID,
            "{:?}.as_u16() returned: {} instead of the expected: {}",
            TESTIDENTIFIER, result, TESTID
        );
    }

    #[test]
    fn identifier_from_u16() {
        let result = Identifier::from(TESTID);
        assert_eq!(
            result, TESTIDENTIFIER,
            "Identifier::from({}) returned: {:?} instead of the expected: {:?}",
            TESTID, result, TESTIDENTIFIER
        );
    }
}
