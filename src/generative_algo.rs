use super::word_gen;
use rand::{Rng, seq::SliceRandom};

use super::WORD_LIST;
use super::TARGET;
use super::GENERATIONS;
use super::POPULATION_SIZE;
use super::MUTATION_RATE;
use super::HARD_MODE;


fn init_population() -> Vec<Vec<char>> {
    let mut population: Vec<Vec<char>> = Vec::new();
    for _ in 0..POPULATION_SIZE {
        if HARD_MODE {
            let new_word: String = word_gen::generate_rand_word_with_random_length(2, 10);
            population.push(new_word.chars().collect());
        } else {
            let new_word = word_gen::generate_rand_word(10);
            population.push(new_word.chars().collect());
        }
    }
    population
}


fn get_pop_score(pop: &Vec<char>) -> i32 {
    let mut score: i32;
    if pop.len() == TARGET.len() {
        score = 50;
    } else {
        score = 0;
    }

    for (i, letter) in pop.iter().enumerate() {
        if TARGET.contains(*letter) {
            score += 5;
        }
        match TARGET.chars().nth(i) {
            Some(c) if *letter == c => score += 40,
            None => score -= 10,
            _ => (),
        }
    }
    score
}


fn get_best_score() -> i32 {
    let target_vec: Vec<char> = TARGET.chars().collect();
    get_pop_score(&target_vec)
}


fn make_child(parent1: &Vec<char>, parent2: &Vec<char>) -> Vec<char> {
    let mut rng = rand::thread_rng();
    let crosspoint: usize;
    if parent1.len() != parent2.len() {
        crosspoint = rng.gen_range(0..(if parent1.len() < parent2.len() {parent1.len()} else {parent2.len()}));
    } else {
        crosspoint = rng.gen_range(0..(parent1.len()));

    }
    
    let mut child = vec![];
    child.extend(parent1[..crosspoint].to_vec());
    child.extend(parent2[crosspoint..].to_vec());

    for i in 0..child.len() {
        if rng.gen_range(0.0..1.0) < MUTATION_RATE {
            child[i] = WORD_LIST[rng.gen_range(0..WORD_LIST.len())];
        }
    }

    if rng.gen_range(0.0..1.0) < (MUTATION_RATE / 10.0) {
        child.push(WORD_LIST[rng.gen_range(0..WORD_LIST.len())]);
    }
    if rng.gen_range(0.0..1.0) < (MUTATION_RATE / 10.0) {
        child.remove(0);
    }
    child
}


pub fn generative_algo() -> (Vec<char>, u32){
    let mut rng = rand::thread_rng();
    let mut population = init_population();
    population.sort_by(|a, b| get_pop_score(&b.to_vec()).cmp(&get_pop_score(&a.to_vec())));

    let top_performer_range = population.len() / 5;
    let target_score = get_best_score();
    let mut apex_pops: Vec<Vec<char>> = population[..top_performer_range].to_vec();

    for _gen in 0..GENERATIONS {
        let mut next_generation: Vec<Vec<char>> = apex_pops;

        while next_generation.len() < population.len() {
            let mut iter = population.choose_multiple(&mut rng, 2);

            next_generation.push(make_child(&iter.next().unwrap(), &iter.next().unwrap()));
        }

        next_generation.sort_by(|a, b| get_pop_score(&b.to_vec()).cmp(&get_pop_score(&a.to_vec())));
        population = next_generation;

        apex_pops = population[..top_performer_range].to_vec();

        if get_pop_score(&population[0]) == target_score {
            return (population.get(0).unwrap().to_vec(), _gen);
        }


    }
    return (population.get(0).unwrap().to_vec(), GENERATIONS);
}
