use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut l = 0;
    let mut r = n;
    let mut ans = vec![];

    while l < r {
        if &s[l..r].iter().collect::<String>() < &s[l..r].iter().rev().collect::<String>() {
            ans.push(s[l]);
            l += 1;
        } else {
            ans.push(s[r - 1]);
            r -= 1;
        }
    }

    println!("{}", ans.iter().join(""));
}
