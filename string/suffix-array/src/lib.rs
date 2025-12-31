#[derive(Debug)]
pub struct SuffixArray {}

impl SuffixArray {
    fn sa_naive<T: Copy + Ord + PartialOrd>(s: &[T]) -> Vec<usize> {
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
            if a == n { std::cmp::Ordering::Less } else { std::cmp::Ordering::Greater }
        });
        sa
    }

    fn sa_doubling(s: &[usize]) -> Vec<usize> {
        let n = s.len();
        let mut sa = (0..n).collect::<Vec<usize>>();
        let mut rank = s.to_vec();
        let mut rank_temp = vec![0; n];
        let mut k = 1;
        while k < n {
            let cmp = |&a: &usize, &b: &usize| {
                if rank[a] != rank[b] {
                    rank[a].cmp(&rank[b])
                } else {
                    let ra = if a + k < n { rank[a + k] as isize } else { -1 };
                    let rb = if b + k < n { rank[b + k] as isize } else { -1 };
                    if ra < rb { std::cmp::Ordering::Less } else { std::cmp::Ordering::Greater }
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

    fn sa_is(s: &[usize], upper: usize) -> Vec<usize> {
        let n = s.len();
        if n == 0 {
            return vec![];
        }
        if n == 1 {
            return vec![0];
        }
        if n == 2 {
            return if s[0] < s[1] { vec![0, 1] } else { vec![1, 0] };
        }

        let mut sa = vec![None; n];
        let mut ls = vec![0; n];
        for i in (0..n - 1).rev() {
            if s[i] == s[i + 1] {
                ls[i] = ls[i + 1];
            } else if s[i] < s[i + 1] {
                ls[i] = 1;
            } else if s[i] > s[i + 1] {
                ls[i] = 0;
            }
        }
        let mut sum_l = vec![0; upper + 1];
        let mut sum_s = vec![0; upper + 1];
        for i in 0..n {
            if ls[i] == 0 {
                sum_s[s[i]] += 1;
            } else {
                sum_l[s[i] + 1] += 1;
            }
        }
        for i in 0..upper + 1 {
            sum_s[i] += sum_l[i];
            if i < upper {
                sum_l[i + 1] += sum_s[i];
            }
        }

        let induce = |lms: &[usize], sa: &mut Vec<Option<usize>>| {
            sa.fill(None);
            let mut buf = sum_s.clone();
            for &d in lms {
                if d == n {
                    continue;
                }
                sa[buf[s[d]]] = Some(d);
                buf[s[d]] += 1;
            }
            let mut buf = sum_l.clone();
            sa[buf[s[n - 1]]] = Some(n - 1);
            buf[s[n - 1]] += 1;
            for i in 0..n {
                if let Some(v) = sa[i] {
                    if v >= 1 && ls[v - 1] == 0 {
                        sa[buf[s[v - 1]]] = Some(v - 1);
                        buf[s[v - 1]] += 1;
                    }
                }
            }
            let mut buf = sum_l.clone();
            for i in (0..n).rev() {
                if let Some(v) = sa[i] {
                    if v >= 1 && ls[v - 1] != 0 {
                        buf[s[v - 1] + 1] -= 1;
                        sa[buf[s[v - 1] + 1]] = Some(v - 1);
                    }
                }
            }
        };

        let mut lms_map = vec![None; n + 1];
        let mut m = 0;
        for i in 1..n {
            if ls[i - 1] == 0 && ls[i] != 0 {
                lms_map[i] = Some(m);
                m += 1;
            }
        }
        let mut lms = vec![];
        lms.reserve(m);
        for i in 1..n {
            if ls[i - 1] == 0 && ls[i] != 0 {
                lms.push(i);
            }
        }
        induce(&lms, &mut sa);
        if m > 0 {
            let mut sorted_lms = vec![];
            sorted_lms.reserve(m);
            for &v_option in &sa {
                if let Some(v) = v_option {
                    if lms_map[v].is_some() {
                        sorted_lms.push(v);
                    }
                }
            }
            let mut rec_s = vec![0; m];
            let mut rec_upper = 0;
            rec_s[lms_map[sorted_lms[0]].unwrap()] = 0;
            for i in 1..m {
                let mut l = sorted_lms[i - 1];
                let mut r = sorted_lms[i];
                let end_l = if lms_map[l].unwrap() + 1 < m { lms[lms_map[l].unwrap() + 1] } else { n };
                let end_r = if lms_map[r].unwrap() + 1 < m { lms[lms_map[r].unwrap() + 1] } else { n };
                let mut same = true;
                if end_l - l != end_r - r {
                    same = false;
                } else {
                    while l < end_l {
                        if s[l] != s[r] {
                            break;
                        }
                        l += 1;
                        r += 1;
                    }
                    if l == n || s[l] != s[r] {
                        same = false;
                    }
                }
                if !same {
                    rec_upper += 1;
                }
                rec_s[lms_map[sorted_lms[i]].unwrap()] = rec_upper;
            }
            let rec_sa = SuffixArray::sa_is(&rec_s, rec_upper);
            for i in 0..m {
                sorted_lms[i] = lms[rec_sa[i]];
            }
            induce(&sorted_lms, &mut sa);
        }
        sa.into_iter().map(|a| a.unwrap()).collect()
    }

    pub fn suffix_array<T: Copy + Ord + PartialOrd>(s: &[T]) -> Vec<usize> {
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
        Self::sa_is(&t, now)
    }
}
