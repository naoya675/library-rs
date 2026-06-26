// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP2_8_D

use proconio::input;

use treap_map::TreapMap;

fn main() {
    input! {
        q: usize,
    }
    let mut map = TreapMap::<String, Vec<i64>>::new();
    for _ in 0..q {
        input! { t: usize }
        match t {
            0 => {
                input! { key: String, x: i64 }
                map.entry(key).or_insert_with(Vec::new).push(x);
            }
            1 => {
                input! { key: String }
                if let Some(values) = map.get(&key) {
                    for &value in values {
                        println!("{}", value);
                    }
                }
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
                    let (key, values) = map.kth(k);
                    for &value in values {
                        println!("{} {}", key, value);
                    }
                }
            }
            _ => unreachable!(),
        }
    }
}
