// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_1_B

use proconio::input;

use union_find_with_potential::UnionFindWithPotential;

fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut uf = UnionFindWithPotential::<i64>::new_default(n);
    for _ in 0..q {
        input! { query: usize, }
        match query {
            0 => {
                input! { x: usize, y: usize, z: i64, }
                uf.merge(x, y, z);
            }
            1 => {
                input! { x: usize, y: usize, }
                if uf.same(x, y) {
                    println!("{}", uf.diff(x, y));
                } else {
                    println!("?");
                }
            }
            _ => unreachable!(),
        }
    }
}
