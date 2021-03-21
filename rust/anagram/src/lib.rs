use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let c: Vec<char> = word.to_lowercase().chars().into_iter().collect();
    let mut cs = c.clone();
    cs.sort_unstable();

    let mut h = HashSet::new();

    for s in possible_anagrams {
        let s = *s;
        let mut p: Vec<char> = s.to_lowercase().chars().into_iter().collect();
        if p == c {
            break;
        }
        p.sort_unstable();
        if p == cs {
            h.insert(s);
        }
    }

    h
}
