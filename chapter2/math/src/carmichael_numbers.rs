use proconio::input;

fn main() {
    input! {
        n: u64,
    }

    println!(
        "{}",
        if !is_prime(n) && (2..n).all(|i| pow_mod(i, n, n) == i) {
            "Yes"
        } else {
            "No"
        }
    );
}

fn is_prime(n: u64) -> bool {
    (2..).take_while(|i| i * i <= n).all(|i| n % i != 0)
}

fn pow_mod(a: u64, mut n: u64, m: u64) -> u64 {
    let mut res = 1;
    let mut base = a;
    while n > 0 {
        if n & 1 == 1 {
            res *= base;
            res %= m;
        }
        base *= base;
        base %= m;
        n >>= 1;
    }
    res
}
