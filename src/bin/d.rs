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

    let k = sc.next_usize();
    let s = sc.next_chars();
    let t = sc.next_chars();

    let mut cards = vec![k; 9];
    for &si in &s {
        if si == '#' {
            break;
        }
        cards[si.to_digit(10).unwrap() as usize - 1] -= 1;
    }
    for &si in &t {
        if si == '#' {
            break;
        }
        cards[si.to_digit(10).unwrap() as usize - 1] -= 1;
    }

    let mut win = 0;

    let mut tak = BTreeMap::new();
    let mut aok = BTreeMap::new();
    for &si in &s {
        if si == '#' {
            break;
        }
        *tak.entry(si.to_digit(10).unwrap() as usize).or_insert(0) += 1;
    }
    for &ti in &t {
        if ti == '#' {
            break;
        }
        *aok.entry(ti.to_digit(10).unwrap() as usize).or_insert(0) += 1;
    }
    for i in 1..=9{
        tak.entry(i).or_insert(0);
        aok.entry(i).or_insert(0);
    }
    // writeln!(writer, "{:?}", score(&tak));
    // writeln!(writer, "{:?}", score(&aok));

    let mut ans = 0.0;

    for i in 0..9 {
        for j in 0..9 {
            let pos = cards[i]
                * if i == j {
                    cards[i].saturating_sub(1)
                } else {
                    cards[j]
                };
            *tak.entry(i + 1).or_insert(0) += 1;
            *aok.entry(j + 1).or_insert(0) += 1;
            let ta = score(&tak);
            let ao = score(&aok);
            *tak.entry(i + 1).or_insert(0) -= 1;
            *aok.entry(j + 1).or_insert(0) -= 1;
            if ta > ao {
                win += pos;
            }
        }
    }
    let sum = 9 * k - 8;
    // writeln!(writer, "{}", ans).unwrap();
    writeln!(writer, "{}", win as f64 / (sum * (sum - 1)) as f64).unwrap();
}

fn score(m: &BTreeMap<usize, usize>) -> usize {
    let mut sum = 0;
    for (k, v) in m {
        sum += *k * 10usize.pow(*v as u32);
    }
    sum
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
