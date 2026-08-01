use std::cmp::Reverse;

pub fn longest_increasing_subsequence<T: Ord>(a: &[T], strict: bool) -> Vec<usize> {
    let n = a.len();
    let mut dp = vec![];
    let mut prev = vec![None; n];
    for i in 0..n {
        let k = if strict {
            dp.partition_point(|&j| a[j] < a[i])
        } else {
            dp.partition_point(|&j| a[j] <= a[i])
        };
        if k > 0 {
            prev[i] = Some(dp[k - 1]);
        }
        if k == dp.len() {
            dp.push(i);
        } else {
            dp[k] = i;
        }
    }

    let mut res = vec![];
    let mut cur = dp.last().copied();
    while let Some(i) = cur {
        res.push(i);
        cur = prev[i];
    }
    res.reverse();
    res
}

pub fn longest_increasing_subsequence_2d<T: Ord + Copy>(a: &[(T, T)], strict: bool) -> Vec<usize> {
    let mut ord: Vec<usize> = (0..a.len()).collect();
    if strict {
        ord.sort_by_key(|&i| (a[i].0, Reverse(a[i].1)));
    } else {
        ord.sort_by_key(|&i| a[i]);
    }
    let y = ord.iter().map(|&i| a[i].1).collect::<Vec<_>>();

    longest_increasing_subsequence(&y, strict).iter().map(|&k| ord[k]).collect()
}
