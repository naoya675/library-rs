// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP2_8_C

use proconio::input;

use treap_map::TreapMap;

fn main() {
    input! {
        q: usize,
    }
    let mut map = TreapMap::<String, i64>::new();
    for _ in 0..q {
        input! { t: usize }
        match t {
            0 => {
                input! { key: String, x: i64 }
                map.insert(key, x);
            }
            1 => {
                input! { key: String }
                println!("{}", map.get(&key).copied().unwrap_or(0));
            }
            2 => {
                input! { key: String }
                map.remove(&key);
            }
            3 => {
                input! { l: String, r: String } // dump [l, r]
                let lo = map.lower_bound(&l);
                let hi = map.upper_bound(&r);
                for k in lo..hi {
                    let (key, value) = map.kth(k);
                    println!("{} {}", key, value);
                }
            }
            _ => unreachable!(),
        }
    }
}
