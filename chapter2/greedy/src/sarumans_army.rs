use proconio::input;

fn main() {
    input! {
        n: usize,
        r: u64,
        mut x: [u64; n],
    }

    x.sort_unstable();

    let mut pos = x[0];
    let mut ans = 0;
    for x in x.iter() {
        if x - pos >= r {
            pos = *x;
            ans += 1;
        }
    }

    println!("{}", ans);
}
