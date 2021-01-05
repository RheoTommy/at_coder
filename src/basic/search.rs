pub fn around((i, j): (usize, usize), h: usize, w: usize) -> Vec<(usize, usize)> {
    let (i, j) = (i as isize, j as isize);
    vec![
        (i - 1, j - 1),
        (i - 1, j),
        (i - 1, j + 1),
        (i, j - 1),
        (i, j + 1),
        (i + 1, j - 1),
        (i + 1, j),
        (i + 1, j + 1),
    ]
    .into_iter()
    .filter(|(ii, jj)| (0..h as isize).contains(ii) && (0..w as isize).contains(jj))
    .map(|(ii, jj)| (ii as usize, jj as usize))
    .collect::<Vec<_>>()
}

pub fn next((i, j): (usize, usize), h: usize, w: usize) -> Vec<(usize, usize)> {
    let (i, j) = (i as isize, j as isize);
    vec![(i - 1, j), (i, j - 1), (i, j + 1), (i + 1, j)]
        .into_iter()
        .filter(|(ii, jj)| (0..h as isize).contains(ii) && (0..w as isize).contains(jj))
        .map(|(ii, jj)| (ii as usize, jj as usize))
        .collect::<Vec<_>>()
}
