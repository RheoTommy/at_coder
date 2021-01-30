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

pub mod lib {}

fn main() {
    let out = stdout();
    let mut writer = BufWriter::new(out.lock());
    let mut sc = Scanner::new();
    let n = sc.next_int() * 2;

    let mut cnt = 0;
    for i in 1.. {
        if i * i > n {
            break;
        }
        if n % i != 0 {
            continue;
        }
        let j = n / i;

        {
            let l = (i + j - 1) / 2;
            let k = i - l;
            if (k + l) * (l - k + 1) == n {
                cnt += 1;
            }
        }
        {
            let i = -i;
            let j = -j;
            let l = (i + j - 1) / 2;
            let k = i - l;
            if (k + l) * (l - k + 1) == n {
                cnt += 1;
            }
        }
    }

    writeln!(writer, "{}", cnt).unwrap();
}

pub mod basic {
    pub const U_INF: u128 = 1 << 60;
    pub const I_INF: i128 = 1 << 60;

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
