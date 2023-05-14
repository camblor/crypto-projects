use rand::{Rng, distributions::{Uniform}};
use ndarray::{Array, Dim};
struct PublicKey {
    a: Array<i128, Dim<[usize; 2]>>, // 2D array of f64
    b: Array<i128, Dim<[usize; 2]>>, // 2D array of f64
}

struct SecretKey(Array<i128, Dim<[usize; 2]>>);

struct Ciphertext {
    u: Array<i128, Dim<[usize; 2]>>, // 2D array of f64
    v: Array<i128, Dim<[usize; 2]>>, // 2D array of f64
}

const Q: i128 = 13;

fn keygen() -> (PublicKey, SecretKey){
    let nrows = 7;
    let ncols = 4;
    let mut rng = rand::thread_rng();

    let a = Array::from_shape_fn((nrows, ncols), |_|  rng.gen_range(0..Q));
    let s_a = Array::from_shape_fn((ncols, 1), |_|  rng.gen_range(0..Q));
    let e_a = Array::from_shape_fn((nrows, 1), |_|  rng.gen_range(0..=1));

    // println!("A {:?}", a);
    // println!("sA {:?}", s_a);
    // println!("eA {:?}", e_a);

    let result = (a.dot(&s_a).mapv(|x| x % Q) + e_a).mapv(|x| x % Q);
    //println!("result {:?}", result);

    let pk = PublicKey {
        a,
        b: result
    };

    let sk = SecretKey(s_a);

    (pk, sk)
}

fn encrypt(pk: PublicKey, m: i128) -> Ciphertext{
    let mut rng = rand::thread_rng();
    let r = Array::from_shape_fn((1,7), |_|  rng.gen_range(0..=1));
    let u = r.dot(&pk.a).mapv(|x| x % Q);
    let v = (r.dot(&pk.b).mapv(|x| x % Q) + 1 * 13 / 2).mapv(|x| x % Q);
    println!("Encrypt  {:?}", m);
    //println!("u {:?}", u);
    //println!("v {:?}", v);
    let ciphertext = Ciphertext {
        u,
        v
    };

    ciphertext
}

fn decrypt(sk: SecretKey, ciphertext: Ciphertext) -> () {
    let w = ciphertext.u.dot(&sk.0).mapv(|x| x % Q);
    println!("ciphertext.v {:?}", ciphertext.v);
    println!("w {:?}", w);
    let w: i128 = w[[0, 0]];
    let cipherv: i128 = ciphertext.v[[0, 0]];
    let result: i128;
    if cipherv >= w {
        result = ((cipherv - w) % 13).try_into().unwrap();
    } else {
        result = (((Q-1) - (w - cipherv)) % 13).try_into().unwrap();
    }
    // w = ciphertext.v.sub(w);
    // w = w.mapv(|x| x % Q);
    println!("result {:?}", result);
}

fn main() {
    let range = Uniform::from(0..20);
    
    let mut a = Vec::new();

    let mut values: Vec<u64> = rand::thread_rng().sample_iter(&range).take(100).collect();
    a.push(values);

    values = rand::thread_rng().sample_iter(&range).take(100).collect();
    a.push(values);


    //println!("Vectorized: {:?}", values);
    //println!("Matrix: {:?}", a);
    let (pk, sk) = keygen();

    let m: i128 = 0;



    let ciphertext = encrypt(pk, m);
    decrypt(sk, ciphertext);
}
