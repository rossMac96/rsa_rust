use num_bigint::{BigUint, RandomBits};
use num_traits::{One,Zero,pow};
use rand::Rng;

fn main() {
    let (p,q,n) = setup_cipher_params(256);
    println!("{}, {}", p, p.bits());
    println!("{}, {}", q, q.bits());
    println!("{}, {}", n, n.bits());
    let e = BigUint::from(65537 as u32);
    let totient_n = totient_of_n(p,q);
    println!("{}",totient_n);
}

fn m_i(a: BigUint, b: BigUint) -> BigUint {
    let mut x = BigUint::zero();
    let mut y = BigUint::zero();
    let g = gcd(a.clone(), b.clone(), &mut x, &mut y);
    if g == BigUint::one() {
        (x%b.clone() + b.clone()) % b
    } else {
        BigUint::zero()
    }
}

fn gcd(a: BigUint, b: BigUint, x: &mut BigUint, y: &mut BigUint) -> BigUint {
    if a == BigUint::zero() {
        *x = BigUint::zero();
        *y = BigUint::one();
        b
    } else {
        let mut x1 = BigUint::zero();
        let mut y1 = BigUint::zero();
        let g = gcd(b.clone() % a.clone(), a.clone(), &mut x1, &mut y1);
        *x = y1 - (b / a) * x1.clone();
        *y = x1;
        g
    }
}

fn totient_of_n(p: BigUint, q: BigUint) -> BigUint {
    let totient = (p-BigUint::one()) * (q-BigUint::one());
    totient
}

fn setup_cipher_params(size: usize) -> (BigUint, BigUint, BigUint) {
    let mut p = BigUint::zero();
    let mut q = BigUint::zero();
    while p == q {
        //throw out both and try again
        p = get_rand_prime(&size/2);
        q = get_rand_prime(&size/2);
    }
    let n = &p*&q;
    (p,q,n)
}

fn get_rand_prime(size:usize) -> BigUint {
    let mut rng = rand::thread_rng();
    let mut found = false;
    let mask = get_bit_mask(size);
    let mut prime_candidate = BigUint::zero();
    while !found {
        prime_candidate = rng.sample(RandomBits::new(size));
        prime_candidate |= &mask;
        if is_prime(&prime_candidate) {
            found = true;
        }
    }
    prime_candidate
}

fn get_bit_mask(size:usize) -> BigUint {
    let mut bitmask = pow(BigUint::from(2 as u8),size-1);
    bitmask ^= pow(BigUint::from(2 as u8), size-2);
    bitmask ^= BigUint::one();
    bitmask
}


fn is_prime(input: &BigUint) -> bool {
    //run probe tests, if one fails input is not prime
    //need to see why this fails if the input is one of the probes
    let exp = input - &BigUint::one();
    let mut prime = true;
    let probes = [2,3,5,7,11,13,17,43,683];
    for probe_idx in 0..probes.len() {
        let mod_exp = BigUint::modpow(&BigUint::from(probes[probe_idx] as u32),&exp,&input);
        if !BigUint::is_one(&mod_exp) {
            prime = false;
            break;
        }
    }
    return prime
}