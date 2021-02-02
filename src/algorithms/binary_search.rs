use cargo_snippet::snippet;

/// 関数Fを適用すると[false..false,true..true]の形にできるような配列において、初めてtrueになるIndexを返す
/// O(log n)
#[snippet("binary_search")]
#[snippet("lis")]
pub trait BinarySearch<T> {
    /// 存在しなかった場合は配列の長さを返す
    fn lower_bound(&self, f: impl Fn(&T) -> bool) -> usize;
    /// 存在しなかった場合はNoneを返す
    fn lower_bound_safe(&self, f: impl Fn(&T) -> bool) -> Option<usize>;
}

#[snippet("binary_search")]
#[snippet("lis")]
impl<T> BinarySearch<T> for [T] {
    /// 任意のランダムアクセス可能なスライスに対して実行可能な実装
    fn lower_bound(&self, f: impl Fn(&T) -> bool) -> usize {
        let mut left: isize = -1;
        let mut right = self.len() as isize;

        while right - left > 1 {
            let mid = (left + right) / 2;
            if f(&self[mid as usize]) {
                right = mid;
            } else {
                left = mid;
            }
        }

        right as usize
    }

    fn lower_bound_safe(&self, f: impl Fn(&T) -> bool) -> Option<usize> {
        let i = self.lower_bound(f);
        if i == self.len() {
            None
        } else {
            Some(i)
        }
    }
}

#[snippet("binary_search")]
pub fn lower_bound(l: i128, r: i128, f: impl Fn(i128) -> bool) -> i128 {
    let mut left = l;
    let mut right = r;
    while right - left > 1 {
        let mid = (left + right) / 2;
        if f(mid) {
            right = mid;
        } else {
            left = mid;
        }
    }
    right
}
