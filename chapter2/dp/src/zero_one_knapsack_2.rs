use proconio::input;

fn main() {
    input! {
        n: usize,
        wv: [(u64, usize); n],
        w: u64,
    }

    let mut dp = vec![!0u64; wv.iter().map(|x| x.1).sum::<usize>() + 1];
    dp[0] = 0;
    for &(w, v) in &wv {
        for i in (0..dp.len().saturating_sub(v)).rev() {
            dp[i + v] = dp[i + v].min(dp[i].saturating_add(w));
        }
    }

    println!("{}", dp.iter().rposition(|dp| *dp <= w).unwrap());
}
