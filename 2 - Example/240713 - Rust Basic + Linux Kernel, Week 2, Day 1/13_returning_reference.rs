fn main() {
    // v should have at least one element.
    fn smallest(v: &[i32]) -> &i32 {
        let mut s = &v[0];

        for r in &v[1..] {
            if *r < *s {
                s = r;
            }
        }

        s
    }

    let s;

    {
        let parabola = [9, 4, 1, 0, 1, 4, 9];
        s = smallest(&parabola);
    }

    assert_eq!(*s, 0); // Bad: Points to element of dropped array
}
