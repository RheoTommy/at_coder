use super::super::math::num_type::MaxIdent;
use super::binary_search::BinarySearch;
use cargo_snippet::snippet;

#[snippet("lis")]
pub fn lis<T: Ord + Clone + MaxIdent>(v: &[T]) -> (usize, Vec<T>) {
    let mut dp = vec![T::max_ident(); v.len()];
    for vi in v {
        let i = dp.lower_bound(|j| vi <= j);
        dp[i] = vi.clone();
    }
    let i = dp.lower_bound(|j| j >= &T::max_ident());
    (i, dp)
}
