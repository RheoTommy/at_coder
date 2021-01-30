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

pub mod lib {
    pub fn compress<T: std::hash::Hash + Clone + std::cmp::Ord + std::cmp::Eq>(
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
}

fn main() {
    let out = stdout();
    let mut writer = BufWriter::new(out.lock());
    let mut sc = Scanner::new();
    let n = sc.next_usize();
    let m = sc.next_usize();
    let mut vertexes = vec![vec![]; n];
    for _ in 0..m {
        let a = sc.next_usize() - 1;
        let b = sc.next_usize() - 1;
        vertexes[a].push(b);
        vertexes[b].push(a);
    }
    let k = sc.next_usize();
    let c = (0..k).map(|_| sc.next_usize() - 1).collect::<Vec<_>>();
    let (zipped, index) = compress(&c);
    let mut distance = vec![vec![U_INF; k]; k];
    for i in 0..k {
        let mut seen = vec![false; n];
        let mut dp = vec![U_INF; n];
        let mut queue = VecDeque::new();
        dp[zipped[i]] = 0;
        queue.push_back((0, zipped[i]));
        seen[zipped[i]] = true;
        while let Some((cost, i)) = queue.pop_front() {
            for &j in &vertexes[i] {
                if seen[j] {
                    continue;
                }
                seen[j] = true;
                dp[j] = cost + 1;
                queue.push_back((cost + 1, j));
            }
        }

        for j in 0..k {
            distance[i][j] = dp[zipped[j]];
        }
    }
    // dbg!(&distance);

    let mut dp = vec![vec![U_INF; k]; 1 << k];
    for &ci in &c {
        dp[1 << index[&ci]][index[&ci]] = 0;
    }
    for bit in 0..1 << k {
        for i in 0..k {
            if bit >> i & 1 == 0 {
                continue;
            }
            for j in 0..k {
                if bit >> j & 1 == 1 {
                    continue;
                }
                dp[bit | 1 << j][j] = dp[bit | 1 << j][j].min(dp[bit][i] + distance[i][j]);
            }
        }
    }

    let mut ans = U_INF;
    for i in 0..k {
        ans = ans.min(dp[(1 << k) - 1][i]);
    }
    if ans == U_INF {
        writeln!(writer, "{}", -1).unwrap();
    } else {
        writeln!(writer, "{}", ans + 1).unwrap();
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
