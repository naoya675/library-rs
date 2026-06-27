// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP2_7_D

use proconio::input;

use treap_map::TreapMap;

query::define_query! {
    Query {
        0 => Query0(x: usize),
        1 => Query1(x: usize),
        2 => Query2(x: usize),
        3 => Query3(l: usize, r: usize), // dump [l, r]
    }
}

fn main() {
    input! {
        q: usize,
        queries: [Query; q],
    }
    let mut map = TreapMap::<usize, usize>::new();
    let mut cnt = 0;

    for query in queries {
        match query {
            Query0(x) => {
                *map.entry(x).or_insert(0) += 1;
                cnt += 1;
                println!("{}", cnt);
            }
            Query1(x) => {
                println!("{}", map.get(&x).copied().unwrap_or(0));
            }
            Query2(x) => {
                if let Some(c) = map.remove(&x) {
                    cnt -= c;
                }
            }
            Query3(l, r) => {
                let lo = map.lower_bound(&l);
                let hi = map.upper_bound(&r);
                for k in lo..hi {
                    let (&key, &cnt) = map.kth(k);
                    for _ in 0..cnt {
                        println!("{}", key);
                    }
                }
            }
        }
    }
}
