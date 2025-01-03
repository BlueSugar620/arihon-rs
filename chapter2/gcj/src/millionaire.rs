use proconio::input;

fn main() {
    input! {
        m: usize,
        p: f64,
        x: usize,
    }

    let n = 1 << m;

    let mut dp = vec![0.0; n + 1];
    dp[n] = 1.0;

    for _ in 0..m {
        let mut swp = vec![0.0; n + 1];
        for i in 0..=n {
            swp[i] = (0..=i.min(n - i))
                .map(|j| p * dp[i + j] + (1.0 - p) * dp[i - j])
                .fold(0.0, |acc: f64, a| acc.max(a));
        }
        dp = swp;
    }

    println!("{}", dp[x * n / 1_000_000]);
}
