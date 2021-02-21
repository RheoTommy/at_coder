use cargo_snippet::snippet;

#[snippet("coord_comp")]
pub fn compress<T:Clone + std::cmp::Ord + std::cmp::Eq>(
    v: &[T],
) -> (Vec<T>, std::collections::BTreeMap<T, usize>) {
    let mut v = v.iter().collect::<Vec<_>>();
    v.sort();
    v.dedup();
    let mut res = std::collections::BTreeMap::new();
    let mut zip = Vec::with_capacity(v.len());
    for (i, vi) in v.into_iter().enumerate() {
        zip.push(vi.clone());
        res.insert(vi.clone(), i);
    }
    (zip, res)
}
