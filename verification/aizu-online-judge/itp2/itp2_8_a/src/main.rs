// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP2_8_A

use fast_io::{Output, define_query, input, output};

use treap_map::TreapMap;

define_query! {
    Query {
        0 => Query0(key: String, x: i64),
        1 => Query1(key: String),
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
                output!(out, map.get(&key).unwrap());
            }
        }
    }
}
