use lazy_static::lazy_static;
mod generate_word;
use generate_word::generate_word_list;


lazy_static! {
    static ref WORD_LIST: Vec<char> = generate_word_list();
}

fn init_population(population_size: usize, hard_mode: bool) -> Vec<String> {
    let mut population = Vec::new();
    for _ in 0..population_size {
        if hard_mode {
            population.push(generate_rand_word_with_random_length());
        } else {
            population.push(generate_rand_word());
        }
    }
    population
}

fn get_pop_score(pop: Vec<char>, target: String) -> i32 {
    if pop.len() == target.len() {
        let mut score = 50;
    } else {
        let mut score = 0;
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
    let crosspoint: i8 = rand::thread_rng().gen_range(0..parent1.len());
    let mut child: Vec<String> = parent1[..crosspoint].to_vec() + parent2[crosspoint..].to_vec();

    for i in ..child.len() {
        if rng.gen_range(0.0..1.0) < mutation_rate {
            child[i] = ;
        }
    }
}