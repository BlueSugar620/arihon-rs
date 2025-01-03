use proconio::input;
use sieve_of_eratosthenes::SieveOfEratosthenes;

fn main() {
    input! {
        n: usize,
    }

    let sieve = SieveOfEratosthenes::new(n);

    println!("{}", sieve.prime_counting(n));
}

pub mod sieve_of_eratosthenes {
    pub struct SieveOfEratosthenes {
        is_prime: Box<[bool]>,
    }

    impl SieveOfEratosthenes {
        pub fn new(n: usize) -> Self {
            let mut is_prime = vec![true; n + 1];
            is_prime[0] = false;
            is_prime[1] = false;
            for i in 2..=n {
                if is_prime[i] {
                    for j in (2..).take_while(|j| i * j <= n) {
                        is_prime[i * j] = false;
                    }
                }
            }
            Self {
                is_prime: is_prime.into_boxed_slice(),
            }
        }

        pub fn is_prime(&self, n: usize) -> bool {
            self.is_prime[n]
        }

        pub fn prime_counting(&self, n: usize) -> usize {
            (0..=n).filter(|i| self.is_prime[*i]).count()
        }
    }
}
