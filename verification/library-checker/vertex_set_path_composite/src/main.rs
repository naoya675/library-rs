// verification-helper: PROBLEM https://judge.yosupo.jp/problem/vertex_set_path_composite

use proconio::input;

use euler_tour::EulerTour;
use modint::Modint;
use segment_tree::SegmentTree;

type Mint = Modint<998244353>;

query::define_query! {
    Query {
        0 => Query0(p: usize, c: i64, d: i64),
        1 => Query1(u: usize, v: usize, x: i64),
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
        ab: [(i64, i64); n],
        uv: [(usize, usize); n - 1],
        queries: [Query; q],
    }
    let mut et = EulerTour::<usize>::new(n);
    for &(u, v) in &uv {
        et.add_edge(u, v, 0);
        et.add_edge(v, u, 0);
    }
    et.init(0);

    let affine = |a: i64, b: i64| -> (Mint, Mint) { (Mint::new(a), Mint::new(b)) };
    let inv_affine = |a: i64, b: i64| -> (Mint, Mint) {
        let a = Mint::new(a);
        let b = Mint::new(b);
        (Mint::new(1) / a, -b / a)
    };
    let e = affine(1, 0);
    let mut st1 = SegmentTree::<(Mint, Mint)>::new(n + n, |x, y| (x.0 * y.0, x.1 * y.0 + y.1), e);
    let mut st2 = SegmentTree::<(Mint, Mint)>::new(n + n, |y, x| (x.0 * y.0, x.1 * y.0 + y.1), e);

    for i in 0..n {
        let (a, b) = ab[i];
        let (in_idx, out_idx) = et.index(i);
        st1.set(in_idx, affine(a, b));
        st2.set(in_idx, affine(a, b));
        st1.set(out_idx, inv_affine(a, b));
        st2.set(out_idx, inv_affine(a, b));
    }

    for query in queries {
        match query {
            Query0(p, c, d) => {
                let (in_idx, out_idx) = et.index(p);
                st1.set(in_idx, affine(c, d));
                st2.set(in_idx, affine(c, d));
                st1.set(out_idx, inv_affine(c, d));
                st2.set(out_idx, inv_affine(c, d));
            }
            Query1(u, v, x) => {
                let x = std::cell::Cell::new(Mint::new(x));
                et.for_each_with(
                    u,
                    v,
                    |l, r| {
                        let (a, b) = st1.prod(l, r);
                        x.set(x.get() * a + b);
                    },
                    |l, r| {
                        let (a, b) = st2.prod(l, r);
                        x.set(x.get() * a + b);
                    },
                );
                println!("{}", x.get());
            }
        }
    }
}
