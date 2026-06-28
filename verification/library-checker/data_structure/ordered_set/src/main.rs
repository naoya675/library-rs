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
                if x <= trie.len() {
                    println!("{}", trie.kth(x - 1));
                } else {
                    println!("-1");
                }
            }
            Query3(x) => {
                println!("{}", trie.upper_bound(x));
            }
            Query4(x) => {
                let k = trie.upper_bound(x);
                if k > 0 {
                    println!("{}", trie.kth(k - 1));
                } else {
                    println!("-1");
                }
            }
            Query5(x) => {
                let k = trie.lower_bound(x);
                if k < trie.len() {
                    println!("{}", trie.kth(k));
                } else {
                    println!("-1");
                }
            }
        }
    }
}
