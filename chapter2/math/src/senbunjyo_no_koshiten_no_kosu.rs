use proconio::input;

fn main() {
    input! {
        x: [i64; 2],
        y: [i64; 2],
    }

    let (x, y) = ((x[0] - y[0]).abs() as u64, (x[1] - y[1]).abs() as u64);
    println!("{}", gcd(x, y).max(1) - 1);
}

pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}
