#![allow(unused_macros)]
#![allow(dead_code)]
#![allow(unused_imports)]

// # ファイル構成
// - use宣言
// - libモジュール
// - main関数
// - basicモジュール
//
// 常に使うテンプレートライブラリはbasicモジュール内にあります。
// 問題に応じて使うライブラリlibモジュール内にコピペしています。
// ライブラリのコードはこちら → https://github.com/RheoTommy/at_coder
// Twitterはこちら → https://twitter.com/RheoTommy

use std::io::{stdout, BufWriter, Write};
use std::{collections::*, vec};

use itertools::Itertools;

use crate::basic::*;
use crate::lib::*;

pub mod lib {
    pub const MOD: i32 = MOD998244353;
    pub const MOD1000000007: i32 = 1000000007;
    pub const MOD998244353: i32 = 998244353;
    /// 常に設定したModを取り続ける正整数型
    #[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
    pub struct ModInt {
        /// 保持する値
        value: i32,
    }
    impl ModInt {
        /// isizeを受け取り、ModIntに変換する
        pub fn new(n: i32) -> Self {
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
                0 => ModInt::new(1),
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
    impl std::fmt::Display for ModInt {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.value)
        }
    }
    impl<T: Into<Num>> From<T> for ModInt {
        fn from(t: T) -> Self {
            let n: Num = t.into();
            Self::new((n.0 % MOD as i128) as i32)
        }
    }
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
    impl<T: Into<ModInt>> std::ops::AddAssign<T> for ModInt {
        fn add_assign(&mut self, rhs: T) {
            *self = *self + rhs.into()
        }
    }
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
    impl<T: Into<ModInt>> std::ops::SubAssign<T> for ModInt {
        fn sub_assign(&mut self, rhs: T) {
            *self = *self - rhs.into();
        }
    }
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
    impl<T: Into<ModInt>> std::ops::MulAssign<T> for ModInt {
        fn mul_assign(&mut self, rhs: T) {
            *self = *self * rhs.into();
        }
    }
    impl<T: Into<ModInt>> std::ops::Div<T> for ModInt {
        type Output = ModInt;
        fn div(self, rhs: T) -> Self::Output {
            let rhs: ModInt = rhs.into();
            self * rhs.inv()
        }
    }
    impl<T: Into<ModInt>> std::ops::DivAssign<T> for ModInt {
        fn div_assign(&mut self, rhs: T) {
            let rhs = rhs.into();
            *self = *self / rhs;
        }
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
                fac[i] = fac[i - 1] * i;
                inv[i] =
                    ModInt::new(MOD) - inv[(MOD % i as i32) as usize] * ModInt::new(MOD / i as i32);
                f_inv[i] = f_inv[i - 1] * inv[i];
            }
            Self { fac, f_inv }
        }
        /// nCkをO(1)で計算
        pub fn comb(&self, n: usize, k: usize) -> ModInt {
            if n < k {
                return ModInt::new(0);
            }
            self.fac[n] * (self.f_inv[k] * self.f_inv[n - k])
        }
    }
    /// 加減算
    pub trait AddSubIdent {
        fn add_sub_ident() -> Self;
    }
    /// 乗除算
    pub trait MulDivIdent {
        fn mul_div_ident() -> Self;
    }
    /// GCD
    pub trait GCDIdent {
        fn gcd_ident() -> Self;
    }
    /// LCM
    pub trait LCMIdent {
        fn lcm_ident() -> Self;
    }
    /// Max
    pub trait MaxIdent {
        fn max_ident() -> Self;
    }
    /// Min
    pub trait MinIdent {
        fn min_ident() -> Self;
    }
    /// Xor
    pub trait XorIdent {
        fn xor_ident() -> Self;
    }
    /// 変換用
    pub struct Num(pub i128);
    macro_rules! impl_num_from {
        ($ t : ty ) => {
            impl From<Num> for $t {
                fn from(t: Num) -> $t {
                    t.0 as $t
                }
            }
        };
    }
    impl_num_from!(i8);
    impl_num_from!(i16);
    impl_num_from!(i32);
    impl_num_from!(i64);
    impl_num_from!(i128);
    impl_num_from!(isize);
    impl_num_from!(u8);
    impl_num_from!(u16);
    impl_num_from!(u32);
    impl_num_from!(u64);
    impl_num_from!(u128);
    impl_num_from!(usize);
    macro_rules! impl_num_into {
        ($ t : ty ) => {
            impl Into<Num> for $t {
                fn into(self) -> Num {
                    Num(self as i128)
                }
            }
        };
    }
    impl_num_into!(i8);
    impl_num_into!(i16);
    impl_num_into!(i32);
    impl_num_into!(i64);
    impl_num_into!(i128);
    impl_num_into!(isize);
    impl_num_into!(u8);
    impl_num_into!(u16);
    impl_num_into!(u32);
    impl_num_into!(u64);
    impl_num_into!(u128);
    impl_num_into!(usize);
    impl<T: From<Num>> AddSubIdent for T {
        fn add_sub_ident() -> Self {
            Num(0).into()
        }
    }
    impl<T: From<Num>> MulDivIdent for T {
        fn mul_div_ident() -> Self {
            Num(1).into()
        }
    }
    impl<T: From<Num>> GCDIdent for T {
        fn gcd_ident() -> Self {
            Num(0).into()
        }
    }
    impl<T: From<Num>> LCMIdent for T {
        fn lcm_ident() -> Self {
            Num(1).into()
        }
    }
    macro_rules! impl_min {
        ($ t : ident ) => {
            impl MinIdent for $t {
                fn min_ident() -> Self {
                    std::$t::MIN
                }
            }
        };
    }
    impl_min!(i8);
    impl_min!(i16);
    impl_min!(i32);
    impl_min!(i64);
    impl_min!(i128);
    impl_min!(isize);
    impl_min!(u8);
    impl_min!(u16);
    impl_min!(u32);
    impl_min!(u64);
    impl_min!(u128);
    impl_min!(usize);
    macro_rules! impl_max {
        ($ t : ident ) => {
            impl MaxIdent for $t {
                fn max_ident() -> Self {
                    std::$t::MAX
                }
            }
        };
    }
    impl_max!(i8);
    impl_max!(i16);
    impl_max!(i32);
    impl_max!(i64);
    impl_max!(i128);
    impl_max!(isize);
    impl_max!(u8);
    impl_max!(u16);
    impl_max!(u32);
    impl_max!(u64);
    impl_max!(u128);
    impl_max!(usize);
    impl<T: From<Num>> XorIdent for T {
        fn xor_ident() -> Self {
            Num(0).into()
        }
    }
}

fn main() {
    let out = stdout();
    let mut writer = BufWriter::new(out.lock());
    let mut sc = Scanner::new();
    let h = sc.next_usize();
    let w = sc.next_usize();
    let k = sc.next_uint();
    let c = (0..k)
        .map(|_| (sc.next_usize() - 1, sc.next_usize() - 1, sc.next_char()))
        .collect::<Vec<_>>();

    let mut grid = vec![vec![None; w]; h];
    for &(i, j, c) in &c {
        grid[i][j] = Some(c);
    }
    let mut dp = vec![vec![ModInt::new(0); w]; h];
    dp[0][0] = ModInt::new(3).pow((h * w - k as usize) as u128);
    let key = ModInt::new(3).inv();
    for i in 0..h {
        for j in 0..w {
            let tmp = dp[i][j];
            if let Some(c) = grid[i][j] {
                match c {
                    'D' => {
                        if i + 1 != h {
                            dp[i + 1][j] += tmp;
                        }
                    }
                    'R' => {
                        if j + 1 != w {
                            dp[i][j + 1] += tmp;
                        }
                    }
                    'X' => {
                        if i + 1 != h {
                            dp[i + 1][j] += tmp;
                        }
                        if j + 1 != w {
                            dp[i][j + 1] += tmp;
                        }
                    }
                    _ => unreachable!(),
                }
            } else {
                if i + 1 != h {
                    dp[i + 1][j] += tmp * 2 * key;
                }
                if j + 1 != w {
                    dp[i][j + 1] += tmp * 2 * key;
                }
            }
        }
    }

    writeln!(writer, "{}", dp[h - 1][w - 1]).unwrap();
}

pub mod basic {
    pub const U_INF: usize = 1 << 60;
    pub const I_INF: isize = 1 << 60;

    pub struct Scanner {
        buf: std::collections::VecDeque<String>,
        reader: std::io::BufReader<std::io::Stdin>,
    }

    impl Scanner {
        pub fn new() -> Self {
            Self {
                buf: std::collections::VecDeque::new(),
                reader: std::io::BufReader::new(std::io::stdin()),
            }
        }

        fn scan_line(&mut self) {
            use std::io::BufRead;
            let mut flag = 0;
            while self.buf.is_empty() {
                let mut s = String::new();
                self.reader.read_line(&mut s).unwrap();
                let mut iter = s.split_whitespace().peekable();
                if iter.peek().is_none() {
                    if flag >= 5 {
                        panic!("There is no input!");
                    }
                    flag += 1;
                    continue;
                }

                for si in iter {
                    self.buf.push_back(si.to_string());
                }
            }
        }

        pub fn next<T: std::str::FromStr>(&mut self) -> T {
            self.scan_line();
            self.buf
                .pop_front()
                .unwrap()
                .parse()
                .unwrap_or_else(|_| panic!("Couldn't parse!"))
        }

        pub fn next_usize(&mut self) -> usize {
            self.next()
        }

        pub fn next_int(&mut self) -> i128 {
            self.next()
        }

        pub fn next_uint(&mut self) -> u128 {
            self.next()
        }

        pub fn next_chars(&mut self) -> Vec<char> {
            self.next::<String>().chars().collect()
        }

        pub fn next_string(&mut self) -> String {
            self.next()
        }

        pub fn next_char(&mut self) -> char {
            self.next()
        }

        pub fn next_float(&mut self) -> f64 {
            self.next()
        }
    }
}
