// verification-helper: PROBLEM https://judge.yosupo.jp/problem/two_sat

use fast_io::{Output, input, output};

use two_sat::TwoSat;

fn main() {
    std::thread::Builder::new()
        .stack_size(256 * 1024 * 1024)
        .spawn(actual_main)
        .unwrap()
        .join()
        .unwrap();
}

fn actual_main() {
    input! {
        _p: String,
        _cnf: String,
        n: usize,
        m: usize,
        ab: [(i64, i64, i64); m],
    }
    let mut out = Output::new();

    let mut ts = TwoSat::new(n + 1);
    for (a, b, _) in ab {
        ts.add_clause(a.unsigned_abs() as usize, a > 0, b.unsigned_abs() as usize, b > 0);
    }

    if ts.satisfiable() {
        output!(out, "s SATISFIABLE");
        let ans = ts.answer();
        out.print("v");
        out.print(" ");
        out.print_iter((1..=n).map(|i| if ans[i] { i as i64 } else { -(i as i64) }), " ");
        out.print(" ");
        out.println("0");
    } else {
        output!(out, "s UNSATISFIABLE");
    }
}
