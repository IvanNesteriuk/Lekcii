use std::collections::HashMap;

fn char_indexes(text: String) -> HashMap<char, Vec<usize>> {
    let mut outcome: HashMap<char, Vec<usize>> = HashMap::new();
    text.chars()
        .into_iter()
        .zip(1..)
        .filter(|&(c, _)| c.is_alphabetic())
        .for_each(|(c, idx)| {
            outcome
                .entry(c.to_ascii_lowercase())
                .or_insert_with(Vec::new)
                .push(idx);
        });

    outcome
}

#[test]
fn char_indexes_test() {
    let indexes = char_indexes(String::from("Hello, world!"));
    println!("{:?}", indexes);
}
