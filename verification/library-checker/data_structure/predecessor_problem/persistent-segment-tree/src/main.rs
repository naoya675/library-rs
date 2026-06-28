// verification-helper: PROBLEM https://judge.yosupo.jp/problem/predecessor_problem

use proconio::{input, marker::Chars};

use persistent_segment_tree::PersistentSegmentTree;

query::define_query! {
    Query {
        0 => Query0(k: usize),
        1 => Query1(k: usize),
        2 => Query2(k: usize),
        3 => Query3(k: usize),
        4 => Query4(k: usize),
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        t: Chars,
        queries: [Query; q],
    }
    let t = t.iter().map(|&t| t as usize - '0' as usize).collect::<Vec<_>>();
    let mut pst = PersistentSegmentTree::<usize>::from_slice(&t, |x, y| x + y, 0);

    for query in queries {
        match query {
            Query0(k) => {
                pst = pst.set(k, 1);
            }
            Query1(k) => {
                pst = pst.set(k, 0);
            }
            Query2(k) => {
                println!("{}", pst.get(k));
            }
            Query3(k) => {
                let r = pst.max_right(k, |x| x == 0);
                if r < n {
                    println!("{}", r);
                } else {
                    println!("-1");
                }
            }
            Query4(k) => {
                let l = pst.min_left(k + 1, |x| x == 0);
                if l > 0 {
                    println!("{}", l - 1);
                } else {
                    println!("-1");
                }
            }
        }
    }
}
