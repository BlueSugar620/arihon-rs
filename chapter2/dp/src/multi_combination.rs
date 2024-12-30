use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        q: usize,
    }

    let mut dp = vec![0; m + 1];
    dp[0] = 1;

    for &a in a.iter() {
        let mut swp = vec![0; m + 1];
        for j in 0..=m {
            swp[j] = (if j > 0 { swp[j - 1] } else { 0 } + dp[j]
                - if j > a { dp[j - 1 - a] } else { 0 })
                % q;
        }
        dp = swp;
    }

    println!("{}", dp[m]);
}
