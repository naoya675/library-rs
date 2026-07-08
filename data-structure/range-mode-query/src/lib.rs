#[derive(Debug, Clone, Copy)]
struct BNode {
    value: usize,
    rank: usize,
}

#[derive(Debug, Clone, Copy)]
struct SNode {
    mode: usize,
    freq: usize,
}

#[derive(Debug, Clone)]
pub struct RangeModeQuery {
    n: usize,
    t: usize,
    a: Vec<i64>,
    b: Vec<BNode>,
    q: Vec<Vec<usize>>,
    s: Vec<Vec<SNode>>,
}

impl RangeModeQuery {
    pub fn new(a: &[i64]) -> Self {
        let n = a.len();
        let mut sorted = a.to_vec();
        sorted.sort();
        sorted.dedup();
        let m = sorted.len();

        let mut b = vec![];
        let mut q = vec![vec![]; m];
        for (i, v) in a.iter().enumerate() {
            let value = sorted.binary_search(v).unwrap();
            let rank = q[value].len();
            b.push(BNode { value, rank });
            q[value].push(i);
        }

        let t = (n as f64).sqrt() as usize;
        let blocks = (n + t - 1) / t;

        let mut s = vec![vec![]; blocks];
        let mut freq = vec![0; m];
        for f in 0..blocks {
            freq.fill(0);
            let mut cur = SNode { mode: 0, freq: 0 };
            for l in (f + 1)..=blocks {
                for i in (l - 1) * t..(l * t).min(n) {
                    let e = b[i].value;
                    freq[e] += 1;
                    if freq[e] > cur.freq {
                        cur = SNode { mode: e, freq: freq[e] };
                    }
                }
                s[f].push(cur);
            }
        }
        Self { n, t, a: sorted, b, q, s }
    }

    // [l, r)
    pub fn query(&self, l: usize, r: usize) -> (i64, usize) {
        assert!(l < r && r <= self.n);
        let t = self.t;
        let block_l = (l + t - 1) / t;
        let block_r = r / t;

        if block_l >= block_r {
            let mut cur = SNode { mode: 0, freq: 0 };
            for x in l..r {
                let BNode { value: v, rank: k } = self.b[x];
                while k + cur.freq < self.q[v].len() && self.q[v][k + cur.freq] < r {
                    cur.mode = v;
                    cur.freq += 1;
                }
            }
            return (self.a[cur.mode], cur.freq);
        }

        let mut cur = self.s[block_l][block_r - 1 - block_l];
        for x in l..block_l * t {
            let BNode { value: v, rank: k } = self.b[x];
            while k + cur.freq < self.q[v].len() && self.q[v][k + cur.freq] < r {
                cur.mode = v;
                cur.freq += 1;
            }
        }
        for x in block_r * t..r {
            let BNode { value: v, rank: k } = self.b[x];
            while cur.freq <= k && self.q[v][k - cur.freq] >= l {
                cur.mode = v;
                cur.freq += 1;
            }
        }

        (self.a[cur.mode], cur.freq)
    }
}
