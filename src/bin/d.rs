#![allow(unused_macros)]
#![allow(dead_code)]
#![allow(unused_imports)]

use std::collections::*;
use std::io::{stdout, BufWriter, Write};

use itertools::Itertools;
use lib::coord_comp;

use crate::lib::Scanner;

const U_INF: usize = 1 << 60;
const I_INF: isize = 1 << 60;

fn main() {
    let out = stdout();
    let mut writer = BufWriter::new(out.lock());
    let mut sc = Scanner::new();
    let n = sc.next_usize();
    let c = sc.next_usize();
    let mut coord = Vec::new();
    let mut services = Vec::new();
    for _ in 0..n {
        let a = sc.next_usize();
        let b = sc.next_usize() + 1;
        let c = sc.next_usize();
        coord.push(a);
        coord.push(b);
        services.push((a, b, c));
    }
    let (zipped, index) = coord_comp(&coord);
    let mut imos = vec![0isize; zipped.len() + 10];
    for &(a, b, c) in &services {
        imos[index[&a]] += c as isize;
        imos[index[&b]] -= c as isize;
    }
    for i in 1..imos.len() {
        imos[i] += imos[i - 1];
    }

    // eprintln!("{:?}", imos);

    let mut ans = 0;
    for i in 0..zipped.len() - 1 {
        if imos[i] > c as isize {
            ans += (zipped[i + 1] - zipped[i]) * c;
        } else {
            ans += imos[i] as usize * (zipped[i + 1] - zipped[i]);
        }
    }
    writeln!(writer, "{}", ans).unwrap();
}

pub mod lib {
    pub fn coord_comp<
        T: std::hash::Hash + Clone + std::cmp::Ord + std::cmp::PartialEq + std::cmp::Eq,
    >(
        v: &[T],
    ) -> (Vec<T>, std::collections::HashMap<T, usize>) {
        let mut v = v.iter().collect::<Vec<_>>();
        v.sort();
        v.dedup();
        let mut res = std::collections::HashMap::new();
        let mut zip = Vec::with_capacity(v.len());
        for (i, vi) in v.into_iter().enumerate() {
            zip.push(vi.clone());
            res.insert(vi.clone(), i);
        }
        (zip, res)
    }

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
