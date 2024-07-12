// Attempt #1
// This code has several problems, and doesn't compile.
// static mut STASH: &i32;     // Every static must be initialized.
//                             // Mutable statics are inherently not thread-safe.
// fn f(p: &i32) {
//     STASH = p;              // For these reasons, you may access a mutable static only within an unsafe block.
// }

// Attempt #2
// static mut STASH: &i32 = &128;

// fn f<'a>(p: &'a i32)
// fn f(p: &i32) {
//     // Still not good enough
//     unsafe {
//         STASH = p;              // Since STASH lives for the program's entire execution,
//     }                           // the reference type it holds must have a lifetime of the same length
// }

// Attempt #3
static mut STASH: &i32 = &10;

fn f(p: &'static i32) {
    unsafe {
        STASH = p;
    }
}

fn main() {
    // ...
}
