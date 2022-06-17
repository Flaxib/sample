use std::collections::HashMap;
use std::collections::HashSet;

fn compute_histogram(word: &str) -> HashMap<char, i32> {
    let mut res = HashMap::<char, i32>::new();
    for c in word.chars() {
        res.insert(
            c,
            match res.get(&c) {
                Some(n) => n + 1,
                None => 1,
            },
        );
    }
    res
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word_insensitive = word.to_lowercase();
    let word_histogram = compute_histogram(&word_insensitive);

    let mut res = HashSet::<&'a str>::new();

    for i_word in possible_anagrams.iter() {
        if word_insensitive == i_word.to_lowercase() {
            continue;
        }

        if word_histogram == compute_histogram(&i_word.to_lowercase()) {
            res.insert(i_word);
        }
    }

    res
}
