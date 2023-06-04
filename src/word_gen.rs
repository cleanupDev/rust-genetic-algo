use rand::Rng;
use super::WORD_LIST;


pub fn generate_rand_word(number: u8) -> String {
    let mut rng = rand::thread_rng();
    let mut word = String::new();
    for _ in 0..number {
        let c: char = WORD_LIST[rng.gen_range(0..WORD_LIST.len())];
        word.push(c);
    }
    word
}


pub fn generate_rand_word_with_random_length(lower_range: u8, upper_range: u8) -> String {
    let mut rng = rand::thread_rng();
    let mut word = String::new();
    let length = rng.gen_range(lower_range..=upper_range);
    for _ in 0..length {
        let c = WORD_LIST[rng.gen_range(0..WORD_LIST.len())];
        word.push(c);
    }
    word
}


pub fn make_char_list() -> Vec<char>{
    let mut char_list: Vec<char> = Vec::new();
    for i in 33..=126 {
        char_list.push(char::from(i));
    }
    char_list
}