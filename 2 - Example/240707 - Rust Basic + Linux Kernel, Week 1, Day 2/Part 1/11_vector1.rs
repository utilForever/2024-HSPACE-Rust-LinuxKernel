fn main() {
    let mut primes = vec![2, 3, 5, 7];

    assert_eq!(primes.iter().product::<i32>(), 210);

    primes.push(11);
    primes.push(13);

    assert_eq!(primes.iter().product::<i32>(), 30030);

    let mut pal = Vec::new();
    pal.push("step");
    pal.push("on");
    pal.push("no");
    pal.push("pets");

    assert_eq!(pal, vec!["step", "on", "no", "pets"]);

    let v: Vec<i32> = (0..5).collect();

    assert_eq!(v, [0, 1, 2, 3, 4]);

    // A palindrome!
    let mut palindrome = vec!["a man", "a plan", "a canal", "panama"];

    palindrome.reverse();

    // Reasonable yet disappointing:
    assert_eq!(palindrome, vec!["panama", "a canal", "a plan", "a man"]);
}
