use cargo_snippet::snippet;

/// セグ木にのせるMonoid
#[snippet("seg_tree")]
pub trait Monoid {
    type Item: std::fmt::Debug + Clone;
    /// 単位元
    fn id() -> Self::Item;
    /// 二項演算
    fn op(a: &Self::Item, b: &Self::Item) -> Self::Item;
}

#[snippet("seg_tree")]
pub struct SegTree<M: Monoid> {
    data: Vec<M::Item>,
    n: usize,
}

#[snippet("seg_tree")]
impl<M: Monoid> std::fmt::Debug for SegTree<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = &self.data[self.n - 1..];
        write!(f, "{:?}", v)
    }
}

#[snippet("seg_tree")]
impl<M: Monoid> SegTree<M> {
    /// すべて単位元で埋めた長さnのセグ木の生成
    pub fn new(n: usize) -> Self {
        let mut i = 1;
        while i < n {
            i *= 2;
        }
        let data = (0..2 * i - 1).map(|_| M::id()).collect::<Vec<_>>();
        Self { data, n: i }
    }

    /// O(n)でスライスからセグ木を生成
    pub fn from_slice(slice: &[M::Item]) -> Self {
        let mut i = 1;
        while i < slice.len() {
            i *= 2;
        }
        let mut data = vec![M::id(); 2 * i - 1];
        for j in 0..slice.len() {
            data[j + i - 1] = slice[j].clone();
        }
        if slice.len() != 1 {
            for j in (0..=(i - 2)).rev() {
                data[j] = M::op(&data[j * 2 + 1], &data[j * 2 + 2]);
            }
        }
        Self { data, n: i }
    }

    /// 一点更新
    pub fn set(&mut self, mut i: usize, x: M::Item) {
        i += self.n - 1;
        self.data[i] = x.clone();
        while i > 0 {
            i = (i - 1) / 2;
            self.data[i] = M::op(&self.data[i * 2 + 1], &self.data[i * 2 + 2]);
        }
    }

    /// 一点取得
    pub fn get(&self, mut i: usize) -> &M::Item {
        i += self.n - 1;
        &self.data[i]
    }

    /// 区間クエリ
    /// [l,r)の演算結果を求める
    pub fn fold(&self, mut l: usize, mut r: usize) -> M::Item {
        let mut l_ans = M::id();
        let mut r_ans = M::id();
        l += self.n - 1;
        r += self.n - 1;
        while l < r {
            if l & 1 == 0 {
                l_ans = M::op(&l_ans, &self.data[l]);
            }
            if r & 1 == 0 {
                r_ans = M::op(&self.data[r - 1], &r_ans);
                r -= 2;
            }
            l >>= 1;
            r >>= 1;
        }

        M::op(&l_ans, &r_ans)
    }
}

#[snippet("seg_tree")]
pub struct Max;
#[snippet("seg_tree")]
pub struct Min;
#[snippet("seg_tree")]
pub struct Sum;
#[snippet("seg_tree")]
pub struct Mul;
#[snippet("seg_tree")]
pub struct Xor;
#[snippet("seg_tree")]
impl Monoid for Max {
    type Item = i64;

    fn id() -> Self::Item {
        std::i64::MIN / 2
    }

    fn op(a: &Self::Item, b: &Self::Item) -> Self::Item {
        *a.max(b)
    }
}
#[snippet("seg_tree")]
impl Monoid for Min {
    type Item = i64;

    fn id() -> Self::Item {
        std::i64::MAX / 2
    }

    fn op(a: &Self::Item, b: &Self::Item) -> Self::Item {
        *a.min(b)
    }
}
#[snippet("seg_tree")]
impl Monoid for Sum {
    type Item = i64;

    fn id() -> Self::Item {
        0
    }

    fn op(a: &Self::Item, b: &Self::Item) -> Self::Item {
        a + b
    }
}
#[snippet("seg_tree")]
impl Monoid for Mul {
    type Item = i64;

    fn id() -> Self::Item {
        1
    }

    fn op(a: &Self::Item, b: &Self::Item) -> Self::Item {
        a * b
    }
}
#[snippet("seg_tree")]
impl Monoid for Xor {
    type Item = i64;

    fn id() -> Self::Item {
        0
    }

    fn op(a: &Self::Item, b: &Self::Item) -> Self::Item {
        a ^ b
    }
}

#[snippet("seg_tree")]
pub type MaxSegTree = SegTree<Max>;
#[snippet("seg_tree")]
pub type MinSegTree = SegTree<Min>;
#[snippet("seg_tree")]
pub type SumSegTree = SegTree<Sum>;
#[snippet("seg_tree")]
pub type MulSegTree = SegTree<Mul>;
#[snippet("seg_tree")]
pub type XorSegTree = SegTree<Xor>;
