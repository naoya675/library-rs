// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP2_8_C

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

    let mut map = TreapMap::new();

    for query in queries {
        match query {
            Query0(key, x) => {
                map.insert(key, x);
            }
            Query1(key) => {
                output!(out, map.get(&key).copied().unwrap_or(0));
            }
            Query2(key) => {
                map.remove(&key);
            }
            Query3(l, r) => {
                let lo = map.lower_bound(&l);
                let hi = map.upper_bound(&r);
                for k in lo..hi {
                    let (key, value) = map.kth(k);
                    output!(out, key, value);
                }
            }
        }
    }
}
