#[cfg(test)]
mod tests {
    use crate::iterator_tools::ParallelPosition;

    #[test]
    fn find_test_1() {
        let result = ParallelPosition::find(
            0..,
            |x| x == 10_000,
            28,
            1,
        );
        assert_eq!(result, Some(10_000));
    }

    #[test]
    fn find_test_2() {
        let result = ParallelPosition::find(
            0..,
            |x| x == 0,
            100,
            50,
        );
        assert_eq!(result, Some(0));
    }

    #[test]
    fn find_test_3() {
        let result = ParallelPosition::find(
            0..,
            |x| x == 100_000,
            5,
            100,
        );
        assert_eq!(result, Some(100_000));
    }

    #[test]
    fn find_test_4() {
        let result = ParallelPosition::find(
            1..100000,
            |x| x == 0,
            5,
            100,
        );
        assert_eq!(result, None);
    }
}
