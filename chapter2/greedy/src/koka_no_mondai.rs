use proconio::input;

const COINS: [u64; 6] = [1, 5, 10, 50, 100, 500];

fn main() {
    input! {
        c: [u64; 6],
        mut a: u64,
    }

    let ans = c
        .iter()
        .zip(COINS.iter())
        .rev()
        .map(|(&c, &x)| {
            let res = a / x;
            a -= c.min(res) * x;
            c.min(res)
        })
        .sum::<u64>();

    println!("{}", ans);
}
