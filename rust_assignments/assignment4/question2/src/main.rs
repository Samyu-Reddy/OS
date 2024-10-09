fn most_frequent_word(text: &str) -> (String, usize) {
    let mut words: Vec<&str> = text.split_whitespace().collect(); 
    let mut max_word = String::new();
    let mut max_count = 0;

    while let Some(word) = words.pop() {
        let mut count = 1;

        for &w in words.iter() {
            if w == word {
                count += 1;
            }
        }

        if count > max_count {
            max_word = word.to_string();
            max_count = count;
        }
    }
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
    let (word, count) = most_frequent_word(text);
    println!("Most frequent word: \"{}\" ({} times)", word, count);
}

