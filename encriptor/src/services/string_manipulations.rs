pub fn divide_words(words: Vec<&str>, num_groups: usize) -> Vec<Vec<&str>> {
    if num_groups == 0 {
        panic!("num_groups cannot be zero");
    }

    let group_size = words.len() / num_groups;
    let mut remainder = words.len() % num_groups;

    let mut divided_words: Vec<Vec<&str>> = Vec::with_capacity(num_groups);
    let mut iter = words.iter();

    for _ in 0..num_groups {
        let mut current_group_size = group_size;
        if remainder > 0 {
            current_group_size += 1;
            remainder -= 1;
        }

        let current_group: Vec<&str> = iter.by_ref().take(current_group_size).cloned().collect();
        divided_words.push(current_group);
    }

    return divided_words;
}
