pub fn raindrops(n: u32) -> String {
    let mut s = String::new();
    if n % 3 == 0 {
        s.push_str("Pling")
    } else if n % 5 == 0 {
        s.push_str("Plang")
    } else if n % 7 == 0 {
        s.push_str("Plong")
    } else {
        s = n.to_string()
    }
    s
}
