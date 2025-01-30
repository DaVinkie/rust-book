fn main() {
    let word = String::from("Hello my friend.");
    let n = first_word(&word);
    println!("The ending index of the first word is: {}", n);
    println!("The first word is: {}", &word[..n]);
    // De .. syntax is inclusief eerst en laatste element van een range, dus [0..2] == [..2] en [3..len()] == [3..]
    let word_1_s = first_word_sliced(&word);
    let word_2_s = second_word_sliced(&word);
    println!("The first sliced word is: {word_1_s}");
    println!("The second sliced word is: {word_2_s}");
}


fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}


fn first_word_sliced(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}


fn second_word_sliced(s: &str) -> &str {
    let bytes = s.as_bytes();
    let mut first_space = 0;
    let mut space_count = 0;
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            space_count += 1;
            if space_count == 2 {
                return &s[first_space+1..i]
            }
            first_space = i;
        }
    }
    &s[..]
}