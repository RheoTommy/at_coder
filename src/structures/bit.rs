use cargo_snippet::snippet;

#[snippet("bit")]
pub trait Monoid {
    type Item: std::fmt::Debug + Clone;
    fn id() -> Self::Item;
    fn op(a: &Self::Item, b: &Self::Item) -> Self::Item;
    fn sub(a: &Self::Item, b: &Self::Item) -> Self::Item;
}

#[snippet("bit")]
pub struct BIT<M: Monoid> {
    data: Vec<M::Item>,
}

#[snippet("bit")]
impl<M: Monoid> BIT<M> {
    pub fn new() -> Self {
        Self {
            data: vec![M::id(); 1],
        }
    }
    pub fn push(&mut self, x: &M::Item) {
        let n = self.data.len();
        let mut tmp = x.clone();
        let mut d = 1;
        let k = n & n.wrapping_neg();
        while d != k {
            tmp = M::op(&self.data[n - d], &tmp);
            d *= 2;
        }
        self.data.push(tmp.clone());
    }
    fn prefix_sum(&self, mut i: usize) -> M::Item {
        let mut res = M::id();
        while i != 0 {
            res = M::op(&res, &self.data[i]);
            i -= i & i.wrapping_neg();
        }
        res
    }
    pub fn add(&mut self, mut i: usize, x: &M::Item) {
        i += 1;
        while i < self.data.len() {
            let tmp = M::op(&self.data[i], x);
            self.data[i] = tmp;
            i += i & i.wrapping_neg();
        }
    }
    pub fn sum(&self, left: usize, right: usize) -> M::Item {
        M::sub(&self.prefix_sum(right), &self.prefix_sum(left))
    }
}

#[snippet("bit")]
pub struct Add;
#[snippet("bit")]
pub struct Xor;
#[snippet("bit")]
impl Monoid for Add {
    type Item = i64;

    fn id() -> Self::Item {
        0
    }

    fn op(a: &Self::Item, b: &Self::Item) -> Self::Item {
        a + b
    }

    fn sub(a: &Self::Item, b: &Self::Item) -> Self::Item {
        a - b
    }
}
#[snippet("bit")]
impl Monoid for Xor {
    type Item = i64;

    fn id() -> Self::Item {
        0
    }

    fn op(a: &Self::Item, b: &Self::Item) -> Self::Item {
        a ^ b
    }

    fn sub(a: &Self::Item, b: &Self::Item) -> Self::Item {
        a ^ b
    }
}

#[snippet("bit")]
pub type AddBIT = BIT<Add>;
#[snippet("bit")]
pub type XorBIT = BIT<Xor>;
