use std::time::Instant;
mod word_gen;
mod generative_algo;
use lazy_static::lazy_static;

lazy_static! {
    // Easy mode
    pub static ref TARGET: String = word_gen::generate_rand_word(10);
    // Hard mode
    //pub static ref TARGET: String = word_gen::generate_rand_word_with_random_length(2, 10);

    pub static ref WORD_LIST: Vec<char> = word_gen::make_char_list();
}

pub const GENERATIONS: u32 = 100000;
pub const POPULATION_SIZE: u16 = 100;
pub const MUTATION_RATE: f32 = 0.025;
pub const HARD_MODE: bool = false;


fn main() {
    let start_time = Instant::now();
    let (result, gens) = generative_algo::generative_algo();
    let end_time = Instant::now();

    let elapsed_time = end_time - start_time;

    println!("Target: {}", *TARGET);
    println!("Result: {}", result.iter().collect::<String>());
    println!("After: {} generations", &gens);
    println!("Elapsed time: {:?}", elapsed_time);
}
