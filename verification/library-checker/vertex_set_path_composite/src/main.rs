// verification-helper: PROBLEM https://judge.yosupo.jp/problem/vertex_set_path_composite

use proconio::input;

use euler_tour::EulerTour;
use modint::StaticModint;
use segment_tree::SegmentTree;

type Mint = StaticModint<998244353>;

query::define_query! {
    Query {
        0 => Query0(p: usize, c: u64, d: u64),
        1 => Query1(u: usize, v: usize, x: u64),
    }
}

fn main() {
    std::thread::Builder::new()
        .stack_size(64 * 1024 * 1024)
        .spawn(actual_main)
        .unwrap()
        .join()
        .unwrap();
}

fn actual_main() {
    input! {
        n: usize,
        q: usize,
        ab: [(u64, u64); n],
        uv: [(usize, usize); n - 1],
        queries: [Query; q],
    }
    let mut et = EulerTour::<usize>::new(n);
    uv.iter().for_each(|&(u, v)| {
        et.add_edge(u, v, 0);
        et.add_edge(v, u, 0);
    });
    et.init(0);

    let val = |a: u64, b: u64| (Mint::new(a), Mint::new(b));
    let invval = |a: u64, b: u64| (Mint::new(1) / Mint::new(a), -Mint::new(b) / Mint::new(a));
    let mut st1 = SegmentTree::<(Mint, Mint)>::new(n + n, |a, b| (a.0 * b.0, a.1 * b.0 + b.1), val(1, 0));
    let mut st2 = SegmentTree::<(Mint, Mint)>::new(n + n, |b, a| (a.0 * b.0, a.1 * b.0 + b.1), val(1, 0));
    for i in 0..n {
        let (a, b) = ab[i];
        let index = et.index(i);
        st1.set(index.0, val(a, b));
        st2.set(index.0, val(a, b));
        st1.set(index.1, invval(a, b));
        st2.set(index.1, invval(a, b));
    }
    for query in queries {
        match query {
            Query0(p, c, d) => {
                let index = et.index(p);
                st1.set(index.0, val(c, d));
                st2.set(index.0, val(c, d));
                st1.set(index.1, invval(c, d));
                st2.set(index.1, invval(c, d));
            }
            Query1(u, v, x) => {
                let x = std::cell::RefCell::new(Mint::new(x));
                et.for_each_with(
                    u,
                    v,
                    |l, r| {
                        let (a, b) = st1.prod(l, r);
                        let tmp = *x.borrow() * a + b;
                        *x.borrow_mut() = tmp;
                    },
                    |l, r| {
                        let (a, b) = st2.prod(l, r);
                        let tmp = *x.borrow() * a + b;
                        *x.borrow_mut() = tmp;
                    },
                );
                println!("{}", x.into_inner());
            }
        }
    }
}
