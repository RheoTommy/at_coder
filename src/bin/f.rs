#![allow(unused_macros)]
#![allow(dead_code)]
#![allow(unused_imports)]

use std::io::{stdout, BufWriter, Write};
use std::{collections::*, vec};

use itertools::Itertools;
use num::PrimInt;

use crate::lib::Scanner;

const U_INF: usize = 1 << 60;
const I_INF: isize = 1 << 60;

fn main() {
    let out = stdout();
    let mut writer = BufWriter::new(out.lock());
    let mut sc = Scanner::new();
    let n = sc.next_usize();
    let m = sc.next_usize();
    let mut edges = HashSet::new();
    for _ in 0..m {
        let a = sc.next_usize() - 1;
        let b = sc.next_usize() - 1;
        edges.insert((a, b));
        edges.insert((b, a));
    }

    let mut dp = vec![false; 1 << n];
    for i in 0..n {
        dp[1 << i] = true;
    }

    for state in 0..1 << n {
        if !dp[state] {
            continue;
        }

        'outer: for i in 0..n {
            if (state >> i) & 1 == 1 {
                continue;
            }

            for j in 0..n {
                if (state >> j) & 1 == 1 && !edges.contains(&(i, j)) {
                    continue 'outer;
                }
            }
            dp[state | 1 << i] = true;
        }
    }

    let mut memo = vec![U_INF; 1 << n];
    memo[0] = 0;

    for state in 0..1 << n {
        if dp[state] {
            memo[state] = 1;
        }
    }

    let mut dp = memo;

    for state in 0..1 << n {
        let mut t = state;
        while t > 0 {
            dp[state] = dp[state].min(dp[state ^ t] + dp[t]);
            t = (t - 1) & state;
        }
    }

    writeln!(writer, "{}", dp[(1 << n) - 1]).unwrap();
}

fn rec(state: usize, n: usize, memo: &mut HashMap<usize, usize>) -> usize {
    if memo.contains_key(&state) {
        return memo[&state];
    }

    0
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
