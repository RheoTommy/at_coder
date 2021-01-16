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
    pub fn pow<T: std::ops::Mul<Output = T> + From<isize> + Copy>(x: T, n: u128) -> T {
        if n == 0 {
            1isize.into()
        } else if n == 1 {
            x
        } else if n % 2 == 1 {
            x * pow(x, n - 1)
        } else {
            pow(x * x, n / 2)
        }
    }
    pub fn mod_pow<
        T: std::ops::Mul<Output = T> + std::ops::Rem<Output = T> + From<isize> + Copy,
    >(
        x: T,
        n: u128,
        m: T,
    ) -> T {
        if n == 0 {
            1isize.into()
        } else if n == 1 {
            x % m
        } else if n % 2 == 1 {
            x * mod_pow(x, n - 1, m) % m
        } else {
            mod_pow(x * x % m, n / 2, m)
        }
    }
    pub fn gcd<T: From<usize> + std::ops::Rem<Output = T> + Copy + std::cmp::Ord>(a: T, b: T) -> T {
        if a < b {
            gcd(b, a)
        } else if b == 0.into() {
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
        T: From<usize>
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
    let n = sc.next_usize() as u128;
    let m = sc.next_i128() as isize;
    let b = mod_pow(10, n as u128, m * m) / m;
    writeln!(writer, "{}", b).unwrap();
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

        pub fn next_i128(&mut self) -> i128 {
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
