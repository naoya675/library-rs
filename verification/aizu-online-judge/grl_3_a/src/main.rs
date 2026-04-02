// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_3_A

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
    });
    ll.build();
    let mut a = ll.articulation().iter().map(|&v| v).collect::<Vec<_>>();
    a.sort();

    for &v in &a {
        println!("{}", v);
    }
}
