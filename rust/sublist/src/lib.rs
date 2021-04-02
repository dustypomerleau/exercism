mod trash;

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist2<T: PartialEq>(a: &[T], b: &[T]) -> Comparison {
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

pub fn sublist<T: PartialEq>(a: &[T], b: &[T]) -> Comparison {
    match a == b {
        true => Comparison::Equal,
        _ => match (a.len(), b.len()) {
            (0, 0) => Comparison::Equal,
            (0, _) => Comparison::Sublist,
            (_, 0) => Comparison::Superlist,
            (_, _) => match a.len() < b.len() {
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
    let y_win = y.windows(x.len());
    let mut sub = false;
    for w in y_win {
        if w == x {
            sub = true;
            break;
        }
    }
    sub
}
