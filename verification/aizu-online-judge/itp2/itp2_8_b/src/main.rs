// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP2_8_B

use proconio::input;

use treap_map::TreapMap;

query::define_query! {
    Query {
        0 => Query0(key: String, x: i64),
        1 => Query1(key: String),
        2 => Query2(key: String),
    }
}

fn main() {
    input! {
        q: usize,
        queries: [Query; q],
    }
    let mut map = TreapMap::<String, i64>::new();
    for query in queries {
        match query {
            Query0(key, x) => {
                map.insert(key, x);
            }
            Query1(key) => {
                println!("{}", map.get(&key).copied().unwrap_or(0));
            }
            Query2(key) => {
                map.remove(&key);
            }
        }
    }
}
