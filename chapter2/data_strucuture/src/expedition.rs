use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
        mut p: usize,
        mut ab: [(usize, usize); n],
    }
    ab.push((0, 0));
    ab.push((l, 0));
    ab.sort_unstable_by_key(|x| x.0);

    let mut heap = std::collections::BinaryHeap::new();
    let mut ans = 0;

    for v in ab.windows(2) {
        while heap.len() > 0 && p < v[1].0 - v[0].0 {
            p += heap.pop().unwrap();
            ans += 1;
        }
        if p < v[1].0 - v[0].0 {
            println!("-1");
            return;
        }
        p -= v[1].0 - v[0].0;
        heap.push(v[1].1);
    }

    println!("{}", ans);
}
