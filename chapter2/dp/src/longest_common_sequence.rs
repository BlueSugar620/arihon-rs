use proconio::{input, marker::Bytes};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: Bytes,
        t: Bytes,
    }

    let mut dp = vec![vec![0; m + 1]; n + 1];
    for (i, &s) in s.iter().enumerate() {
        for (j, &t) in t.iter().enumerate() {
            if s == t {
                dp[i + 1][j + 1] = dp[i][j] + 1;
            } else {
                dp[i + 1][j + 1] = dp[i + 1][j].max(dp[i][j + 1]);
            }
        }
    }

    println!("{}", dp[n][m]);
}
