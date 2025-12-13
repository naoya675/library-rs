pub fn manacher<T: Copy + PartialEq>(s: &Vec<T>) -> Vec<usize> {
    if s.len() == 0 {
        return vec![];
    }
    let mut res = vec![0; s.len()];
    let mut i = 0;
    let mut j = 0;
    while i < s.len() {
        while i >= j && i + j < s.len() && s[i - j] == s[i + j] {
            j += 1;
        }
        res[i] = j;
        let mut k = 1;
        while i >= k && i + k < s.len() && k + res[i - k] < j {
            res[i + k] = res[i - k];
            k += 1;
        }
        i += k;
        j -= k;
    }
    res
}
