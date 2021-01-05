pub fn coord_comp<
    T: std::hash::Hash + Clone + std::cmp::Ord + std::cmp::PartialEq + std::cmp::Eq,
>(
    v: &[T],
) -> (Vec<T>, std::collections::HashMap<T, usize>) {
    let mut v = v.iter().collect::<Vec<_>>();
    v.sort();
    v.dedup();
    let mut res = std::collections::HashMap::new();
    let mut zip = Vec::with_capacity(v.len());
    for (i, vi) in v.into_iter().enumerate() {
        zip.push(vi.clone());
        res.insert(vi.clone(), i);
    }
    (zip, res)
}
