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
    use cargo_snippet::snippet;
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
    pub struct Num(i128);
    macro_rules! impl_num {
        ($ t : ty ) => {
            impl From<Num> for $t {
                fn from(t: Num) -> $t {
                    t.0 as $t
                }
            }
        };
    }
    impl_num!(i8);
    impl_num!(i16);
    impl_num!(i32);
    impl_num!(i64);
    impl_num!(i128);
    impl_num!(isize);
    impl_num!(u8);
    impl_num!(u16);
    impl_num!(u32);
    impl_num!(u64);
    impl_num!(u128);
    impl_num!(usize);
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
    pub fn pow<T: std::ops::Mul<Output = T> + MulDivIdent + Copy>(x: T, n: u128) -> T {
        if n == 0 {
            T::mul_div_ident()
        } else if n == 1 {
            x
        } else if n % 2 == 1 {
            x * pow(x, n - 1)
        } else {
            pow(x * x, n / 2)
        }
    }
    pub fn mod_pow<
        T: std::ops::Mul<Output = T> + std::ops::Rem<Output = T> + MulDivIdent + Copy,
    >(
        x: T,
        n: u128,
        m: T,
    ) -> T {
        if n == 0 {
            T::mul_div_ident() % m
        } else if n == 1 {
            x % m
        } else if n % 2 == 1 {
            x * mod_pow(x, n - 1, m) % m
        } else {
            mod_pow(x * x % m, n / 2, m)
        }
    }
    pub fn gcd<T: GCDIdent + std::ops::Rem<Output = T> + Copy + std::cmp::Ord>(a: T, b: T) -> T {
        if b == T::gcd_ident() {
            a
        } else {
            gcd(b, a % b)
        }
    }
    pub fn ext_gcd(a: i128, b: i128) -> (i128, i128, i128) {
        if a < b {
            let (g, y, x) = ext_gcd(b, a);
            return (g, x, y);
        }
        if b == 0 {
            return (a, 1, 0);
        }
        let (g, x, y) = ext_gcd(b, a % b);
        (g, y, x - a / b * y)
    }
    pub fn lcm<
        T: GCDIdent
            + std::ops::Mul<Output = T>
            + std::ops::Div<Output = T>
            + std::ops::Rem<Output = T>
            + Copy
            + std::cmp::Ord,
    >(
        a: T,
        b: T,
    ) -> T {
        let g = gcd(a, b);
        a * b / g
    }
    /// O(√N)
    pub fn divisors(n: u128) -> Vec<u128> {
        let mut res = Vec::new();
        for i in 1.. {
            if i * i > n {
                break;
            }
            if n % i == 0 {
                res.push(i);
                if n / i != i {
                    res.push(n / i);
                }
            }
        }
        res
    }
    /// O(√N)
    pub fn is_prime(n: u128) -> bool {
        for i in 2.. {
            if i * i > n {
                break;
            }
            if n % i == 0 {
                return false;
            }
        }
        true
    }
    /// O(√N)
    pub fn primes(mut n: u128) -> std::collections::HashMap<u128, u128> {
        let mut res = std::collections::HashMap::new();
        for i in 2.. {
            if i * i > n {
                break;
            }
            let mut ex = 0;
            while n % i == 0 {
                ex += 1;
                n /= i;
            }
            if ex != 0 {
                res.insert(i, ex);
            }
        }
        if n != 0 {
            *res.entry(n).or_insert(0) += 1;
        }
        res
    }
}

fn main() {
    let out = stdout();
    let mut writer = BufWriter::new(out.lock());
    let mut sc = Scanner::new();
    let a = 50i128;
    let b = 100;
    writeln!(writer, "{}", gcd(a, b)).unwrap();
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
