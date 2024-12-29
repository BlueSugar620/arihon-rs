use proconio::{input, marker::Bytes};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Bytes; n],
    }

    solve1(n, m, &s);
}

fn solve1(n: usize, m: usize, s: &Vec<Vec<u8>>) {
    let mut s = s.clone();
    let mut ans = 0;
    for i in 0..n {
        for j in 0..m {
            if s[i][j] == b'W' {
                ans += 1;
                s[i][j] = b'.';
                f(i, j, n, m, &mut s);
            }
        }
    }
    println!("{}", ans);
}

fn f(i: usize, j: usize, n: usize, m: usize, s: &mut Vec<Vec<u8>>) {
    for (di, dj) in [
        (1, 0),
        (0, 1),
        (!0, 0),
        (0, !0),
        (1, 1),
        (1, !0),
        (!0, 1),
        (!0, !0),
    ] {
        let i = i.wrapping_add(di);
        let j = j.wrapping_add(dj);
        if i < n && j < m && s[i][j] == b'W' {
            s[i][j] = b'.';
            f(i, j, n, m, s);
        }
    }
}
