use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }

    let mut dp = vec![!0; n + 1];
    dp[0] = 0;

    for &a in &a {
        for i in 1..=n {
            if dp[i - 1] < a && a < dp[i] {
                dp[i] = a;
            }
        }
    }

    println!("{}", dp.iter().rposition(|dp| *dp != !0).unwrap());
}
