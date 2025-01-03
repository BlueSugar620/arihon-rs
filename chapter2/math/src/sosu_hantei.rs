use proconio::input;

fn main() {
    input! {
        n: u64,
    }

    println!("{}", if is_prime(n) { "Yes" } else { "No" });
}

fn is_prime(n: u64) -> bool {
    (2..).take_while(|i| i * i <= n).all(|i| n % i != 0)
}
