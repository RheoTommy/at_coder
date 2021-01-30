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
    /// セグ木にのせるMonoid
    pub trait Monoid {
        type Item: std::fmt::Debug + Clone;
        /// 単位元
        fn id() -> Self::Item;
        /// 二項演算
        fn op(a: &Self::Item, b: &Self::Item) -> Self::Item;
    }
    pub struct SegTree<M: Monoid> {
        data: Vec<M::Item>,
        n: usize,
    }
    impl<M: Monoid> std::fmt::Debug for SegTree<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let v = &self.data[self.n - 1..];
            write!(f, "{:?}", v)
        }
    }
    impl<M: Monoid> SegTree<M> {
        /// すべて単位元で埋めた長さnのセグ木の生成
        pub fn new(n: usize) -> Self {
            let mut i = 1;
            while i < n {
                i *= 2;
            }
            let data = (0..2 * i - 1).map(|_| M::id()).collect::<Vec<_>>();
            Self { data, n: i }
        }
        /// O(n)でスライスからセグ木を生成
        pub fn from_slice(slice: &[M::Item]) -> Self {
            let mut i = 1;
            while i < slice.len() {
                i *= 2;
            }
            let mut data = vec![M::id(); 2 * i - 1];
            for j in 0..slice.len() {
                data[j + i - 1] = slice[j].clone();
            }
            if slice.len() != 1 {
                for j in (0..=(i - 2)).rev() {
                    data[j] = M::op(&data[j * 2 + 1], &data[j * 2 + 2]);
                }
            }
            Self { data, n: i }
        }
        /// 一点更新
        pub fn set(&mut self, mut i: usize, x: M::Item) {
            i += self.n - 1;
            self.data[i] = x.clone();
            while i > 0 {
                i = (i - 1) / 2;
                self.data[i] = M::op(&self.data[i * 2 + 1], &self.data[i * 2 + 2]);
            }
        }
        /// 一点取得
        pub fn get(&self, mut i: usize) -> &M::Item {
            i += self.n - 1;
            &self.data[i]
        }
        /// 区間クエリ
        /// [l,r)の演算結果を求める
        pub fn fold(&self, mut l: usize, mut r: usize) -> M::Item {
            let mut l_ans = M::id();
            let mut r_ans = M::id();
            l += self.n - 1;
            r += self.n - 1;
            while l < r {
                if l & 1 == 0 {
                    l_ans = M::op(&l_ans, &self.data[l]);
                }
                if r & 1 == 0 {
                    r_ans = M::op(&self.data[r - 1], &r_ans);
                    r -= 2;
                }
                l >>= 1;
                r >>= 1;
            }
            M::op(&l_ans, &r_ans)
        }
    }
    pub struct Max;
    pub struct Min;
    pub struct Sum;
    pub struct Mul;
    pub struct Xor;
    impl Monoid for Max {
        type Item = i128;
        fn id() -> Self::Item {
            std::i128::MIN
        }
        fn op(a: &Self::Item, b: &Self::Item) -> Self::Item {
            *a.max(b)
        }
    }
    impl Monoid for Min {
        type Item = i128;
        fn id() -> Self::Item {
            std::i128::MAX
        }
        fn op(a: &Self::Item, b: &Self::Item) -> Self::Item {
            *a.min(b)
        }
    }
    impl Monoid for Sum {
        type Item = i128;
        fn id() -> Self::Item {
            0
        }
        fn op(a: &Self::Item, b: &Self::Item) -> Self::Item {
            a + b
        }
    }
    impl Monoid for Mul {
        type Item = i128;
        fn id() -> Self::Item {
            1
        }
        fn op(a: &Self::Item, b: &Self::Item) -> Self::Item {
            a * b
        }
    }
    impl Monoid for Xor {
        type Item = i128;
        fn id() -> Self::Item {
            0
        }
        fn op(a: &Self::Item, b: &Self::Item) -> Self::Item {
            a ^ b
        }
    }
    pub type MaxSegTree = SegTree<Max>;
    pub type MinSegTree = SegTree<Min>;
    pub type SumSegTree = SegTree<Sum>;
    pub type MulSegTree = SegTree<Mul>;
    pub type XorSegTree = SegTree<Xor>;
}

fn main() {
    let out = stdout();
    let mut writer = BufWriter::new(out.lock());
    let mut sc = Scanner::new();
    let n = sc.next_usize();
    let a = (0..n).map(|_| sc.next_usize()).collect::<Vec<_>>();
    let mut st = SumSegTree::new(n);
    let mut x = 0;
    for i in 0..n {
        let ai = a[i];
        x += st.fold(ai, n);
        st.set(ai, 1);
    }
    writeln!(writer, "{}", x).unwrap();
    for k in 1..n {
        let ki = a[k - 1];
        x -= ki as i128 ;
        x += (n - ki - 1) as i128;
        writeln!(writer, "{}", x).unwrap();
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
