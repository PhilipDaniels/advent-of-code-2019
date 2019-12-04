
struct ConstrainedRange {
    min: u32,
    max: u32,
    digits: Vec<u8>,
    max_digits: Vec<u8>,
}

impl ConstrainedRange {
    fn new(min: u32, max: u32) -> Self {
        Self {
            min: min,
            max: max,
            digits: Self::num_to_digits(min),
            max_digits: Self::num_to_digits(max),
        }
    }

    /// Convert n to a vec of its digits.
    /// 0 is the most significant digit (preserves reading order).
    fn num_to_digits(mut n: u32) -> Vec<u8> {
        if n == 0 { return vec![0]; }

        let mut result = Vec::with_capacity(6);

        while n != 0 {
            let rem = n % 10;       // Extract the rightmost digit.
            result.push(rem as u8);
            n /= 10;
        }

        result.reverse();
        result
    }

    fn digits_to_num(digits: &[u8]) -> u32 {
        0
    }

    /// Returns true if the current value is valid according
    /// to the rules of the puzzle.
    fn is_valid(&self) -> bool {
        if self.digits > self.max_digits {
            return false;
        }

        // Check for increasing digits towards the least signficant digit.
        for i in 0..self.digits.len() - 1 {
            if self.digits[i + 1] < self.digits[i] {
                return false
            }
        }

        // Check for a repeated pair. Need 1 repeated pair to
        // be valid.
        for i in 0..self.digits.len() - 1 {
            if self.digits[i + 1] == self.digits[i] {
                return true
            }
        }

        false
    }

    // This is used for part 1.
    fn is_valid2(digits: &[u8]) -> bool {
        // Check for increasing digits towards the least signficant digit.
        for i in 0..digits.len() - 1 {
            if digits[i + 1] < digits[i] {
                return false;
            }
        }

        // Check for a repeated pair. Need 1 repeated pair to
        // be valid.
        for i in 0..digits.len() - 1 {
            if digits[i + 1] == digits[i] {
                return true;
            }
        }

        false
    }

    // This is used for part 2.
    fn is_valid3(digits: &[u8]) -> bool {
        for i in 0..digits.len() - 1 {
            if digits[i + 1] < digits[i] {
                return false;
            }
        }

        let mut this_run_length = 1;
        let mut this_run_digit = digits[0];

        for i in 1..digits.len() {
            if digits[i] == this_run_digit {
                this_run_length += 1;
            } else {
                // Run is at an end, was it of length 2?
                if this_run_length == 2 {
                    return true;
                } else {
                    this_run_length = 1;
                    this_run_digit = digits[i];
                }
            }
        }

        this_run_length == 2
    }
}

impl Iterator for ConstrainedRange {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if self.min > self.max {
            return None;
        }

        while self.min <= self.max {
            let t = self.min;
            self.min += 1;
            // Simple, brute force check rather than incrementing the digits
            // vec, which can be done 'cleverly' to skip some values.
            let digits = Self::num_to_digits(t);
            if Self::is_valid3(&digits) {
                return Some(t);
            }
        }

        return None;
    }
}


fn main() {
    let range = ConstrainedRange::new(264793, 803935);

    let num_valid_passwords = range.count();
    // 966 is the answer for the first part.
    // 628 is the answer for the second part.
    println!("Num valid passwords = {}", num_valid_passwords);
}

#[cfg(test)]
mod tests {
    use super::ConstrainedRange;

    #[test]
    pub fn test_num_to_digits() {
        assert_eq!(ConstrainedRange::num_to_digits(0), vec![0]);
        assert_eq!(ConstrainedRange::num_to_digits(1), vec![1]);
        assert_eq!(ConstrainedRange::num_to_digits(10), vec![1, 0]);
        assert_eq!(ConstrainedRange::num_to_digits(12), vec![1, 2]);
    }
}