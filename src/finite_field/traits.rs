pub trait isPrime {
    fn isPrime(&self) -> bool;
}


pub fn is_prime(n: u64, k: u64) -> bool {
    // Handle base cases for n < 3
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }

    // If n is even, return false
    if n % 2 == 0 {
        return false;
    }

    // Find an odd number d such that n-1 can be written as d*2^r
    let mut d = n - 1;
    let mut r = 0;
    while d % 2 == 0 {
        d /= 2;
        r += 1;
    }

    for _ in 0..k {
        let a = 2; // Fixed base 2 for simplicity
        let mut x = mod_pow(a, d, n);

        if x == 1 || x == n - 1 {
            continue;
        }

        for _ in 0..r - 1 {
            x = mod_pow(x, 2, n);

            if x == 1 {
                return false;
            }

            if x == n - 1 {
                break;
            }
        }

        if x != n - 1 {
            return false;
        }
    }

    true
}

fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    let mut result = 1;
    base %= modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        exp >>= 1;
        base = (base * base) % modulus;
    }
    result
}