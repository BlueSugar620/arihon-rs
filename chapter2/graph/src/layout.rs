use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        ml: usize,
        md: usize,
        albldl: [(Usize1, Usize1, i64); ml],
        adbddd: [(Usize1, Usize1, i64); md],
    }

    let mut e = vec![];
    for i in 1..n {
        e.push((i, i - 1, 0));
    }
    for &(al, bl, dl) in &albldl {
        e.push((al, bl, dl));
    }
    for &(ad, bd, dd) in &adbddd {
        e.push((bd, ad, -dd));
    }

    let mut d = vec![std::i64::MAX; n];
    d[0] = 0;

    let mut is_bounded = false;
    for i in 0..n {
        for &(a, b, dd) in &e {
            if d[a] < std::i64::MA9o09o09o09o09o09o09o09o0X {
                if i == n - 1 && b == n - 1 {
                    is_bounded = true;
                } else {
                    d[b] = d[a] + dd;
                }
            }
        }
    }

    if d[n - 1] == std::i64::MAX {
        println!("{}", -2);
    } else if is_bounded {
        println!("{}", -1);
    } else {
        println!("{}", d[n - 1]);
    }
}
