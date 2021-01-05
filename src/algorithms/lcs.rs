pub fn lcs<T: Eq + Clone>(s: &[T], t: &[T]) -> (usize, Vec<T>) {
    let mut dp = vec![vec![0; t.len() + 2]; s.len() + 2];
    for i in 0..=s.len() {
        for j in 0..=t.len() {
            if i < s.len() && j < t.len() && s[i] == t[j] {
                dp[i + 1][j + 1] = (dp[i + 1][j + 1]).max(dp[i][j] + 1);
            }
            dp[i + 1][j + 1] = (dp[i + 1][j + 1]).max(dp[i][j]);
            dp[i + 1][j] = (dp[i + 1][j]).max(dp[i][j]);
            dp[i][j + 1] = (dp[i][j + 1]).max(dp[i][j]);
        }
    }
    let mut res = Vec::with_capacity(dp[s.len()][t.len()]);
    let mut i = s.len();
    let mut j = t.len();
    while dp[i][j] != 0 {
        if i > 0 && dp[i - 1][j] == dp[i][j] {
            i -= 1;
            continue;
        } else if j > 0 && dp[i][j - 1] == dp[i][j] {
            j -= 1;
            continue;
        } else {
            res.push(s[i - 1].clone());
            i -= 1;
            j -= 1;
        }
    }
    res.reverse();
    (dp[s.len()][t.len()], res)
}