use lcp_array::lcp_array;
use suffix_array::suffix_array;

pub fn longest_common_substring<T: Copy + Ord>(s: &[T], t: &[T]) -> ((usize, usize), (usize, usize)) {
    let n = s.len();
    let mut st = vec![];
    st.extend(s.iter().map(|&c| Some(c)));
    st.push(None);
    st.extend(t.iter().map(|&c| Some(c)));

    let sa = suffix_array(&st);
    let lcp = lcp_array(&st, &sa);
    let mut res = ((0, 0), (0, 0));
    let mut max = 0;
    for i in 0..lcp.len() {
        let p = sa[i].min(sa[i + 1]);
        let q = sa[i].max(sa[i + 1]);
        if p < n && n < q && lcp[i] > max {
            max = lcp[i];
            let (l1, l2) = (p, q - (n + 1));
            res = ((l1, l1 + max), (l2, l2 + max));
        }
    }
    res
}
