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

use itertools::{assert_equal, Itertools};

use crate::basic::*;
use crate::lib::*;

pub mod lib {
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

    pub fn pow<T: std::ops::Mul<Output = T> + MulDivIdent + Copy>(x: T, n: u64) -> T {
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
        n: u64,
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

    pub fn ext_gcd(a: i64, b: i64) -> (i64, i64, i64) {
        if b == 0 {
            if a < 0 {
                return (-a, -1, 0);
            } else {
                return (a, 1, 0);
            }
        }
        let (g, s, t) = ext_gcd(b, a % b);
        return (g, t, s - (a / b) * t);
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
    pub fn divisors(n: u64) -> Vec<u64> {
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
    pub fn is_prime(n: u64) -> bool {
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
    pub fn primes(mut n: u64) -> std::collections::BTreeMap<u64, u64> {
        let mut res = std::collections::BTreeMap::new();
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

    pub fn float_to_int(s: &[char], x: u32) -> i64 {
        if !s.contains(&'.') {
            return s.iter().collect::<String>().parse::<i64>().unwrap() * 10i64.pow(x);
        }
        let n = s.len();
        let i = s
            .iter()
            .enumerate()
            .filter(|(_, ci)| **ci == '.')
            .next()
            .unwrap()
            .0;
        let l = n - i - 1;
        let t = s
            .iter()
            .skip_while(|ci| **ci == '0')
            .filter(|ci| **ci != '.')
            .collect::<String>()
            .parse::<i64>()
            .unwrap()
            * 10i64.pow(x - l as u32);
        t
    }
}

fn main() {
    let out = stdout();
    let mut writer = BufWriter::new(out.lock());
    let mut sc = Scanner::new();
    let t = sc.next_usize();

    for _ in 0..t {
        let x = sc.next_int();
        let y = sc.next_int();
        let p = sc.next_int();
        let q = sc.next_int();

        let mut ans = I_INF;

        let a = 2 * (x + y);
        let b = p + q;

        for t1 in x..x + y {
            for t2 in p..p + q {
                let x = calc(b, ((t2 - t1) % a + a) % a, a);
                ans = (ans).min(x);
            }
        }

        if ans >= I_INF {
            writeln!(writer, "infinity").unwrap();
        } else {
            writeln!(writer, "{}", ans).unwrap();
        }
    }
}

fn mod_inv(a: i64, m: i64) -> i64 {
    let (g, x, y) = ext_gcd(a, m);
    let aa = m / g;
    (x % aa + aa) % aa
}

fn calc(mut a: i64, mut b: i64, mut m: i64) -> i64 {
    if b == 0 {
        return 0;
    }

    let g = gcd(a, gcd(b, m));
    a /= g;
    b /= g;
    m /= g;
    if gcd(a, m) != 1 {
        I_INF
    } else {
        (mod_inv(a, m) * b) % m
    }
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
