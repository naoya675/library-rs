#[derive(Debug)]
pub struct RunLendth {}

impl RunLendth {
    pub fn encode<T: Copy + PartialEq>(s: &[T]) -> Vec<(T, usize)> {
        let mut res = vec![];
        if s.len() == 0 {
            return res;
        }
        let mut i = 0;
        while i < s.len() {
            let mut j = i;
            while j < s.len() && s[i] == s[j] {
                j += 1;
            }
            res.push((s[i], j - i));
            i = j;
        }
        res
    }

    pub fn decode<T: Copy>(s: &[(T, usize)]) -> Vec<T> {
        let mut res = vec![];
        if s.len() == 0 {
            return res;
        }
        for &(value, c) in s {
            res.extend(vec![value; c]);
        }
        res
    }
}
