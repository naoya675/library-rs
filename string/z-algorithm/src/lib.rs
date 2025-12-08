pub fn z_algorithm<T: Copy + PartialEq>(s: &Vec<T>) -> Vec<usize> {
    if s.len() == 0 {
        return vec![];
    }
    let mut res = vec![0; s.len()];
    let mut i = 1;
    let mut j = 0;
    while i < s.len() {
        while i + j < s.len() && s[i + j] == s[j] {
            j += 1;
        }
        res[i] = j;
        if j == 0 {
            i += 1;
            continue;
        }
        let mut k = 1;
        while i + k < s.len() && k + res[k] < j {
            res[i + k] = res[k];
            k += 1;
        }
        i += k;
        j -= k;
    }
    res[0] = s.len();
    res
}

/*
pub fn z_algorithm<T: Copy + PartialEq>(s: &Vec<T>) -> Vec<usize> {
    if s.len() == 0 {
        return vec![];
    }
    let mut res = vec![0; s.len()];
    let mut i = 1;
    let mut j = 0;
    while i < s.len() {
        res[i] = if res[j] + j <= i { 0 } else { std::cmp::min(res[j] + j - i, res[i - j]) };
        while i + res[i] < s.len() && s[res[i]] == s[i + res[i]] {
            res[i] += 1;
        }
        if res[j] + j < res[i] + i {
            j = i;
        }
        i += 1;
    }
    res[0] = s.len();
    res
}
*/
