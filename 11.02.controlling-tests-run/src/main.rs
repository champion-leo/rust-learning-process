// run test without parallelism 
// cargo test -- --test-threads=1

// run test with print display
// cargo test -- --show-output

// run one specific test
// cargo test one_hundred
// running 1 test
// test tests::one_hundred ... ok

// run all test whose name contains 'add'
// cargo test add
// running 2 tests
// test tests::add_two_and_two ... ok
// test tests::add_three_and_two ... ok

// run tests marked as ignore
// cargo test -- --ignored
// running 1 test
// test tests::expensive_test ... ok

// run all tests (including test marked as ignore)
// cargo test -- --include-ignored

fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run
    }
}

fn main() {
    prints_and_returns_10(32);
}
