use std::collections::HashMap;

#[allow(dead_code)]
fn check_cache<F>(func: F) -> impl FnMut(i64) -> (i64, bool)
where
    F: Fn(i64) -> i64,
{
    let mut cache = HashMap::new();

    move |x| {
        if let Some(&ret) = cache.get(&x) {
            (ret, true)
        } else {
            let ret = func(x);
            cache.insert(x, ret);

            (ret, false)
        }
    }
}

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
