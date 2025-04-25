#[derive(Debug)]
pub struct SuffixArray;

impl SuffixArray {
    fn sa_naive<T: Copy + Ord + PartialOrd>(s: &Vec<T>) -> Vec<usize> {
        let n = s.len();
        let mut sa = (0..n).collect::<Vec<usize>>();
        sa.sort_by(|&a, &b| {
            if a == b {
                return std::cmp::Ordering::Greater;
            }
            let mut i = a;
            let mut j = b;
            while i < n && j < n {
                if s[i] != s[j] {
                    if s[i] < s[j] {
                        return std::cmp::Ordering::Less;
                    } else {
                        return std::cmp::Ordering::Greater;
                    }
                }
                i += 1;
                j += 1;
            }
            if a == n {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        });
        sa
    }

    fn sa_doubling(s: &Vec<usize>) -> Vec<usize> {
        let n = s.len();
        let mut sa = (0..n).collect::<Vec<usize>>();
        let mut rank = s.clone();
        let mut rank_temp = vec![0; n];
        let mut k = 1;
        while k < n {
            let cmp = |&a: &usize, &b: &usize| {
                if rank[a] != rank[b] {
                    rank[a].cmp(&rank[b])
                } else {
                    let ra = if a + k < n { rank[a + k] as isize } else { -1 };
                    let rb = if b + k < n { rank[b + k] as isize } else { -1 };
                    if ra < rb {
                        std::cmp::Ordering::Less
                    } else {
                        std::cmp::Ordering::Greater
                    }
                }
            };
            sa.sort_by(cmp);
            rank_temp[sa[0]] = 0;
            for i in 1..n {
                rank_temp[sa[i]] = rank_temp[sa[i - 1]] + if cmp(&sa[i - 1], &sa[i]) == std::cmp::Ordering::Less { 1 } else { 0 };
            }
            std::mem::swap(&mut rank, &mut rank_temp);
            k <<= 1;
        }
        sa
    }

    fn sa_is(s: &Vec<usize>, upper: usize) -> Vec<usize> {
        todo!()
    }

    pub fn suffix_array<T: Copy + Ord + PartialOrd>(s: &Vec<T>) -> Vec<usize> {
        let n = s.len();
        let mut idx = (0..n).collect::<Vec<usize>>();
        idx.sort_by(|&a, &b| s[a].cmp(&s[b]));
        let mut t = vec![0; n];
        let mut now = 0;
        for i in 0..n {
            if i > 0 && s[idx[i - 1]] != s[idx[i]] {
                now += 1;
            }
            t[idx[i]] = now;
        }
        Self::sa_doubling(&t)
    }
}
