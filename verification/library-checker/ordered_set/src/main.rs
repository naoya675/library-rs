// verification-helper: PROBLEM https://judge.yosupo.jp/problem/ordered_set

use proconio::input;

use binary_trie::BinaryTrie;

query::define_query! {
    Query {
        0 => Query0(x: usize),
        1 => Query1(x: usize),
        2 => Query2(x: usize),
        3 => Query3(x: usize),
        4 => Query4(x: usize),
        5 => Query5(x: usize),
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
        queries: [Query; q],
    }
    let mut trie = BinaryTrie::new(30);
    for &x in &a {
        trie.insert(x);
    }
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
                println!("{}", if x <= trie.len() { trie.kth(x - 1).to_string() } else { "-1".to_string() });
            }
            Query3(x) => {
                println!("{}", trie.upper_bound(x));
            }
            Query4(x) => {
                let k = trie.upper_bound(x);
                println!("{}", if k > 0 { trie.kth(k - 1).to_string() } else { "-1".to_string() })
            }
            Query5(x) => {
                let k = trie.lower_bound(x);
                println!("{}", if k < trie.len() { trie.kth(k).to_string() } else { "-1".to_string() });
            }
        }
    }
}
