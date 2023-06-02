// a function that randomly generates a word with length 10 from a list of possible characters
// and returns it as a String
use rand::Rng;

pub fn generate_rand_word(number: i32) -> String {
    let mut rng = rand::thread_rng();
    let mut word = String::new();
    for _ in 0..number {
        let c = rng.gen_range('a'..='z');
        word.push(c);
    }
    word
}

// generate a word with a random lenght between 2 and 10 from a list of possible characters
// and returns it as a String

pub fn generate_rand_word_with_random_length(lower_range: i8, upper_range: i8) -> String {
    let mut rng = rand::thread_rng();
    let mut word = String::new();
    let length = rng.gen_range(lower_range..=upper_range);
    for _ in 0..length {
        let c = rng.gen_range('a'..='z');
        word.push(c);
    }
    word
}

// a list containing all the possible characters that can be used to generate a word

pub fn generate_word_list() -> Vec<char> {
    let mut word_list = Vec::new();
    for c in 'a'..='z' {
        word_list.push(c);
    }
    for c in 'A'..='Z' {
        word_list.push(c);
    }
    for c in '0'..='9' {
        word_list.push(c);
    }
    let special_chars = vec![':', ';', '?', '!'];
    for c in special_chars {
        word_list.push(c);
    }
    word_list
}