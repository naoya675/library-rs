// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP2_8_D

use fast_io::{Output, define_query, input, output};

use treap_map::TreapMap;

define_query! {
    Query {
        0 => Query0(key: String, x: i64),
        1 => Query1(key: String),
        2 => Query2(key: String),
        3 => Query3(l: String, r: String), // dump [l, r]
    }
}

fn main() {
    input! {
        q: usize,
        queries: [Query; q],
    }
    let mut out = Output::new();

    let mut map = TreapMap::<String, Vec<i64>>::new();

    for query in queries {
        match query {
            Query0(key, x) => {
                map.entry(key).or_default().push(x);
            }
            Query1(key) => {
                if let Some(values) = map.get(&key) {
                    for &value in values {
                        output!(out, value);
                    }
                }
            }
            Query2(key) => {
                map.remove(&key);
            }
            Query3(l, r) => {
                let lo = map.lower_bound(&l);
                let hi = map.upper_bound(&r);
                for k in lo..hi {
                    let (key, values) = map.kth(k);
                    for &value in values {
                        output!(out, key, value);
                    }
                }
            }
        }
    }
}
