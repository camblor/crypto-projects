use rand::{Rng, distributions::{Standard, Uniform}};

fn generate_random_number(min: u32, max: u32) -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..=max)
}

fn main() {
    fizzbuzz();
    let range = Uniform::from(0..20);
    let values: Vec<u64> = rand::thread_rng().sample_iter(&range).take(100).collect();
    println!("Vectorized: {:?}", values);
}

fn fizzbuzz() -> () {
    let mut i = 0;
    let mut vec = Vec::new();

    loop {
        if i > 10 {
            break;
        }
        vec.push(generate_random_number(1, 100));
        i += 1;
    }
    println!("Vector: {:?}", vec);
}