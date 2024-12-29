use itertools::iproduct;
use proconio::{input, marker::Bytes};

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [Bytes; n],
    }
    solve1(n, m, &a);
}

fn solve1(n: usize, m: usize, a: &Vec<Vec<u8>>) {
    let s = iproduct!(0..n, 0..m)
        .find(|&(i, j)| a[i][j] == b'S')
        .unwrap();
    let g = iproduct!(0..n, 0..m)
        .find(|&(i, j)| a[i][j] == b'G')
        .unwrap();

    let mut d = vec![vec![!0; m]; n];
    let mut que = std::collections::VecDeque::new();

    d[s.0][s.1] = 0;
    que.push_back(s);

    while let Some((i, j)) = que.pop_front() {
        let x = d[i][j];
        for (di, dj) in [(1, 0), (0, 1), (!0, 0), (0, !0)] {
            let i = i.wrapping_add(di);
            let j = j.wrapping_add(dj);
            if i < n && j < m && d[i][j] == !0 && (a[i][j] == b'.' || a[i][j] == b'G') {
                d[i][j] = x + 1;
                que.push_back((i, j));
            }
        }
    }

    println!("{}", d[g.0][g.1]);
}
