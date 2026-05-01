// verification-helper: PROBLEM https://judge.yosupo.jp/problem/range_set_range_composite

use std::cell::RefCell;

use proconio::input;

use interval_set::IntervalSet;
use modint::Modint;
use segment_tree::SegmentTree;

type Mint = Modint<998244353>;

query::define_query! {
    Query {
        0 => Query0(l: usize, r: usize, c: i64, d: i64),
        1 => Query1(l: usize, r: usize, x: i64),
    }
}

fn op_affine(x: (Mint, Mint), y: (Mint, Mint)) -> (Mint, Mint) {
    let (a1, b1) = x;
    let (a2, b2) = y;
    (a1 * a2, b1 * a2 + b2)
}

fn pow_affine(f: (Mint, Mint), mut n: u64) -> (Mint, Mint) {
    let mut value = f;
    let mut res = (Mint::new(1), Mint::new(0));
    while n > 0 {
        if n & 1 == 1 {
            res = op_affine(res, value);
        }
        value = op_affine(value, value);
        n >>= 1;
    }
    res
}

struct State {
    st: RefCell<SegmentTree<(Mint, Mint)>>,
    set: IntervalSet<usize, ((Mint, Mint), u64)>,
    e: (Mint, Mint),
    time: u64,
}

impl State {
    fn new(n: usize) -> Self {
        let e = (Mint::new(1), Mint::new(0));
        Self {
            st: RefCell::new(SegmentTree::new(n, op_affine, e)),
            set: IntervalSet::new((e, 0)),
            e,
            time: 0,
        }
    }

    fn apply(&mut self, l: usize, r: usize, val: (Mint, Mint)) {
        self.time += 1;
        self.set.update_inner(
            l,
            r,
            (val, self.time),
            |a, b, v| {
                self.st.borrow_mut().set(a, pow_affine(v.0, (b - a) as u64));
            },
            |a, _, _| {
                self.st.borrow_mut().set(a, self.e);
            },
        );
    }

    fn apply_split(&mut self, p: usize) {
        if let Some((a, _, val)) = self.set.get(p) {
            if a < p {
                self.apply(a, p, val.0);
            }
        }
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        ab: [(i64, i64); n],
        queries: [Query; q],
    }
    let mut state = State::new(n);
    for (i, &(a, b)) in ab.iter().enumerate() {
        state.apply(i, i + 1, (Mint::new(a), Mint::new(b)));
    }

    for query in queries {
        match query {
            Query0(l, r, c, d) => {
                state.apply(l, r, (Mint::new(c), Mint::new(d)));
            }
            Query1(l, r, x) => {
                state.apply_split(l);
                state.apply_split(r);
                let (a, b) = state.st.borrow().prod(l, r);
                println!("{}", Mint::new(x) * a + b);
            }
        }
    }
}
