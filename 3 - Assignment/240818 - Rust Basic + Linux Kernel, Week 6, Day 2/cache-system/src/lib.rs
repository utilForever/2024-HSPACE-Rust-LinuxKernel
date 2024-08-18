// TODO: Implement a function "check_cache" that takes a closure and returns a new closure.

#[cfg(test)]
mod tests {
    use crate::check_cache;

    #[test]
    fn test_compute_and_cache() {
        let mut compute_func = check_cache(|x| x * x);

        assert_eq!(compute_func(4), (16, false)); // Compute: 16
        assert_eq!(compute_func(4), (16, true)); // Use cache: 16
        assert_eq!(compute_func(5), (25, false)); // Compute: 25
    }
}
