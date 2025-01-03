use proconio::input;

fn main() {
    input! {
        n: usize,
        mut x: [i64; n],
        mut y: [i64; n],
    }

    x.sort_unstable();
    y.sort_unstable();

    println!(
        "{}",
        x.iter()
            .zip(y.iter().rev())
            .map(|(x, y)| x * y)
            .sum::<i64>()
    );
}
