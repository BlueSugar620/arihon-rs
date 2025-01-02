use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1, u64); m],
    }

    let mut e = vec![vec![]; n];
    for &(a, b, d) in &ab {
        e[a].push((b, d));
        e[b].push((a, d));
    }

    let mut first = vec![!0; n];
    let mut second = vec![!0; n];
    let mut heap = std::collections::BinaryHeap::new();

    first[0] = 0;
    heap.push((!0, 0));

    while let Some((d, u)) = heap.pop() {
        let d = !d;
        if second[u] < d {
            continue;
        } else if first[u] < d {
            second[u] = d;
            for &(v, dd) in &e[u] {
                heap.push((!(d + dd), v));
            }
        } else {
            first[u] = d;
            for &(v, dd) in &e[u] {
                heap.push((!(d + dd), v));
            }
        }
    }

    println!("{}", second[n - 1]);
}
