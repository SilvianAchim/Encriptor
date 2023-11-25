pub fn divide_words(words: Vec<&str>, num_groups: usize) -> Vec<Vec<&str>> {
    let mut divided_words: Vec<Vec<&str>> = vec![Vec::new(); num_groups];

    for (index, &word) in words.iter().enumerate() {
        divided_words[index % num_groups].push(word);
    }

    return divided_words;
}