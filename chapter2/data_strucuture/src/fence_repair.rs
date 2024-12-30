use proconio::input;

fn main() {
    input! {
        n: usize,
        l: [u64; n],
    }
    let mut l = l
        .iter()
        .copied()
        .map(|x| !x)
        .collect::<std::collections::BinaryHeap<u64>>();

    let mut ans = 0;
    while l.len() > 1 {
        let x = !l.pop().unwrap();
        let y = !l.pop().unwrap();
        ans += x + y;
        l.push(!(x + y));
    }

    println!("{}", ans);
}
