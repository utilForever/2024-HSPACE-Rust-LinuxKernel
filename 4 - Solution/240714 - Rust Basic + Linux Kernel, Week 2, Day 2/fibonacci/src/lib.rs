pub fn fib(n: u32) -> u32 {
    if n <= 2 {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fib_1() {
        let ret = fib(1);
        assert_eq!(ret, 1);
    }

    #[test]
    fn fib_2() {
        let ret = fib(2);
        assert_eq!(ret, 1);
    }

    #[test]
    fn fib_5() {
        let ret = fib(5);
        assert_eq!(ret, 5);
    }

    #[test]
    fn fib_20() {
        let ret = fib(20);
        assert_eq!(ret, 6765);
    }
}
