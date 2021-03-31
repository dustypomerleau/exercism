#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(a: &[T], b: &[T]) -> Comparison {
    match a == b {
        true => Comparison::Equal,
        _ => match a {
            [] => Comparison::Sublist,
            _ => match a.len() < b.len() {
                true => match is_sublist(a, b) {
                    true => Comparison::Sublist,
                    _ => Comparison::Unequal,
                },
                _ => match is_sublist(b, a) {
                    true => Comparison::Superlist,
                    _ => Comparison::Unequal,
                },
            },
        },
    }
}

fn is_sublist<T: PartialEq>(x: &[T], y: &[T]) -> bool {
    let mut sub = false;
    for i in 0..x.len() + 1 {
        if x == &y[i..i + x.len()] {
            sub = true;
            break;
        }
    }
    sub
}
