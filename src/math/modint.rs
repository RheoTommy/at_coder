pub const MOD: isize = 1000000007;
// pub const MOD: isize = 998244353;

/// 常に設定したModを取り続ける正整数型
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct ModInt {
    /// 保持する値
    value: isize,
}

impl ModInt {
    /// isizeを受け取り、ModIntに変換する
    pub fn new(n: isize) -> Self {
        let mut value = n % MOD;
        if value < 0 {
            value += MOD;
        }
        Self { value }
    }

    /// べき乗計算関数
    /// 二分累乗法を用いるため、計算量はO(log n)
    pub fn pow(self, n: usize) -> Self {
        match n {
            0 => ModInt::new(1),
            1 => self,
            n if n % 2 == 0 => (self * self).pow(n / 2),
            _ => self * self.pow(n - 1),
        }
    }

    /// 逆元を返す
    /// フェルマーの小定理より、べき乗を用いて計算するため、計算量はO(log MOD)
    pub fn inv(self) -> Self {
        self.pow((MOD - 2) as usize)
    }
}

impl std::fmt::Display for ModInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<usize> for ModInt {
    fn from(n: usize) -> Self {
        Self::new(n as isize)
    }
}

impl From<u64> for ModInt {
    fn from(n: u64) -> Self {
        Self::new(n as isize)
    }
}

impl From<u32> for ModInt {
    fn from(n: u32) -> Self {
        Self::new(n as isize)
    }
}

impl From<u16> for ModInt {
    fn from(n: u16) -> Self {
        Self::new(n as isize)
    }
}

impl From<u8> for ModInt {
    fn from(n: u8) -> Self {
        Self::new(n as isize)
    }
}

impl From<isize> for ModInt {
    fn from(n: isize) -> Self {
        Self::new(n)
    }
}

impl From<i64> for ModInt {
    fn from(n: i64) -> Self {
        Self::new(n as isize)
    }
}

impl From<i32> for ModInt {
    fn from(n: i32) -> Self {
        Self::new(n as isize)
    }
}

impl From<i16> for ModInt {
    fn from(n: i16) -> Self {
        Self::new(n as isize)
    }
}

impl From<i8> for ModInt {
    fn from(n: i8) -> Self {
        Self::new(n as isize)
    }
}

impl std::ops::Add for ModInt {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut tmp = self.value + rhs.value;
        if tmp >= MOD {
            tmp %= MOD;
        }

        Self { value: tmp }
    }
}

impl std::ops::AddAssign for ModInt {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl std::ops::Sub for ModInt {
    type Output = ModInt;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut tmp = self.value;
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

impl std::ops::SubAssign for ModInt {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl std::ops::Mul for ModInt {
    type Output = ModInt;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut tmp = self.value * rhs.value;
        if tmp >= MOD {
            tmp %= MOD;
        }

        Self { value: tmp }
    }
}

impl std::ops::MulAssign for ModInt {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl std::ops::Div for ModInt {
    type Output = ModInt;

    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.inv()
    }
}

impl std::ops::DivAssign for ModInt {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

#[test]
fn test_modint() {
    const MOD: isize = 1000000007;
    let add = (1000000000 + 1000000000) % MOD;
    let add_mod = ModInt::new(1000000000) + ModInt::new(1000000000);
    assert_eq!(add, add_mod.value);

    let rem = (1 - 2 + MOD) % MOD;
    let rem_mod = ModInt::new(1) - ModInt::new(2);
    assert_eq!(rem, rem_mod.value);

    let mul = (12345679 * 900) % MOD;
    let mul_mod = ModInt::new(12345679) * ModInt::new(900);
    assert_eq!(mul, mul_mod.value);

    let div = (5000 / 10) % MOD;
    let div_mod = ModInt::new(5000) / ModInt::new(10);
    assert_eq!(div, div_mod.value);
}

/// 二項係数を高速に計算するテーブルを作成する
/// 構築 O(N)
/// クエリ O(1)
/// メモリ O(N)
pub struct CombTable {
    fac: Vec<ModInt>,
    f_inv: Vec<ModInt>,
}

impl CombTable {
    /// O(N)で構築
    pub fn new(n: usize) -> Self {
        let mut fac = vec![ModInt::new(1); n + 1];
        let mut f_inv = vec![ModInt::new(1); n + 1];
        let mut inv = vec![ModInt::new(1); n + 1];
        inv[0] = ModInt::new(0);
        for i in 2..=n {
            fac[i] = fac[i - 1] * i.into();
            inv[i] =
                ModInt::new(MOD) - inv[(MOD % i as isize) as usize] * ModInt::new(MOD / i as isize);
            f_inv[i] = f_inv[i - 1] * inv[i];
        }
        Self { fac, f_inv }
    }

    /// nCkをO(1)で計算
    pub fn comb(&self, n: usize, k: usize) -> ModInt {
        if n < k || n * k == 0 {
            return ModInt::new(0);
        }
        self.fac[n] * (self.f_inv[k] * self.f_inv[n - k])
    }
}