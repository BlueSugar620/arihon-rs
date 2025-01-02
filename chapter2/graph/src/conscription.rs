use disjoint_set_union::DisjointSetUnion;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        r: usize,
        mut abd: [(Usize1, Usize1, u64); r],
    }

    abd.sort_unstable_by_key(|x| !x.2);

    let mut dsu = DisjointSetUnion::new(n + m);
    let mut tot = 0;
    for &(a, b, d) in abd.iter() {
        if !dsu.is_same(a, b + m) {
            dsu.unite(a, b + m);
            tot += d;
        }
    }

    println!("{}", 10000 * (n + m) as u64 - tot);
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
