use mockall::{
    automock,
    mock
};

pub fn add(left: i64, right: i64) -> i64 {
    private_add(left, right)
}

/// Divides the first number by the second number. Panics if second number is 0
///
/// # Example
///
/// ```
/// assert_eq!(2, testing::divide(8, 4));
/// ```
pub fn divide(left: i64, right: i64) -> i64 {
    left / right
}

fn private_add(left: i64, right: i64) -> i64 {
    left + right
}

#[automock]
trait Calculator {
    fn add(&self, left: i64, right: i64) -> i64;
    fn subtract(&self, left: i64, right: i64) -> i64;
}

trait Beeper {
    fn beep(&self);
}

mock! {
    BeeperCalculator {}

    impl Calculator for BeeperCalculator {
        fn add(&self, left: i64, right: i64) -> i64;
        fn subtract(&self, left: i64, right: i64) -> i64;
    }

    impl Beeper for BeeperCalculator {
        fn beep(&self);
    }
}

#[cfg(test)]
mod tests {
    use super::{
        Beeper,
        Calculator,
        MockBeeperCalculator,
        MockCalculator,
        add,
        divide,
        private_add
    };
    use mockall::predicate::eq;

    #[test]
    fn assert() {
        let result = add(2, 2);
        assert!(result == 4);
    }

    #[test]
    fn assert_with_message() {
        let result = add(2, 2);
        assert!(result == 4, "Expected 4, actual value was {}", result);
    }

    #[test]
    fn assert_eq() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn assert_ne() {
        let result = add(2, 2);
        assert_ne!(result, 3);
    }

    #[test]
    #[ignore]
    fn ignored_failing_test_with_message() {
        let result = add(2, 2);
        assert_eq!(result, 3, "Expected 3, actual value was {}", result);
    }

    #[test]
    fn test_private_fn() {
        let result = private_add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    #[should_panic]
    fn test_panic() {
        divide(2, 0);
    }

    #[test]
    #[should_panic(expected="divide by zero")]
    fn test_panic_with_message_matching() {
        divide(2, 0);
    }

    #[test]
    fn test_mock_expectations_are_met() {
        let mut mock = MockCalculator::new();
        mock.expect_add()
            .with(eq(1), eq(2))
            .return_const(3);

        assert_eq!(mock.add(1, 2), 3);
    }

    #[test]
    fn test_mock_multiple_traits() {
        let mut mock = MockBeeperCalculator::new();
        mock.expect_add().return_const(3);
        mock.expect_beep().return_const(());

        mock.add(1, 2);
        mock.beep();
    }
}
