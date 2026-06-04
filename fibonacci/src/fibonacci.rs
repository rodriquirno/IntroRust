pub struct Fib {
    a: u128,
    b: u128,
}

// convenience constructor so you don't have to write Fib { a: 0, b: 1 } every time
impl Fib {
    pub fn new() -> Self {
        Fib { a: 0, b: 1 }
    }
}

// implement the Iterator trait for Fib, so we can use methods like .take() or .nth()
impl Iterator for Fib {
    type Item = u128;

    // next() returns the next Fibonacci number each time it's called, and updates the struct's internal state
    fn next(&mut self) -> Option<u128> {
        let temp = self.a;
        let sum = self.a.checked_add(self.b)?; // if None, stop here
        self.a = self.b;
        self.b = sum;
        Some(temp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts_correctly() {
        let first: Vec<u128> = Fib::new().take(7).collect();
        assert_eq!(first, vec![0, 1, 1, 2, 3, 5, 8]);
    }

    #[test]
    fn seventh_is_eight() {
        assert_eq!(Fib::new().nth(6), Some(8));
    }

    #[test]
    fn first_is_zero() {
        assert_eq!(Fib::new().nth(0), Some(0));
    }

    // --- overflow ---

    #[test]
    fn high_index_does_not_panic() {
        // if this runs without blowing up, checked_add is doing its job
        let _ = Fib::new().nth(200);
    }

    #[test]
    fn out_of_range_index_is_none() {
        // u128 holds up to ~186, so 200 must return None
        assert_eq!(Fib::new().nth(200), None);
    }

    #[test]
    fn last_valid_index_exists() {
        // index 184 still fits in u128 → must be Some
        assert!(Fib::new().nth(184).is_some());
    }
}