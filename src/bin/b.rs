#![allow(unused_macros)]
#![allow(dead_code)]
#![allow(unused_imports)]

use std::collections::*;
use std::io::{stdout, BufWriter, Write};

use itertools::Itertools;

use crate::lib::Scanner;

const U_INF: usize = 1 << 60;
const I_INF: isize = 1 << 60;

fn main() {
    let out = stdout();
    let mut writer = BufWriter::new(out.lock());
    let mut sc = Scanner::new();
    let n = sc.next_usize();
    let a = (0..n).map(|_| sc.next_isize()).collect::<Vec<_>>();
    let b = (0..n).map(|_| sc.next_isize()).collect::<Vec<_>>();
    let sum = a.into_iter().zip(b).map(|(ai, bi)| ai * bi).sum::<isize>();
    let ans = if sum == 0 { "Yes" } else { "No" };
    writeln!(writer, "{}", ans).unwrap();
}

pub mod lib {
    pub struct Scanner {
        buf: std::collections::VecDeque<String>,
    }

    impl Scanner {
        pub fn new() -> Self {
            Self {
                buf: std::collections::VecDeque::new(),
            }
        }

        fn scan_line(&mut self) {
            let mut flag = 0;
            while self.buf.is_empty() {
                let mut s = String::new();
                std::io::stdin().read_line(&mut s).unwrap();
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

        pub fn next_isize(&mut self) -> isize {
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
