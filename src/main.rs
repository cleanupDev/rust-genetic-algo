use lazy_static::lazy_static;
use std::time::Instant;

use rand::{Rng, seq::SliceRandom};

lazy_static! {
    static ref WORD_LIST: Vec<char> = generate_word_list();
}

fn generate_rand_word(number: i32) -> String {
    let mut rng = rand::thread_rng();
    let mut word = String::new();
    for _ in 0..number {
        let c: char = WORD_LIST[rng.gen_range(0..WORD_LIST.len())];
        word.push(c);
    }
    word
}

// generate a word with a random lenght between 2 and 10 from a list of possible characters
// and returns it as a String

fn generate_rand_word_with_random_length(lower_range: i8, upper_range: i8) -> String {
    let mut rng = rand::thread_rng();
    let mut word = String::new();
    let length = rng.gen_range(lower_range..=upper_range);
    for _ in 0..length {
        let c = WORD_LIST[rng.gen_range(0..WORD_LIST.len())];
        word.push(c);
    }
    word
}

// a list containing all the possible characters that can be used to generate a word

fn generate_word_list() -> Vec<char> {
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

fn init_population(population_size: i32, hard_mode: bool) -> Vec<Vec<char>> {
    let mut population: Vec<Vec<char>> = Vec::new();
    for _ in 0..population_size {
        if hard_mode {
            let new_word: String = generate_rand_word_with_random_length(2, 10);
            population.push(new_word.chars().collect());
        } else {
            let new_word = generate_rand_word(10);
            population.push(new_word.chars().collect());
        }
    }
    population
}

fn get_pop_score(pop: &Vec<char>, target: &String) -> i32 {
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

fn get_best_score(target: &String) -> i32 {
    let target_vec: Vec<char> = target.chars().collect();
    get_pop_score(&target_vec, target)
}

fn make_child(parent1: &Vec<char>, parent2: &Vec<char>, mutation_rate: f32) -> Vec<char> {
    let mut rng = rand::thread_rng();
    let crosspoint = rng.gen_range(0..(parent1.len()/2));
    let vec1 = parent1[..crosspoint].to_vec();
    let vec2 = parent2[crosspoint..].to_vec();
    let mut child = vec![];
    child.extend(vec1);
    child.extend(vec2);

    for i in 0..child.len() {
        if rng.gen_range(0.0..1.0) < mutation_rate {
            child[i] = WORD_LIST[rng.gen_range(0..WORD_LIST.len())];
        }
    }

    if rng.gen_range(0.0..1.0) < (mutation_rate / 10.0) {
        let single_char: char = WORD_LIST[rng.gen_range(0..WORD_LIST.len())];
        child.push(single_char);
    }
    if rng.gen_range(0.0..1.0) < (mutation_rate / 10.0) {
        child.remove(0);
    }
    child
}

fn generative_algo(target: &String, generations: i32, population_size: i32, mutation_rate: f32, hard_mode: bool) -> (Vec<char>, i32){
    let mut rng = rand::thread_rng();
    let mut population = init_population(population_size, hard_mode);
    population.sort_by(|a, b| get_pop_score(&a.to_vec(), &target).cmp(&get_pop_score(&b.to_vec(), &target)).reverse());

    let top_performer_range = population.len() / 5;
    let target_score = get_best_score(&target);
    let mut apex_pops: Vec<Vec<char>> = population[..top_performer_range].to_vec();

    for _gen in 0..generations {
        let mut next_generation: Vec<Vec<char>> = apex_pops;

        while next_generation.len() < population.len() {
            let mut iter = population.choose_multiple(&mut rng, 2);
            let parent1 = iter.next().unwrap();
            let parent2 = iter.next().unwrap();

            next_generation.push(make_child(&parent1, &parent2, mutation_rate));
        }

        next_generation.sort_by(|a, b| get_pop_score(&a.to_vec(), &target).cmp(&get_pop_score(&b.to_vec(), &target)).reverse());
        population = next_generation;

        apex_pops = population[..top_performer_range].to_vec();

        if get_pop_score(&population[0], &target) == target_score {
            return (population.get(0).unwrap().to_vec(), _gen);
        }


    }
    return (population.get(0).unwrap().to_vec(), generations);
}


fn main() {
    let target = generate_rand_word(10);
    let generations = 10000;
    let population_size = 100;
    let mutation_rate = 0.025;
    let hard_mode = false;

    let start_time = Instant::now();
    let (result, gens) = generative_algo(&target, generations, population_size, mutation_rate, hard_mode);
    let end_time = Instant::now();

    let elapsed_time = end_time - start_time;



    println!("Target: {}", &target);
    println!("Result: {}", result.iter().collect::<String>());
    println!("After: {} generations", gens);
    println!("Elapsed time: {:?}", elapsed_time);

}
