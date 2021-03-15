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

use std::collections::*;
use std::io::{stdout, BufWriter, Write};

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
        pub fn pow(self, n: u64) -> Self {
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
            self.pow((MOD - 2) as u64)
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
            Self::new_inner((n.0 % MOD as i64) as i32)
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
            let mut tmp = self.value as i64 * rhs.value as i64;
            if tmp >= MOD as i64 {
                tmp %= MOD as i64;
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
    pub struct Num(pub i64);
    macro_rules! impl_num_from {($ ($ t : ty ) ,* ) => {$ (impl From < Num > for $ t {fn from (t : Num ) -> $ t {t . 0 as $ t } } ) * } ; }
    impl_num_from!(i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, isize, usize);
    macro_rules! impl_num_into {($ ($ t : ty ) ,* ) => {$ (impl Into < Num > for $ t {fn into (self ) -> Num {Num (self as i64 ) } } ) * } ; }
    impl_num_into!(i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, isize, usize);
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
    macro_rules! impl_min {($ ($ t : ident ) ,* ) => {$ (impl MinIdent for $ t {fn min_ident () -> Self {std ::$ t :: MIN } } ) * } ; }
    impl_min!(i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, isize, usize);
    macro_rules! impl_max {($ ($ t : ident ) ,* ) => {$ (impl MaxIdent for $ t {fn max_ident () -> Self {std ::$ t :: MAX } } ) * } ; }
    impl_max!(i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, isize, usize);
    impl<T: From<Num>> XorIdent for T {
        fn xor_ident() -> Self {
            Num(0).into()
        }
    }

    /// UnionFind構造体
    pub struct UnionFindTree {
        /// 頂点`i`の親を格納する配列
        parents: Vec<usize>,
        /// 頂点`i`が親であるときのその木の頂点数
        sizes: Vec<usize>,
        /// 重み付きUnionFindを使う際の重みの格納配列
        weights: Vec<isize>,
        /// 頂点`i`が属する木がループを持っているかどうか
        has_loops: Vec<bool>,
    }

    impl UnionFindTree {
        /// UnionFind初期化
        /// 計算量はO(n)
        pub fn new(n: usize) -> Self {
            let parents = (0..n).collect();
            let sizes = vec![1; n];
            let weights = vec![0; n];
            let has_loops = vec![false; n];
            UnionFindTree {
                parents,
                sizes,
                weights,
                has_loops,
            }
        }
        /// 親を再帰的に求め、途中の計算結果をもとに親の書き換えを行う関数
        /// 計算量はO(a(n)))
        pub fn root(&mut self, x: usize) -> usize {
            if self.parents[x] == x {
                x
            } else {
                let tmp = self.root(self.parents[x]);
                self.weights[x] += self.weights[self.parents[x]];
                self.parents[x] = tmp;
                tmp
            }
        }
        pub fn size(&mut self, x: usize) -> usize {
            let y = self.root(x);
            self.sizes[y]
        }
        pub fn has_loop(&mut self, x: usize) -> bool {
            let y = self.root(x);
            self.has_loops[y]
        }
        /// 2つの頂点が同じ木に属しているかの判定
        /// `self.root()`を呼び出すため、`&mut self`を引数に取る。そのため、命名に`is_`を使っていない
        /// 計算量はO(a(n))
        pub fn same(&mut self, x: usize, y: usize) -> bool {
            self.root(x) == self.root(y)
        }
        /// 重み付きUnionFindを考える際のUnite関数
        /// 計算量はO(a(n))
        pub fn unite_with_weight(&mut self, x: usize, y: usize, w: isize) {
            let root_x = self.root(x);
            let root_y = self.root(y);
            if self.same(x, y) {
                self.has_loops[root_x] = true;
                self.has_loops[root_y] = true;
            } else if self.sizes[root_x] >= self.sizes[root_y] {
                self.parents[root_y] = root_x;
                self.has_loops[root_x] |= self.has_loops[root_y];
                self.sizes[root_x] += self.sizes[root_y];
                self.weights[root_y] = -w - self.weights[y] + self.weights[x];
            } else {
                self.parents[root_x] = root_y;
                self.has_loops[root_y] |= self.has_loops[root_x];
                self.sizes[root_y] += self.sizes[root_x];
                self.weights[root_x] = w + self.weights[y] - self.weights[x];
            }
        }
        /// 重みを考慮しない際のUnite関数
        /// 重みとして0を与えているだけであり、計算量は同じくO(a(n))
        pub fn unite(&mut self, x: usize, y: usize) {
            self.unite_with_weight(x, y, 0);
        }
        /// 重み付きUnionFindにおいて、2つの頂点の距離を返す関数
        /// 2つの頂点が同じ木に属していない場合は`None`を返す
        pub fn diff(&mut self, x: usize, y: usize) -> Option<isize> {
            if self.same(x, y) {
                Some(self.weights[x] - self.weights[y])
            } else {
                None
            }
        }
        pub fn is_parent(&self, x: usize) -> bool {
            self.parents[x] == x
        }
    }
}

fn main() {
    let out = stdout();
    let mut writer = BufWriter::new(out.lock());
    let mut sc = Scanner::new();

    let n = sc.next_usize();
    let f = (0..n).map(|_| sc.next_usize()).collect::<Vec<_>>();
    let mut uf = UnionFindTree::new(n + 1);
    for i in 0..n {
        uf.unite(i + 1, f[i]);
    }

    let cnt = (1..=n).filter(|&i| uf.is_parent(i)).count();
    writeln!(writer, "{}", ModInt::new(2).pow(cnt as u64) - 1).unwrap();
}

pub mod basic {
    pub const U_INF: u64 = 1 << 60;
    pub const I_INF: i64 = 1 << 60;

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

        pub fn next_int(&mut self) -> i64 {
            self.next()
        }

        pub fn next_uint(&mut self) -> u64 {
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
