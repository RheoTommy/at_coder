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
    let mut vertexes = vec![vec![]; n];
    let mut edges = vec![];
    for _ in 0..n - 1 {
        let a = sc.next_usize() - 1;
        let b = sc.next_usize() - 1;
        edges.push((a, b));
        vertexes[a].push(b);
        vertexes[b].push(a);
    }

    let mut seen = vec![false; n];
    let mut depth = vec![0; n];
    let mut children = vec![vec![]; n];
    let mut queue = VecDeque::new();
    queue.push_back(0);
    seen[0] = true;
    while let Some(p) = queue.pop_front() {
        for &c in &vertexes[p] {
            if !seen[c] {
                children[p].push(c);
                depth[c] = depth[p] + 1;
                seen[c] = true;
                queue.push_back(c);
            }
        }
    }

    let mut dp = vec![0; n];
    let q = sc.next_usize();
    for _ in 0..q {
        let t = sc.next_usize();
        let e = sc.next_usize() - 1;
        let x = sc.next_isize();

        let (a, b) = if t == 1 {
            edges[e]
        } else {
            let (i, j) = edges[e];
            (j, i)
        };

        if depth[a] < depth[b] {
            dp[0] += x;
            dp[b] -= x;
        } else {
            dp[a] += x;
        }
    }

    let mut queue = VecDeque::new();
    queue.push_back(0);
    while let Some(p) = queue.pop_front() {
        for &c in &children[p] {
            dp[c] += dp[p];
            queue.push_back(c);
        }
    }

    for i in 0..n {
        writeln!(writer, "{}", dp[i]).unwrap();
    }
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
