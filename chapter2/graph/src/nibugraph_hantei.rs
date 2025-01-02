use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    }

    let mut e = vec![vec![]; n];
    for &(a, b) in &ab {
        e[a].push(b);
        e[b].push(a);
    }

    let mut res = vec![0; n];

    for i in 0..n {
        if res[i] == 0 {
            let mut stack = vec![i];
            res[i] = 1;
            while let Some(u) = stack.pop() {
                for &v in &e[u] {
                    if res[v] == 0 {
                        stack.push(v);
                        res[v] = 3 - res[u];
                    } else if res[v] == res[u] {
                        println!("No");
                        return;
                    }
                }
            }
        }
    }

    println!("Yes");
}
