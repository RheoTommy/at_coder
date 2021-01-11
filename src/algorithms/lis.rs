use super::binary_search::BinarySearch;
use cargo_snippet::snippet;

#[snippet("lis")]
pub fn lis<T: Ord + Clone>(v: &[T], max: T) -> (usize, Vec<T>) {
    let mut dp = vec![max.clone(); v.len()];
    for vi in v {
        let i = dp.lower_bound(|j| vi <= j);
        dp[i] = vi.clone();
    }
    let i = dp.lower_bound(|j| j >= &max);
    (i, dp)
}
