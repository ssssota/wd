pub fn wd(words: Vec<String>, number: Vec<u64>) -> Vec<String> {
    words
        .iter()
        .enumerate()
        .filter_map(|(i, word)| number.contains(&(i as u64 + 1)).then(|| word.clone()))
        .collect()
}
