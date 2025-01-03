use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u64,
    }

    let (x, y, g) = extgcd(a, b);
    if g > 1 {
        println!("{}", -1);
    } else {
        println!("{} {} {} {}", x.max(0), y.max(0), -x.min(0), -y.min(0));
    }
}

fn extgcd(a: u64, b: u64) -> (i64, i64, u64) {
    if b == 0 {
        return (1, 0, a);
    }
    let (x, y, g) = extgcd(b, a % b);
    (y, x - (a / b) as i64 * y, g)
}
