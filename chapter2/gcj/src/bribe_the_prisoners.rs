use memoise::memoise;
use proconio::input;

fn main() {
    input! {
        p: usize,
        q: usize,
        mut a: [usize; q],
    }

    a.insert(0, 0);
    a.push(p + 1);

    println!("{}", dfs(1, a.len() - 1, &a));
}

// a[l - 1], a[r] に囚人がいない状況において、 [l, r) の囚人を解放するのに必要な金貨の最小値
#[memoise(l, r)]
fn dfs(l: usize, r: usize, a: &[usize]) -> usize {
    if l == r {
        return 0;
    }
    let res = (l..r)
        .map(|i| dfs(l, i, a) + dfs(i + 1, r, a))
        .min()
        .unwrap()
        + a[r]
        - a[l - 1]
        - 2;
    res
}
