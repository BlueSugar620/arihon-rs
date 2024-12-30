use proconio::input;

fn main() {
    input! {
        n: usize,
        wv: [(usize, u64); n],
        w: usize,
    }

    let mut dp = vec![0; w + 1];
    for &(wi, vi) in &wv {
        for i in 0..=w.saturating_sub(wi) {
            dp[i + wi] = dp[i + wi].max(dp[i] + vi);
        }
    }

    println!("{}", dp.iter().max().unwrap());
}
