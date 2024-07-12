fn main() {
    {
        let r;

        {
            let x = 1;
            r = &x;         // `x` does not live long enough
        }

        assert_eq!(*r, 1);  // Bad: Reads memory `x` used to occupy
    }
}
