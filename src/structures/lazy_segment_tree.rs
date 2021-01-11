use cargo_snippet::snippet;

/// 遅延セグ木にのせるMonoid
#[snippet("lazy_seg_tree")]
pub trait Monoid {
    type Item: std::fmt::Debug + Clone;
    /// 単位元
    fn id() -> Self::Item;
    /// 二項演算
    fn op(a: &Self::Item, b: &Self::Item) -> Self::Item;
}

#[snippet("lazy_seg_tree")]
pub struct LazySegTree<M: Monoid, L: Monoid> {
    data: Vec<M::Item>,
    lazy: Vec<Option<L::Item>>,
    n: usize,
}

#[snippet("lazy_seg_tree")]
impl<M: Monoid, L: Monoid> LazySegTree<M, L> {
    /// すべて単位元で埋めた長さnの遅延セグ木の生成
    pub fn new(n: usize) -> Self {
        let mut i = 1;
        while i < n {
            i *= 2;
        }
        let data = (0..2 * i - 1).map(|_| M::id()).collect::<Vec<_>>();
        Self {
            data,
            lazy: vec![None; 2 * i - 1],
            n: i,
        }
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
        Self {
            data,
            lazy: vec![None; 2 * i - 1],
            n: i,
        }
    }

    /// そのノードの持つ区間の一番左のIndex
    fn start_of_section(&self, mut k: usize) -> usize {
        while k < self.n - 1 {
            k = k * 2 + 1;
        }
        k
    }

    /// そのノードの持つ区間の長さ
    fn len_of_section(&self, mut k: usize) -> usize {
        let mut j = k;
        while k < self.n - 1 {
            k = k * 2 + 1;
            j = j * 2 + 2;
        }
        j - k + 1
    }

    /// 遅延を評価し子ノードに伝播
    fn eval(&mut self, k: usize, apply: &impl Fn(&M::Item, &L::Item, usize, usize) -> M::Item) {
        if let Some(lv) = self.lazy[k].clone() {
            if k < self.n - 1 {
                self.lazy[k * 2 + 1] = Some(L::op(
                    &lv,
                    self.lazy[k * 2 + 1].as_ref().unwrap_or(&L::id()),
                ));
                self.lazy[k * 2 + 2] = Some(L::op(
                    &lv,
                    self.lazy[k * 2 + 2].as_ref().unwrap_or(&L::id()),
                ));
            }
            self.data[k] = apply(
                &self.data[k],
                &lv,
                self.start_of_section(k) + 1 - self.n,
                self.len_of_section(k),
            );
            self.lazy[k] = None;
        }
    }

    fn fold_sub(
        &mut self,
        start: usize,
        end: usize,
        k: usize,
        l: usize,
        r: usize,
        apply: &impl Fn(&M::Item, &L::Item, usize, usize) -> M::Item,
    ) -> M::Item {
        self.eval(k, apply);
        if end <= l || r <= start {
            M::id()
        } else if start <= l && r <= end {
            self.data[k].clone()
        } else {
            let vl = &self.fold_sub(start, end, k * 2 + 1, l, (l + r) / 2, apply);
            let vr = &self.fold_sub(start, end, k * 2 + 2, (l + r) / 2, r, apply);
            M::op(vl, vr)
        }
    }

    pub fn fold(
        &mut self,
        start: usize,
        end: usize,
        apply: &impl Fn(&M::Item, &L::Item, usize, usize) -> M::Item,
    ) -> M::Item {
        self.fold_sub(start, end, 0, 0, self.n, apply)
    }

    pub fn set_section(
        &mut self,
        start: usize,
        end: usize,
        lv: L::Item,
        apply: &impl Fn(&M::Item, &L::Item, usize, usize) -> M::Item,
    ) {
        self.set_section_sub(start, end, 0, 0, self.n, lv, apply)
    }

    fn set_section_sub(
        &mut self,
        start: usize,
        end: usize,
        k: usize,
        l: usize,
        r: usize,
        lv: L::Item,
        apply: &impl Fn(&M::Item, &L::Item, usize, usize) -> M::Item,
    ) {
        self.eval(k, apply);
        if start <= l && r <= end {
            self.lazy[k] = Some(lv.clone());
            self.eval(k, apply);
        } else if start < r && l < end {
            self.set_section_sub(start, end, k * 2 + 1, l, (l + r) / 2, lv.clone(), apply);
            self.set_section_sub(start, end, k * 2 + 2, (l + r) / 2, r, lv.clone(), apply);
            self.data[k] = M::op(&self.data[k * 2 + 1], &self.data[k * 2 + 2]);
        }
    }
}
