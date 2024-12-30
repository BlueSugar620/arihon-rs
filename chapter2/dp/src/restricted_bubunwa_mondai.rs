use proconio::input;

fn main() {
    input! {
        n: usize,
        am: [(usize, usize); n],
        k: usize,
    }

    let mut dp = vec![false; k + 1];
    dp[0] = true;

    for &(a, m) in &am {
        let mut swp = dp
            .iter()
            .copied()
            .map(|dp| if dp { 0 } else { !0 >> 1 })
            .collect::<Vec<_>>();
        for i in 0..=k.saturating_sub(a) {
            swp[i + a] = swp[i + a].min(if dp[i] { 1 } else { swp[i] + 1 });
        }
        for (dp, swp) in dp.iter_mut().zip(swp.iter()) {
            *dp = if *swp <= m { true } else { false };
        }
    }

    println!("{}", if dp[k] { "Yes" } else { "No" });
}
