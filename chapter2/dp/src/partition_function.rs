use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
    }

    // dp[i][j]: i の j 分割数
    let mut dp = vec![vec![0; m + 1]; n + 1];
    dp[0][0] = 1;
    for j in 1..=m {
        for i in 0..=n {
            dp[i][j] = (if i >= j { dp[i - j][j] } else { 0 } + dp[i][j - 1]) % q;
        }
    }
    println!("{}", dp[n][m]);
}
