// verification-helper: PROBLEM https://judge.yosupo.jp/problem/longest_common_substring

use proconio::{input, marker::Chars};

use lcp_array::lcp_array;
use suffix_array::suffix_array;

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }
    let n = s.len();
    let mut st = s;
    st.push('#');
    st.extend(t.iter());

    let sa = suffix_array(&st);
    let lcp = lcp_array(&st, &sa);
    let mut max = 0;
    let (mut a, mut b) = (0, 0);
    let (mut c, mut d) = (0, 0);
    for i in 0..sa.len() - 1 {
        let (p, q) = (sa[i], sa[i + 1]);
        if p == n || q == n {
            continue;
        }
        if (p < n) != (q < n) && lcp[i] > max {
            max = lcp[i];
            let (ps, pt) = if p < n { (p, q) } else { (q, p) };
            (a, b, c, d) = (ps, ps + max, pt - (n + 1), pt - (n + 1) + max);
        }
    }

    println!("{} {} {} {}", a, b, c, d);
}
