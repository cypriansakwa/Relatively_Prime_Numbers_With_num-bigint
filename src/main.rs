use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::str::FromStr;

// Function to calculate the greatest common divisor using the Euclidean algorithm
fn gcd(a: &BigUint, b: &BigUint) -> BigUint {
    if b.is_zero() {
        a.clone()
    } else {
        gcd(b, &(a % b))
    }
}

// Function to list all positive numbers less than n that are relatively prime to n
fn relatively_prime_numbers(n: &BigUint) -> Vec<BigUint> {
    let mut result = Vec::new();
    let one = BigUint::one();
    let mut i = BigUint::one();

    while &i < n {
        if gcd(&i, n) == one {
            result.push(i.clone());
        }
        i += &one;
    }

    result
}

fn main() {
    let n = BigUint::from_str("42").unwrap(); // You can change this to any number you want to test
    let rel_primes = relatively_prime_numbers(&n);

    println!("Numbers less than {} that are relatively prime to {}: {:?}", n, n, rel_primes);
}



