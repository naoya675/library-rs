// verification-helper: PROBLEM https://judge.yosupo.jp/problem/predecessor_problem

use proconio::{input, marker::Chars};

use persistent_segment_tree_pool::PersistentSegmentTreePool;

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
    let mut pst = PersistentSegmentTreePool::<usize>::new(n, |x, y| x + y, 0);
    let mut versions = pst.from_slice(&t);

    for query in queries {
        match query {
            Query0(k) => {
                versions = pst.set(versions, k, 1);
            }
            Query1(k) => {
                versions = pst.set(versions, k, 0);
            }
            Query2(k) => {
                println!("{}", pst.get(versions, k));
            }
            Query3(k) => {
                let r = pst.max_right(versions, k, |x| x == 0);
                if r < n {
                    println!("{}", r);
                } else {
                    println!("-1");
                }
            }
            Query4(k) => {
                let l = pst.min_left(versions, k + 1, |x| x == 0);
                if l > 0 {
                    println!("{}", l - 1);
                } else {
                    println!("-1");
                }
            }
        }
    }
}
