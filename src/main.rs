use rand::{Rng};
use ndarray::{Array, Dim};
struct PublicKey {
    a: Array<i128, Dim<[usize; 2]>>, // 2D array of f64
    b: Array<i128, Dim<[usize; 2]>>, // 2D array of f64
}

struct SecretKey(i128);


struct Ciphertext {
    u: i128, // 2D array of f64
    v: i128, // 2D array of f64
}

const Q: i128 = 97;

fn keygen() -> (PublicKey, SecretKey) {
    let n  = 20;
    let mut rng = rand::thread_rng();
    let s: i128 = rng.gen_range(0..10000);
    let a = Array::from_shape_fn((n, 1), |_|  rng.gen_range(0..Q));
    let e = Array::from_shape_fn((n, 1), |_|  rng.gen_range(0..4));
    let b  = (a.clone() * s % Q) + e % Q;

    (PublicKey{a,b}, SecretKey(s))
}

fn encrypt(pk: &PublicKey, m: i128) -> Ciphertext{

    let u  = pk.a.sum() % Q;
    let v =  (pk.b.sum() % Q + Q/2*m) % Q;

    Ciphertext {u, v}
}

fn decrypt(sk: &SecretKey, ciphertext: Ciphertext) -> i32 {
    let mut res =  (ciphertext.v - sk.0*ciphertext.u) % Q;
    if res < 0 {
        res = Q + res;
    } 

    if res < Q/2 {
        0
    } else {
        1
    }
}

fn main() {
    let (pk, sk) = keygen();
    let ciphertext1 = encrypt(&pk, 1);
    let ciphertext2 = encrypt(&pk, 1);
    let plain1 = decrypt(&sk, ciphertext1);
    let plain2 = decrypt(&sk, ciphertext2);

    println!("Decrypted {}", plain1);
    println!("Decrypted {}", plain2);
}
