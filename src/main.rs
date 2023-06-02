use lazy_static::lazy_static;

use rand::Rng;

lazy_static! {
    static ref WORD_LIST: Vec<char> = generate_word_list();
}

fn main() {
    println!("{}", generate_rand_word(10));
    println!("{}", generate_rand_word_with_random_length(2, 10));
}


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

fn init_population(population_size: usize, hard_mode: bool) -> Vec<String> {
    let mut population = Vec::new();
    for _ in 0..population_size {
        if hard_mode {
            population.push(generate_rand_word_with_random_length(2, 10));
        } else {
            population.push(generate_rand_word(10));
        }
    }
    population
}

fn get_pop_score(pop: Vec<char>, target: String) -> i32 {
    let mut score: i32 = 0;
    if pop.len() == target.len() {
        score = 50;
    } else {
        score = 0;
    }

    for (i, letter) in pop.iter().enumerate() {
        if target.contains(*letter) {
            score += 5;
        }
        match target.chars().nth(i) {
            Some(c) if *letter == c => score += 40,
            None => score -= 10,
            _ => (),
        }
    }
    score
}

fn get_best_score(target: String) -> i32 {
    let target_vec: Vec<char> = target.chars().collect();
    get_pop_score(target_vec, target)
}

fn make_child(parent1: Vec<char>, parent2: Vec<char>, mutation_rate: f32) -> Vec<char> {
    let mut rng = rand::thread_rng();
    let crosspoint = rng.gen_range(0..parent1.len());
    let vec1 = parent1[..crosspoint].to_vec();
    let vec2 = parent2[crosspoint..].to_vec();
    let mut child = vec![];
    child.extend(vec1);
    child.extend(vec2);

    for i in ..child.len() {
        if rng.gen_range(0.0..1.0) < mutation_rate {
            _;
        }
    }
    child
}
