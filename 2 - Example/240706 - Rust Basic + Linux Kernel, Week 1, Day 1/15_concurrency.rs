// use std::rc::Rc;
// use std::thread;

// fn main() {
//     let rc1 = Rc::new("Hyundai".to_string());
//     let rc2 = rc1.clone();

//     thread::spawn(move || {
//         // Error
//         rc2.clone();
//     });

//     rc1.clone();
// }

use std::sync::Arc;
use std::thread;

fn main() {
    let arc1 = Arc::new("Hyundai".to_string());
    let arc2 = arc1.clone();

    thread::spawn(move || {
        // No error
        let _ = arc2.clone();
    });

    let _ = arc1.clone();
}
