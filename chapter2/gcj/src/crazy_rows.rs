use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [[u8; n]; n],
    }

    let mut pos = a
        .iter()
        .map(|a| a.iter().rposition(|a| *a == 1).map(|x| x + 1).unwrap_or(0))
        .collect::<Vec<_>>();

    let mut ans = 0;
    for i in 0..n {
        let p = pos[i..].iter().position(|x| *x <= i + 1).unwrap();
        ans += p;
        let x = pos.remove(i + p);
        pos.insert(i, x);
    }

    println!("{}", ans);
}
