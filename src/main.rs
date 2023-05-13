use rand::{Rng, random, distributions::{Uniform}};
use ndarray::Array;

fn generate_random_number(min: u32, max: u32) -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..=max)
}

fn test_random_matrix() -> (){
    let nrows = 5;
    let ncols = 5;

    let a = Array::from_shape_fn((nrows, ncols), |_| random::<f64>());

    println!("MATRIX {:?}", a);
}

fn main() {
    fizzbuzz();
    let range = Uniform::from(0..20);

    let mut a = Vec::new();

    let mut values: Vec<u64> = rand::thread_rng().sample_iter(&range).take(100).collect();
    a.push(values);

    values = rand::thread_rng().sample_iter(&range).take(100).collect();
    a.push(values);


    //println!("Vectorized: {:?}", values);
    println!("Matrix: {:?}", a);
    test_random_matrix();
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