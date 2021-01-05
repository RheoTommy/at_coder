/// 順列を生成する構造体
pub struct Permutation<T: Clone> {
    v: Vec<T>,
    l: Vec<usize>,
    not_start: bool,
}

impl<T: Clone> Permutation<T> {
    /// 順列生成をする構造体の初期化
    pub fn new(v: &Vec<T>) -> Permutation<T> {
        Permutation {
            l: vec![0; v.len()],
            v: v.clone(),
            not_start: true,
        }
    }
}

impl<T: Clone> Iterator for Permutation<T> {
    type Item = Vec<T>;
    fn next(&mut self) -> Option<Vec<T>> {
        if self.not_start {
            self.not_start = false;
            return Some(self.v.clone());
        }
        for n in 0..self.v.len() {
            if self.l[n] < n {
                if (n + 1) % 2 == 1 {
                    self.v.swap(0, n);
                    self.l[n] += 1;
                } else {
                    self.v.swap(self.l[n], n);
                    self.l[n] += 1;
                }
                return Some(self.v.clone());
            } else {
                self.l[n] = 0
            }
        }
        return None;
    }
}

pub trait PermutationWithIterator<T: Clone> {
    fn permutations(&mut self) -> Permutation<T>;
}

impl<I> PermutationWithIterator<I::Item> for I
where
    I: Iterator,
    I::Item: Clone,
{
    /// 順列構造体のイテレーターに対する実装
    fn permutations(&mut self) -> Permutation<<I as Iterator>::Item> {
        Permutation::new(&self.collect::<Vec<_>>())
    }
}

/// 組み合わせ全列挙
pub fn comb(n: usize, k: usize) -> Vec<Vec<usize>> {
    let mut it = Vec::new();
    let mut c = (1 << k) - 1;
    while c < 1 << n {
        let mut v = Vec::new();
        for i in 0..n {
            if (c & 1 << i) != 0 {
                v.push(i);
            }
        }
        it.push(v.into());
        let x = c & -c;
        let y = c + x;
        c = ((c & !y) / x >> 1) | y;
    }
    it
}
