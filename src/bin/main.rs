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
    let s = sc.next_usize() - 1;
    let t = sc.next_usize() - 1;
    let mut vertexes = vec![vec![]; n];
    for _ in 0..m {
        let x = sc.next_usize() - 1;
        let y = sc.next_usize() - 1;
        let d = sc.next_usize();
        vertexes[x].push((y, d));
        vertexes[y].push((x, d));
    }

    let mut heap = BinaryHeap::new();
    heap.push((U_INF, 0, s));
    let mut dp_from_s = vec![U_INF; n];
    dp_from_s[s] = 0;
    while let Some((_, cost, now)) = heap.pop() {
        if dp_from_s[now] < cost {
            continue;
        }

        for &(next, time) in &vertexes[now] {
            if cost + time < dp_from_s[next] {
                heap.push((U_INF - cost - time, cost + time, next));
                dp_from_s[next] = cost + time;
            }
        }
    }

    let mut heap = BinaryHeap::new();
    heap.push((U_INF, 0, t));
    let mut dp_from_t = vec![U_INF; n];
    dp_from_t[t] = 0;
    while let Some((_, cost, now)) = heap.pop() {
        if dp_from_t[now] < cost {
            continue;
        }

        for &(next, time) in &vertexes[now] {
            if cost + time < dp_from_t[next] {
                heap.push((U_INF - cost - time, cost + time, next));
                dp_from_t[next] = cost + time;
            }
        }
    }

    let mut c = -1;
    for i in 0..n {
        if dp_from_s[i] == dp_from_t[i] {
            c = i as isize + 1;
            break;
        }
    }

    writeln!(writer, "{}", c).unwrap();
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
