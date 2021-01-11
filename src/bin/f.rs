#![allow(unused_macros)]
#![allow(dead_code)]
#![allow(unused_imports)]

use std::io::{stdout, BufWriter, Write};
use std::{collections::*, fmt::format};

use itertools::Itertools;

use crate::lib::Scanner;

const U_INF: usize = 1 << 60;
const I_INF: isize = 1 << 60;

fn main() {
    let out = stdout();
    let mut writer = BufWriter::new(out.lock());
    let mut sc = Scanner::new();
    let x = sc.next_usize() as u128;
    let y = sc.next_usize() as u128;

    if x >= y {
        writeln!(writer, "{}", x - y).unwrap();
        writer.flush().unwrap();
        std::process::exit(0);
    }

    let mut ans = U_INF;

    // 2倍する回数全探索
    for z in 0..=100 {
        let made = x * 2u128.pow(z);
        let diff = if made > y { made - y } else { y - made };

        let k = z.min(format!("{:b}", diff).len() as u32);
        eprintln!("{:?}", k);
        let mut l = U_INF;

        for bit in 0..3u128.pow(k) {
            let mut acc = 0;
            let mut count = 0;
            for i in 0..k {
                let b = (bit / 3u128.pow(i)) % 3;
                if b == 1 {
                    acc += 2i128.pow(i);
                    count += 1;
                } else if b == 2 {
                    acc -= 2i128.pow(i);
                    count += 1;
                }
            }
            if diff == acc as u128 {
                l = l.min(count);
            }
        }

        ans = ans.min(z as usize + l);
    }

    writeln!(writer, "{}", ans).unwrap();

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
