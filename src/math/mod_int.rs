use super::num_type::*;
use cargo_snippet::snippet;

#[snippet("mod_int")]
pub const MOD: i32 = MOD1000000007;
#[snippet("mod_int")]
pub const MOD1000000007: i32 = 1000000007;
#[snippet("mod_int")]
pub const MOD998244353: i32 = 998244353;

/// 常に設定したModを取り続ける正整数型
#[snippet("mod_int")]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct ModInt {
    /// 保持する値
    value: i32,
}

#[snippet("mod_int")]
impl ModInt {
    pub fn new<T: Into<ModInt>>(n: T) -> Self {
        n.into()
    }

    /// isizeを受け取り、ModIntに変換する
    fn new_inner(n: i32) -> Self {
        let mut value = n % MOD;
        if value < 0 {
            value += MOD;
        }
        Self { value }
    }

    /// べき乗計算関数
    /// 二分累乗法を用いるため、計算量はO(log n)
    pub fn pow(self, n: u128) -> Self {
        match n {
            0 => ModInt::new_inner(1),
            1 => self,
            n if n % 2 == 0 => (self * self).pow(n / 2),
            _ => self * self.pow(n - 1),
        }
    }

    /// 逆元を返す
    /// フェルマーの小定理より、べき乗を用いて計算するため、計算量はO(log MOD)
    pub fn inv(self) -> Self {
        self.pow((MOD - 2) as u128)
    }
}

#[snippet("mod_int")]
impl std::fmt::Display for ModInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[snippet("mod_int")]
impl<T: Into<Num>> From<T> for ModInt {
    fn from(t: T) -> Self {
        let n: Num = t.into();
        Self::new_inner((n.0 % MOD as i128) as i32)
    }
}

#[snippet("mod_int")]
impl<T: Into<ModInt>> std::ops::Add<T> for ModInt {
    type Output = ModInt;

    fn add(self, rhs: T) -> Self::Output {
        let mut tmp = self.value;
        let t: ModInt = rhs.into();
        tmp += t.value;
        if tmp >= MOD {
            tmp -= MOD;
        }
        Self { value: tmp }
    }
}

#[snippet("mod_int")]
impl<T: Into<ModInt>> std::ops::AddAssign<T> for ModInt {
    fn add_assign(&mut self, rhs: T) {
        *self = *self + rhs.into()
    }
}

#[snippet("mod_int")]
impl<T: Into<ModInt>> std::ops::Sub<T> for ModInt {
    type Output = ModInt;

    fn sub(self, rhs: T) -> Self::Output {
        let mut tmp = self.value;
        let rhs: ModInt = rhs.into();
        if tmp < rhs.value {
            tmp += MOD;
        }
        tmp -= rhs.value;
        if tmp >= MOD {
            tmp %= MOD;
        }

        Self { value: tmp }
    }
}

#[snippet("mod_int")]
impl<T: Into<ModInt>> std::ops::SubAssign<T> for ModInt {
    fn sub_assign(&mut self, rhs: T) {
        *self = *self - rhs.into();
    }
}

#[snippet("mod_int")]
impl<T: Into<ModInt>> std::ops::Mul<T> for ModInt {
    type Output = ModInt;

    fn mul(self, rhs: T) -> Self::Output {
        let rhs: ModInt = rhs.into();
        let mut tmp = self.value as i128 * rhs.value as i128;
        if tmp >= MOD as i128 {
            tmp %= MOD as i128;
        }

        Self { value: tmp as i32 }
    }
}

#[snippet("mod_int")]
impl<T: Into<ModInt>> std::ops::MulAssign<T> for ModInt {
    fn mul_assign(&mut self, rhs: T) {
        *self = *self * rhs.into();
    }
}

#[snippet("mod_int")]
impl<T: Into<ModInt>> std::ops::Div<T> for ModInt {
    type Output = ModInt;

    fn div(self, rhs: T) -> Self::Output {
        let rhs: ModInt = rhs.into();
        self * rhs.inv()
    }
}

#[snippet("mod_int")]
impl<T: Into<ModInt>> std::ops::DivAssign<T> for ModInt {
    fn div_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        *self = *self / rhs;
    }
}

#[test]
fn test_mod_int() {
    const MOD: i32 = 1000000007;
    let add = (1000000000 + 1000000000) % MOD;
    let add_mod = ModInt::new_inner(1000000000) + ModInt::new_inner(1000000000);
    let add_impl = ModInt::new_inner(1000000000) + 1000000000;
    assert_eq!(add, add_mod.value);
    assert_eq!(add_mod, add_impl);

    let rem = (1 - 2 + MOD) % MOD;
    let rem_mod = ModInt::new_inner(1) - ModInt::new_inner(2);
    assert_eq!(rem, rem_mod.value);

    let mul = (12345679 * 90) % MOD;
    let mul_mod = ModInt::new_inner(12345679) * ModInt::new_inner(90);
    assert_eq!(mul, mul_mod.value);

    let div = (5000 / 10) % MOD;
    let div_mod = ModInt::new_inner(5000) / ModInt::new_inner(10);
    assert_eq!(div, div_mod.value);
}

/// 二項係数を高速に計算するテーブルを作成する
/// 構築 O(N)
/// クエリ O(1)
/// メモリ O(N)
#[snippet("mod_int")]
pub struct CombTable {
    fac: Vec<ModInt>,
    f_inv: Vec<ModInt>,
}

#[snippet("mod_int")]
impl CombTable {
    /// O(N)で構築
    pub fn new(n: usize) -> Self {
        let mut fac = vec![ModInt::new_inner(1); n + 1];
        let mut f_inv = vec![ModInt::new_inner(1); n + 1];
        let mut inv = vec![ModInt::new_inner(1); n + 1];
        inv[0] = ModInt::new_inner(0);
        for i in 2..=n {
            fac[i] = fac[i - 1] * i;
            inv[i] = ModInt::new_inner(MOD)
                - inv[(MOD % i as i32) as usize] * ModInt::new_inner(MOD / i as i32);
            f_inv[i] = f_inv[i - 1] * inv[i];
        }
        Self { fac, f_inv }
    }

    /// nCkをO(1)で計算
    pub fn comb(&self, n: usize, k: usize) -> ModInt {
        if n < k {
            return ModInt::new_inner(0);
        }
        self.fac[n] * (self.f_inv[k] * self.f_inv[n - k])
    }
}

#[test]
fn test_comb() {
    let comb = CombTable::new(10000);
    assert_eq!(comb.comb(10, 0), 1.into());
}
