use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let mut c: Vec<char> = word.to_lowercase().chars().into_iter().collect();
    c.sort();

    let mut h = HashSet::new();

    for s in possible_anagrams {
        let s = *s;
        let mut pc: Vec<char> = s.to_lowercase().chars().into_iter().collect();
        pc.sort();
        if pc == c {
            h.insert(s);
        }
    }

    h
}
