#![allow(unused_macros)]
#![allow(dead_code)]
#![allow(unused_imports)]

use std::io::{stdout, BufWriter, Write};
use std::{collections::*, vec};

use itertools::Itertools;

use crate::lib::Scanner;

const U_INF: usize = 1 << 60;
const I_INF: isize = 1 << 60;

fn main() {
    let out = stdout();
    let mut writer = BufWriter::new(out.lock());
    let mut sc = Scanner::new();
    let n = sc.next_usize();
    let m = sc.next_usize();
    let a = (0..n).map(|_| sc.next_isize()).collect::<Vec<_>>();
    let mut vertexes = vec![vec![]; n];
    for _ in 0..m {
        let x = sc.next_usize() - 1;
        let y = sc.next_usize() - 1;
        vertexes[x].push(y);
    }

    let mut dp = vec![I_INF; n];
    for i in 0..n {
        for &c in &vertexes[i] {
            dp[c] = dp[c].min(dp[i].min(a[i]));
        }
    }

    let mut ans = -I_INF;
    for i in 1..n {
        ans = ans.max(a[i] - dp[i]);
    }

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
