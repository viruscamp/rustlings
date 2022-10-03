// iterators4.rs
// Execute `rustlings hint iterators4` or use the `hint` watch subcommand for a hint.


pub fn factorial2(num: u64) -> u64 {
    // Complete this function to return the factorial of num
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.

    use std::convert::TryInto;

    struct Fac{
        index: u64, 
        value: u64,
    };
    impl Iterator for Fac {
        type Item = u64;
        fn next(&mut self) -> Option<Self::Item> {
            let value = self.value;
            *self = Fac{ index: self.index + 1, value: self.value * (self.index + 1) };
            Some(value)
        }
    }

    let mut fac = Fac{ index: 0, value: 1};
    fac.nth(num.try_into().unwrap()).unwrap()
}

fn factorial(num: u64) -> u64 {
    (0..num).fold(1, |acc, x| acc * (x+1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
