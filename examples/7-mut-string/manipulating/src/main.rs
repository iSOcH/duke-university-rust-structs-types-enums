fn main() {
    let sentence = "the quick brown fox jumps over the lazy dog".to_string();
    // Use slicing to get the first three characters of the sentence
    println!("{}", &sentence[0..=4]);

    // concatenate using format!
    let description = format!("Title: Quick story\n{}", sentence);
    println!("{}", description);

    // iterate over the characters in the sentence
    // for c in sentence.chars() {
    //     match c {
    //         'a' | 'e' | 'i' | 'o' | 'u' => println!("Got a vowel!"),
    //         _ => continue,
    //     }
    // }

    // Split and collect into a vector
    //let words: Vec<&str> = sentence.split_whitespace().collect();
    let words = sentence.split(' ').collect::<Vec<&str>>();
    println!("{:?}", words);

    let reversed = sentence.chars().rev().collect::<String>();
    println!("{}", reversed);

    let vowel_count = count_vowels(&sentence);
    println!("Vowel count: {vowel_count}");

    let longest_word = longest_word("");
    println!("Longest word: {longest_word}");
}

fn count_vowels(input: &str) -> usize {
    input
        .chars()
        .filter_map(|c| {
            if let 'a' | 'b' | 'e' | 'i' | 'o' | 'u' = c {
                Some(c)
            } else {
                None
            }
        })
        .map(|v| {println!("{v}"); v})
        .count()
}

fn longest_word(input: &str) -> &str {
    let words = input.split_ascii_whitespace();
    let maybe_longest_word = words.max_by(|word1, word2| word1.len().cmp(&word2.len()));

    // max_by returns None if the iterator is empty, in that case (input == "") we can fall back to input
    maybe_longest_word.unwrap_or(input)
}

/*
What is the purpose of slicing a string? In the provided code, what would be the result of uncommenting and executing the line `println!("{}", &sentence[0..=4]);`?
    First five characters of `sentence` would be printed

How does the `format!` macro differ from simple string concatenation using the `+` operator? When would you choose to use `format!` instead of concatenation?
    Supports more compact concatenation and also format strings.
 */