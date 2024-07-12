fn main() {
    struct S<'a, 'b> {
        x: &'a i32,
        y: &'b i32,
    }

    // Same as "fn sum_r_xy<'a, 'b, 'c>(r: &'a i32, s: S<'b, 'c>) -> i32".
    fn sum_r_xy(r: &i32, s: S) -> i32 {
        r + s.x + s.y
    }

    // Same as "fn first_third<'a>(point: &'a [i32; 3]) -> (&'a i32, &'a i32)".
    fn first_third(point: &[i32; 3]) -> (&i32, &i32) {
        (&point[0], &point[2])
    }

    struct StringTable {
        elements: Vec<String>,
    }

    impl StringTable {
        // Same as "fn find_by_prefix<'a, 'b>(&'a self, prefix: &'b str) -> Option<&'a String>".
        fn find_by_prefix(&self, prefix: &str) -> Option<&String> {
            for i in 0..self.elements.len() {
                if self.elements[i].starts_with(prefix) {
                    return Some(&self.elements[i]);
                }
            }

            None
        }
    }
}
