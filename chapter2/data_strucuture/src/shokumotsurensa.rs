use disjoint_set_union::DisjointSetUnion;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let mut dsu = DisjointSetUnion::new(3 * n);
    let mut ans = 0;

    for _ in 0..k {
        input! {
            t: u8,
        }
        if t == 1 {
            input! {
                x: Usize1,
                y: Usize1,
            }

            if x >= n || y >= n {
                ans += 1;
                continue;
            }

            if dsu.is_same(x, n + x) || dsu.is_same(x, 2 * n + y) {
                ans += 1;
            } else {
                for i in 0..3 {
                    dsu.unite(n * i + x, n * i + y);
                }
            }
        } else if t == 2 {
            input! {
                x: Usize1,
                y: Usize1,
            }

            if x >= n || y >= n {
                ans += 1;
                continue;
            }

            if dsu.is_same(x, y) || dsu.is_same(n + x, y) {
                ans += 1;
            } else {
                for i in 0..3 {
                    dsu.unite(n * i + x, n * ((i + 1) % 3) + y);
                }
            }
        } else {
            ans += 1;
        }
    }

    println!("{}", ans);
}

pub mod disjoint_set_union {
    pub struct DisjointSetUnion {
        parents: Vec<isize>,
        cnt: usize,
    }

    impl DisjointSetUnion {
        pub fn new(n: usize) -> Self {
            Self {
                parents: vec![-1; n],
                cnt: n,
            }
        }

        pub fn root(&self, mut v: usize) -> usize {
            while self.parents[v] >= 0 {
                v = self.parents[v] as usize;
            }
            v
        }

        pub fn unite(&mut self, u: usize, v: usize) {
            let mut u = self.root(u);
            let mut v = self.root(v);
            if u == v {
                return;
            }
            if self.parents[u] > self.parents[v] {
                std::mem::swap(&mut u, &mut v);
            }
            self.parents[u] += self.parents[v];
            self.parents[v] = u as isize;
            self.cnt -= 1;
        }

        pub fn is_same(&self, u: usize, v: usize) -> bool {
            self.root(u) == self.root(v)
        }

        pub fn size(&self, v: usize) -> usize {
            -self.parents[self.root(v)] as usize
        }

        pub fn cnt(&self) -> usize {
            self.cnt
        }
    }
}
