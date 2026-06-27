// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP2_8_A

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
                println!("{}", map.get(&key).unwrap());
            }
            _ => unreachable!(),
        }
    }
}
