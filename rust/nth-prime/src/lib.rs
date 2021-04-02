// pub fn nth(n: u32) -> u32 {
//     let mut p = vec![];
//     p.iter().map(f)
// }

use std::fmt::Display;

// let v: Vec = vec![1, 2, 3, 4];

fn slice<T: Display>(v: Vec<T>) -> () {
    match v.as_slice() {
        [] => println!("empty"),
        [first, .., last] => println!("first: {}, last: {}", first, last),
        [_] => println!("fewer than 2 elements in vec!"),
    }
}
