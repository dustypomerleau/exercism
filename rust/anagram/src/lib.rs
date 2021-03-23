use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let wl = word.to_lowercase();
    let ws = get_sorted(&wl);
    possible_anagrams
        .iter()
        .filter(|a| {
            let al = a.to_lowercase();
            al.len() == wl.len() && al != wl && get_sorted(&al) == ws
        })
        .copied()
        .collect()
}

fn get_sorted(word: &str) -> Vec<char> {
    let mut w: Vec<char> = word.chars().collect();
    w.sort_unstable();
    w
}
