// verification-helper: PROBLEM https://judge.yosupo.jp/problem/set_xor_min

use proconio::input;

use binary_trie::BinaryTrie;

query::define_query! {
    Query {
        0 => Query0(x: usize),
        1 => Query1(x: usize),
        2 => Query2(x: usize),
    }
}

fn main() {
    input! {
        q: usize,
        queries: [Query; q],
    }
    let mut trie = BinaryTrie::new(60);

    for query in queries {
        match query {
            Query0(x) => {
                if !trie.contains(x) {
                    trie.insert(x);
                }
            }
            Query1(x) => {
                trie.remove(x);
            }
            Query2(x) => {
                println!("{}", trie.xor_min(x));
            }
        }
    }
}
