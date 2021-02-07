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
use num::integer::Roots;

pub mod lib {}

fn main() {
    let out = stdout();
    let mut writer = BufWriter::new(out.lock());
    let mut sc = Scanner::new();
    let n = sc.next_usize();
    let a = (0..n)
        .map(|_| to_int(&sc.next_chars()))
        .map(|i| (f(i), g(i)))
        .collect::<Vec<_>>();

    let mut dp = vec![vec![0i128; 50]; 50];
    for &(ai, bi) in &a {
        dp[ai as usize][bi as usize] += 1;
    }

    let mut cnt = 0;
    for i in 0..50 {
        for j in 0..50 {
            for k in 0..50 {
                for l in 0..50 {
                    if i + k < 18 || j + l < 18 {
                        continue;
                    }
                    if (i, j) == (k, l) {
                        continue;
                    }
                    cnt += dp[i][j] * dp[k][l];
                }
            }
        }
    }

    cnt /= 2;

    let mut cnt2 = 0;
    for i in 0..50 {
        for j in 0..50 {
            if i + i < 18 || j + j < 18 || dp[i][j] == 0 {
                continue;
            }

            cnt2 += dp[i][j] * (dp[i][j] - 1) / 2;
        }
    }

    writeln!(writer, "{}", cnt + cnt2).unwrap();
}



fn f(x: i128) -> i128 {
    if x % 5 != 0 {
        0
    } else {
        1 + f(x / 5)
    }
}
fn g(x: i128) -> i128 {
    if x % 2 != 0 {
        0
    } else {
        1 + g(x / 2)
    }
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
