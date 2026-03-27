#[derive(Clone, Copy)]
pub struct SecretNumber(i64);

impl SecretNumber {
    pub fn new(value: i64) -> SecretNumber {
        SecretNumber(value)
    }

    pub fn chain_next(self, steps_forward: u64) -> SecretNumber {
        let mut cache = self;
        for _ in 0..steps_forward {
            cache = cache.next();
        }
        cache
    }

    pub fn next(self) -> SecretNumber {
        let mut cache = self;

        let multiplication_result = cache.0 * 64;
        cache = cache.mix(SecretNumber(multiplication_result)).prune();
        let division_result = cache.0 / 32;
        cache = cache.mix(SecretNumber(division_result)).prune();
        let multiplication_result = cache.0 * 2048;
        cache = cache.mix(SecretNumber(multiplication_result)).prune();

        cache
    }

    fn mix(self, other: SecretNumber) -> SecretNumber {
        SecretNumber(self.0 ^ other.0)
    }

    fn prune(self) -> SecretNumber {
        SecretNumber(self.0 % 16777216)
    }

    pub fn value(&self) -> i64 {
        self.0
    }

    pub fn price(&self) -> i8 {
        (self.0 % 10) as i8
    }
}

#[test]
fn test_new() {
    let result = SecretNumber::new(42);
    assert_eq!(
        result.value(),
        42,
        "SecretNumber::new(42).value() returned: {} instead of the expected: 42",
        result.value()
    )
}

#[test]
fn test_chain_next() {
    let result = SecretNumber(123).chain_next(10);
    assert_eq!(
        result.0, 5908254,
        "SecretNumber(123).chain_next(10) returned: {} instead of the expected: {}",
        result.0, 5908254
    );
}

#[test]
fn test_next() {
    let mut current_number = SecretNumber(123);
    let sequence = [
        15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432, 5908254,
    ];

    for next_expected in sequence {
        let next_received = current_number.next().0;
        assert_eq!(
            current_number.next().0,
            next_expected,
            "SecretNumber({}).next() returned: {} instead of the expected: {}",
            current_number.0,
            next_received,
            next_expected
        );
        current_number = SecretNumber(next_expected);
    }
}

#[test]
fn test_mix() {
    let result = SecretNumber(42).mix(SecretNumber(15)).0;
    assert_eq!(
        result, 37,
        "SecretNumber(42).mix(15) returned: {} instead of the expected: 37",
        result,
    );
}

#[test]
fn test_prune() {
    let result = SecretNumber(100000000).prune().0;
    assert_eq!(
        result, 16113920,
        "SecretNumber(100000000).prune() returned: {} instead of the expected: 16113920",
        result
    );
}

#[test]
fn test_value() {
    let result = SecretNumber(42).value();
    assert_eq!(
        result, 42,
        "SecretNumber(42).value() returned: {result} instead of the expected: 42"
    );
}

#[test]
fn test_price() {
    let result = SecretNumber(42).price();
    assert_eq!(
        result, 2,
        "SecretNumber(42).price() returned: {result} instead of the expected: 2"
    );
    let result = SecretNumber(32753).price();
    assert_eq!(
        result, 3,
        "SecretNumber(32753).price() returned: {result} instead of the expected: 3"
    );
    let result = SecretNumber(32759).price();
    assert_eq!(
        result, 9,
        "SecretNumber(32759).price() returned: {result} instead of the expected: 9"
    );
}
