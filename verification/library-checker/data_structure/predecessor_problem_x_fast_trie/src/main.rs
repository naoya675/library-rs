// verification-helper: PROBLEM https://judge.yosupo.jp/problem/predecessor_problem

use fast_io::{Output, define_query, input, output};

use x_fast_trie::XFastTrie;

define_query! {
    Query {
        0 => Query0(k: usize),
        1 => Query1(k: usize),
        2 => Query2(k: usize),
        3 => Query3(k: usize),
        4 => Query4(k: usize),
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        t: Chars,
        queries: [Query; q],
    }
    let mut out = Output::new();

    let bits = n.next_power_of_two().ilog2().max(1);
    let mut trie = XFastTrie::new(bits);
    for (i, &c) in t.iter().enumerate() {
        if c == '1' {
            trie.insert(i);
        }
    }

    for query in queries {
        match query {
            Query0(k) => {
                trie.insert(k);
            }
            Query1(k) => {
                trie.remove(k);
            }
            Query2(k) => {
                output!(out, if trie.contains(k) { 1 } else { 0 });
            }
            Query3(k) => {
                if let Some(x) = trie.successor(k) {
                    output!(out, x);
                } else {
                    output!(out, -1);
                }
            }
            Query4(k) => {
                if let Some(x) = trie.predecessor(k) {
                    output!(out, x);
                } else {
                    output!(out, -1);
                }
            }
        }
    }
}
