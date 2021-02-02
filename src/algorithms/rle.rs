use cargo_snippet::snippet;

#[snippet("rle")]
pub fn rle<T: Eq + Clone>(s: &[T]) -> Vec<(T, usize)> {
    let mut res: Vec<(T, usize)> = vec![];
    for si in s {
        if res.is_empty() || &(res[res.len() - 1].0) != si {
            res.push((si.clone(), 1));
        } else {
            let l = res.len();
            let (_, k) = &mut res[l - 1];
            *k += 1;
        }
    }
    res
}
