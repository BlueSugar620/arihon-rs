use proconio::input;

fn main() {
    input! {
        n: usize,
        mut st: [(u64, u64); n],
    }

    st.sort_unstable_by_key(|x| x.1);

    let mut ans = 0;
    let mut time = 0;
    for &(s, t) in &st {
        if time < s {
            ans += 1;
            time = t;
        }
    }

    println!("{}", ans);
}
