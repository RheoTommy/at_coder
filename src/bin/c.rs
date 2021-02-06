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

use crate::basic::*;
use crate::lib::*;

pub mod lib {}

fn main() {
    let out = stdout();
    let mut writer = BufWriter::new(out.lock());
    let mut sc = Scanner::new();
    let t = sc.next_uint();
    'test: for _ in 0..t {
        let n = sc.next_usize();
        let m = sc.next_usize();
        let a = (0..n).map(|_| sc.next_usize() - 1).collect::<Vec<_>>();
        let b = (0..n).map(|_| sc.next_usize() - 1).collect::<Vec<_>>();
        let c = (0..m).map(|_| sc.next_usize() - 1).collect::<Vec<_>>();

        let mut last = U_INF as usize;
        for i in 0..n {
            if b[i] == c[m - 1] {
                last = i;
                break;
            }
        }

        if last == U_INF as usize {
            writeln!(writer, "{}", "NO").unwrap();
            continue 'test;
        }

        let mut need = vec![VecDeque::new(); n];
        for i in 0..n {
            if a[i] != b[i] {
                need[b[i]].push_back(i);
                if b[i] == c[m - 1] {
                    last = i;
                }
            }
        }

        let mut have = vec![0; n];
        for i in 0..m {
            have[c[i]] += 1;
        }

        for i in 0..n {
            if need[i].len() > have[i] {
                writeln!(writer, "{}", "NO").unwrap();
                continue 'test;
            }
        }

        writeln!(writer, "{}", "YES").unwrap();
        for &ci in &c {
            write!(
                writer,
                "{} ",
                match need[ci].pop_front() {
                    None => last + 1,
                    Some(i) => i + 1,
                }
            )
            .unwrap();
        }
        writeln!(writer).unwrap();
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
