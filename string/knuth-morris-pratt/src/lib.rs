pub fn kmp_table<T: Copy + PartialEq>(pattern: &[T]) -> Vec<usize> {
    if pattern.len() == 0 {
        return vec![];
    }
    let mut i = 1;
    let mut j = 0;
    let mut failure = vec![0; pattern.len()];
    while i < pattern.len() {
        if pattern[i] == pattern[j] {
            failure[i] = j + 1;
            i += 1;
            j += 1;
        } else {
            if j == 0 {
                failure[i] = 0;
                i += 1;
            } else {
                j = failure[j - 1];
            }
        }
    }
    failure
}

pub fn kmp<T: Copy + PartialEq>(target: &[T], pattern: &[T]) -> Vec<usize> {
    let failure = kmp_table(pattern);
    let mut j = 0;
    let mut res = vec![];
    for i in 0..target.len() {
        while j > 0 && target[i] != pattern[j] {
            j = failure[j - 1];
        }
        if target[i] == pattern[j] {
            j += 1;
        }
        if j == pattern.len() {
            res.push(i + 1 - pattern.len());
            j = failure[j - 1];
        }
    }
    res
}
