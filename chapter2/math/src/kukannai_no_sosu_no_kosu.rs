use proconio::input;
use segment_sieve_of_eratosthenes::SegmentSieveOfEratosthenes;

fn main() {
    input! {
        a: usize,
        b: usize,
    }

    let sieve = SegmentSieveOfEratosthenes::new(a - 1, b - 1);
    println!("{}", sieve.prime_counting());
}

pub mod segment_sieve_of_eratosthenes {
    pub struct SegmentSieveOfEratosthenes {
        a: usize,
        b: usize,
        is_prime: Box<[bool]>,
    }

    impl SegmentSieveOfEratosthenes {
        pub fn new(a: usize, b: usize) -> Self {
            let sqrt_b = (b as f64).sqrt() as usize + 1;
            let mut sieves = vec![true; sqrt_b + 1];
            sieves[0] = false;
            sieves[1] = false;
            for i in (2..).take_while(|i| i * i <= sqrt_b) {
                if sieves[i] {
                    for j in (2..).take_while(|j| i * j <= sqrt_b) {
                        sieves[i * j] = false;
                    }
                }
            }

            let mut is_prime = vec![true; b - a + 1];
            for (i, &seive) in sieves.iter().enumerate() {
                if seive {
                    let s = (a + i - 1) / i;
                    for j in (s..).take_while(|j| i * j <= b) {
                        if s != 1 {
                            is_prime[i * j - a] = false;
                        }
                    }
                }
            }

            Self {
                a,
                b,
                is_prime: is_prime.into_boxed_slice(),
            }
        }

        pub fn is_prime(&self, n: usize) -> bool {
            self.is_prime[n - self.a]
        }

        pub fn prime_counting(&self) -> usize {
            (self.a + 1..=self.b)
                .filter(|i| self.is_prime[i - self.a])
                .count()
        }
    }
}
