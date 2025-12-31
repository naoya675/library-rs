#[derive(Debug)]
pub struct LCPArray {}

impl LCPArray {
    pub fn lcp_array<T: Copy + Ord + PartialOrd>(s: &[T], sa: &[usize]) -> Vec<usize> {
        assert!(s.len() == sa.len());
        let n = s.len();
        let mut rank = vec![0; n];
        for i in 0..n {
            assert!(sa[i] < n);
            rank[sa[i]] = i;
        }
        let mut lcp = vec![0; n - 1];
        let mut h = 0;
        for i in 0..n {
            if h > 0 {
                h -= 1;
            }
            if rank[i] == 0 {
                continue;
            }
            let j = sa[rank[i] - 1];
            while j + h < n && i + h < n {
                if s[j + h] != s[i + h] {
                    break;
                }
                h += 1;
            }
            lcp[rank[i] - 1] = h;
        }
        lcp
    }
}
