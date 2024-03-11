// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_A

use proconio::input;
use std::cmp::min;

use data_structure::SegmentTree;

fn main() {
    input! {
        n: usize,
        q: usize,
        com: [(usize, usize, usize); q],
    }
    let mut st = SegmentTree::new(n, |a, b| min(a, b), (1 << 31) - 1);
    for (com, x, y) in com {
        match com {
            0 => st.set(x, y),
            1 => {
                println!("{}", st.prod(x, y + 1));
            }
            _ => unreachable!(),
        }
    }
}
