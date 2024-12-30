use proconio::input;

fn main() {
    input! {
        n: usize,
        wv: [(usize, u64); n],
        w: usize,
    }

    let mut dp = vec![0; w + 1];
    for &(wi, vi) in &wv {
        let mut swp = dp.clone();
        for j in 0..=w.saturating_sub(wi) {
            swp[j + wi] = swp[j + wi].max(dp[j] + vi);
        }
        dp = swp;
    }

    println!("{}", dp.iter().max().unwrap());
}
