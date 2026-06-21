pub fn partition_number<T>(n: usize, k: usize) -> Vec<T>
where
    T: Copy + From<u64> + std::ops::Add<T, Output = T>,
{
    let mut dp = vec![T::from(0); n + 1];
    dp[0] = T::from(1);
    for j in 1..=k {
        for i in j..=n {
            dp[i] = dp[i] + dp[i - j];
        }
    }
    dp
}
