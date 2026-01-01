// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_3_B

use proconio::input;

use low_link::LowLink;

fn main() {
    input! {
        v: usize,
        e: usize,
        st: [(usize, usize); e],
    }
    let mut ll = LowLink::new(v);
    st.iter().for_each(|&(s, t)| {
        ll.add_edge(s, t);
        ll.add_edge(t, s);
    });
    ll.build();
    let mut b = ll.bridge().iter().map(|&(s, t)| if s < t { (s, t) } else { (t, s) }).collect::<Vec<_>>();
    b.sort();

    for (s, t) in &b {
        println!("{} {}", s, t);
    }
}
