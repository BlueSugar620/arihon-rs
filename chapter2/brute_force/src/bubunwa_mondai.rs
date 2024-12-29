use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
        k: i64,
    }

    solve1(n, &a, k);
    solve2(n, &a, k);
}

fn solve1(n: usize, a: &[i64], k: i64) {
    let ans = (0..1 << n)
        .map(|bits| {
            (0..n)
                .filter(|i| (bits >> i) & 1 == 1)
                .map(|i| a[i])
                .sum::<i64>()
        })
        .contains(&k);

    println!("{}", if ans { "Yes" } else { "No" });
}

fn solve2(_n: usize, a: &[i64], k: i64) {
    println!("{}", if rec(0, 0, a, k) { "Yes" } else { "No" });
}

fn rec(i: usize, t: i64, a: &[i64], k: i64) -> bool {
    if i == a.len() {
        return t == k;
    }
    rec(i + 1, t + a[i], a, k) || rec(i + 1, t, a, k)
}
