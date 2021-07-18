// -*- coding:utf-8-unix -*-
// #![feature(map_first_last)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_macros)]
use std::collections::*;
use std::convert::*;
use std::convert::{From, Into};
use std::fmt::Debug;
use std::fs::File;
use std::io::prelude::*;
use std::io::*;
use std::marker::Copy;
use std::mem::*;
use std::ops::Bound::*;
use std::ops::{Add, Mul, Neg, Sub};
use std::str;
use std::vec;
use std::{cmp, process::Output};
use std::{cmp::Ordering, env::consts::DLL_PREFIX};
use std::{cmp::Ordering::*, f32::consts::PI};
const INF: i64 = 1223372036854775807;
const UINF: usize = INF as usize;
const FINF: f64 = 122337203685.0;
const INF128: i128 = 1223372036854775807000000000000;
const LINF: i64 = 2147483647;
const MOD: i64 = 1000000007;
const MPI: f64 = 3.14159265358979323846264338327950288f64;
// const MOD: i64 = 998244353;
// const MOD: i64 = INF;

const UMOD: usize = MOD as usize;
use std::cmp::*;
use std::collections::*;
use std::io::stdin;
use std::io::stdout;
use std::io::Write;

macro_rules! p {
    ($x:expr) => {
        println!("{}", $x);
    };
}
macro_rules! d {
    ($x:expr) => {
        println!("{:?}", $x);
    };
}
macro_rules! dd {
    (x:expr) => {
        dbg!(x);
    };
}

macro_rules! chmin {
    ($base:expr, $($cmps:expr),+ $(,)*) => {{
        let cmp_min = min!($($cmps),+);
        if $base > cmp_min {
            $base = cmp_min;
            true
        } else {
            false
        }
    }};
}

macro_rules! chmax {
    ($base:expr, $($cmps:expr),+ $(,)*) => {{
        let cmp_max = max!($($cmps),+);
        if $base < cmp_max {
            $base = cmp_max;
            true
        } else {
            false
        }
    }};
}

macro_rules! min {
    ($a:expr $(,)*) => {{
        $a
    }};
    ($a:expr, $b:expr $(,)*) => {{
        std::cmp::min($a, $b)
    }};
    ($a:expr, $($rest:expr),+ $(,)*) => {{
        std::cmp::min($a, min!($($rest),+))
    }};
}

macro_rules! max {
    ($a:expr $(,)*) => {{
        $a
    }};
    ($a:expr, $b:expr $(,)*) => {{
        std::cmp::max($a, $b)
    }};
    ($a:expr, $($rest:expr),+ $(,)*) => {{
        std::cmp::max($a, max!($($rest),+))
    }};
}

// use str::Chars;

// use str::Chars;
#[allow(dead_code)]
fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

#[allow(dead_code)]
fn readi() -> (i64) {
    let mut str = String::new();
    let _ = stdin().read_line(&mut str).unwrap();
    let mut iter = str.split_whitespace();
    iter.next().unwrap().parse::<i64>().unwrap()
}

#[allow(dead_code)]
fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    read::<String>()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect()
}
#[allow(dead_code)]
fn read_vec2<T: std::str::FromStr>(n: u32) -> Vec<Vec<T>> {
    (0..n).map(|_| read_vec()).collect()
}

#[allow(dead_code)]
fn readii() -> (i64, i64) {
    let mut str = String::new();
    let _ = stdin().read_line(&mut str).unwrap();
    let mut iter = str.split_whitespace();
    (
        iter.next().unwrap().parse::<i64>().unwrap(),
        iter.next().unwrap().parse::<i64>().unwrap(),
    )
}

fn readff() -> (f64, f64) {
    let mut str = String::new();
    let _ = stdin().read_line(&mut str).unwrap();
    let mut iter = str.split_whitespace();
    (
        iter.next().unwrap().parse::<f64>().unwrap(),
        iter.next().unwrap().parse::<f64>().unwrap(),
    )
}

#[allow(dead_code)]
fn readiii() -> (i64, i64, i64) {
    let mut str = String::new();
    let _ = stdin().read_line(&mut str).unwrap();
    let mut iter = str.split_whitespace();
    (
        iter.next().unwrap().parse::<i64>().unwrap(),
        iter.next().unwrap().parse::<i64>().unwrap(),
        iter.next().unwrap().parse::<i64>().unwrap(),
    )
}
#[allow(dead_code)]
fn readuu() -> (usize, usize) {
    let mut str = String::new();
    let _ = stdin().read_line(&mut str).unwrap();
    let mut iter = str.split_whitespace();
    (
        iter.next().unwrap().parse::<usize>().unwrap(),
        iter.next().unwrap().parse::<usize>().unwrap(),
    )
}

fn readcc() -> (char, char) {
    let mut str = String::new();
    let _ = stdin().read_line(&mut str).unwrap();
    let mut iter = str.split_whitespace();
    (
        iter.next().unwrap().parse::<char>().unwrap(),
        iter.next().unwrap().parse::<char>().unwrap(),
    )
}

#[allow(dead_code)]
fn readuuu() -> (usize, usize, usize) {
    let mut str = String::new();
    let _ = stdin().read_line(&mut str).unwrap();
    let mut iter = str.split_whitespace();
    (
        iter.next().unwrap().parse::<usize>().unwrap(),
        iter.next().unwrap().parse::<usize>().unwrap(),
        iter.next().unwrap().parse::<usize>().unwrap(),
    )
}

#[allow(dead_code)]
fn readuuuu() -> (usize, usize, usize, usize) {
    let mut str = String::new();
    let _ = stdin().read_line(&mut str).unwrap();
    let mut iter = str.split_whitespace();
    (
        iter.next().unwrap().parse::<usize>().unwrap(),
        iter.next().unwrap().parse::<usize>().unwrap(),
        iter.next().unwrap().parse::<usize>().unwrap(),
        iter.next().unwrap().parse::<usize>().unwrap(),
    )
}

fn readiiii() -> (i64, i64, i64, i64) {
    let mut str = String::new();
    let _ = stdin().read_line(&mut str).unwrap();
    let mut iter = str.split_whitespace();
    (
        iter.next().unwrap().parse::<i64>().unwrap(),
        iter.next().unwrap().parse::<i64>().unwrap(),
        iter.next().unwrap().parse::<i64>().unwrap(),
        iter.next().unwrap().parse::<i64>().unwrap(),
    )
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
struct MultiSet {
    set: BTreeSet<usize>,
    map: HashMap<usize, usize>,
}

impl MultiSet {
    fn new() -> MultiSet {
        MultiSet {
            set: BTreeSet::new(),
            map: HashMap::new(),
        }
    }

    ///multiset的に書き出し
    fn print(&self) {
        print!("{{ ");
        for x in &self.set {
            if let Some(&num) = self.map.get(x) {
                for _i in 0..num {
                    print!("{} ", x);
                }
            }
        }
        println!("}}");
    }

    ///重複許可挿入
    fn insert(&mut self, i: usize) -> Option<usize> {
        if let Some(_i) = self.set.get(&i) {
            //setにある
            *self.map.entry(i).or_insert(0) += 1;
        } else {
            //setにない
            self.set.insert(i);
            *self.map.entry(i).or_insert(0) += 1;
        }
        return Some(i);
    }

    ///1つ削除
    fn erase(&mut self, e: usize) -> Option<usize> {
        if let Some(_e) = self.set.get(&e) {
            //setにある
            *self.map.entry(e).or_insert(0) -= 1;
            if self.map[&e] == 0 {
                //なくなった
                self.set.take(&e);
            }
            return Some(e);
        } else {
            //setにない
            return None;
        }
    }

    ///最小値の取得
    fn get_min(&self) -> Option<usize> {
        if let Some(&m) = self.set.iter().nth(0) {
            return Some(m);
        } else {
            return None;
        }
    }

    ///最大値の取得
    fn get_max(&self) -> Option<usize> {
        if let Some(&m) = self.set.iter().last() {
            return Some(m);
        } else {
            return None;
        }
    }
}

#[allow(dead_code)]
pub fn gcd(mut a: i64, mut b: i64) -> i64 {
    if (a < b) {
        let t = a;
        a = b;
        b = t;
    }
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

/// Equivalent to std::lowerbound and std::upperbound in c++
pub trait BinarySearch<T> {
    fn lower_bound(&self, x: &T) -> usize;
    fn upper_bound(&self, x: &T) -> usize;
}
impl<T: Ord> BinarySearch<T> for [T] {
    fn lower_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();
        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less => {
                    low = mid + 1;
                }
                Ordering::Equal | Ordering::Greater => {
                    high = mid;
                }
            }
        }
        low
    }
    fn upper_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();
        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less | Ordering::Equal => {
                    low = mid + 1;
                }
                Ordering::Greater => {
                    high = mid;
                }
            }
        }
        low
    }
}
pub struct Dsu {
    n: usize,
    // root node: -1 * component size
    // otherwise: parent
    parent_or_size: Vec<i32>,
}

impl Dsu {
    // 0 <= size <= 10^8 is constrained.
    pub fn new(size: usize) -> Self {
        Self {
            n: size,
            parent_or_size: vec![-1; size],
        }
    }
    pub fn merge(&mut self, a: usize, b: usize) -> usize {
        assert!(a < self.n);
        assert!(b < self.n);
        let (mut x, mut y) = (self.leader(a), self.leader(b));
        if x == y {
            return x;
        }
        if -self.parent_or_size[x] < -self.parent_or_size[y] {
            std::mem::swap(&mut x, &mut y);
        }
        self.parent_or_size[x] += self.parent_or_size[y];
        self.parent_or_size[y] = x as i32;
        x
    }

    pub fn same(&mut self, a: usize, b: usize) -> bool {
        assert!(a < self.n);
        assert!(b < self.n);
        self.leader(a) == self.leader(b)
    }
    pub fn leader(&mut self, a: usize) -> usize {
        assert!(a < self.n);
        if self.parent_or_size[a] < 0 {
            return a;
        }
        self.parent_or_size[a] = self.leader(self.parent_or_size[a] as usize) as i32;
        self.parent_or_size[a] as usize
    }
    pub fn size(&mut self, a: usize) -> usize {
        assert!(a < self.n);
        let x = self.leader(a);
        -self.parent_or_size[x] as usize
    }
    pub fn groups(&mut self) -> Vec<Vec<usize>> {
        let mut leader_buf = vec![0; self.n];
        let mut group_size = vec![0; self.n];
        for i in 0..self.n {
            leader_buf[i] = self.leader(i);
            group_size[leader_buf[i]] += 1;
        }
        let mut result = vec![Vec::new(); self.n];
        for i in 0..self.n {
            result[i].reserve(group_size[i]);
        }
        for i in 0..self.n {
            result[leader_buf[i]].push(i);
        }
        result
            .into_iter()
            .filter(|x| !x.is_empty())
            .collect::<Vec<Vec<usize>>>()
    }
}

pub struct SCC {
    g: Vec<Vec<usize>>,
    r_g: Vec<Vec<usize>>,
    post_order: VecDeque<usize>,
    used: Vec<bool>,
    pub order: Vec<usize>,
}

impl SCC {
    pub fn new(n: usize) -> Self {
        Self {
            g: vec![vec![]; n],
            r_g: vec![vec![]; n],
            post_order: VecDeque::new(),
            used: vec![false; n],
            order: vec![n; n],
        }
    }
    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.g[u].push(v);
        self.r_g[v].push(u);
    }
    fn dfs(&mut self, u: usize) {
        self.used[u] = true;
        for i in 0..self.g[u].len() {
            let v = self.g[u][i];
            if !self.used[v] {
                self.dfs(v);
            }
        }
        self.post_order.push_front(u);
    }
    fn rdfs(&mut self, u: usize, k: usize) {
        self.used[u] = true;
        self.order[u] = k;
        for i in 0..self.r_g[u].len() {
            let v = self.r_g[u][i];
            if !self.used[v] {
                self.rdfs(v, k);
            }
        }
    }
    pub fn build(&mut self) {
        for v in 0..self.g.len() {
            if !self.used[v] {
                self.dfs(v);
            }
        }
        // dbg!(&self.post_order);
        self.used = vec![false; self.g.len()];
        let mut k = 0;
        for i in 0..self.post_order.len() {
            let v = self.post_order[i];
            if !self.used[v] {
                self.rdfs(v, k);
                k += 1;
            }
        }
    }
}

fn scc_if(graph: &Vec<Vec<(usize, usize)>>) -> (Vec<usize>, Vec<Vec<(usize, usize)>>) {
    let mut res = 0;
    let n = graph.len();
    let mut scc = SCC::new(n);
    let mut es = vec![];
    for i in 0..n {
        for j in 0..graph[i].len() {
            let a = i;
            let b = graph[i][j].0;
            scc.add_edge(a, b);
            es.push((a, b));
        }
    }

    scc.build();
    // d!(scc.order);
    let mut nn = 0;
    for i in 0..scc.order.len() {
        nn = max(scc.order[i], nn);
    }
    nn += 1;
    let mut graph2 = vec![vec![(0 as usize, 0 as usize); (0) as usize]; (nn) as usize];
    let mut st = BTreeSet::new();
    for i in es {
        let a = scc.order[i.0];
        let b = scc.order[i.1];
        if st.contains(&(a, b)) {
            continue;
        }
        st.insert((a, b));
        graph2[a].push((b, 1));
    }

    (scc.order, graph2)
}

// #[test]
// fn test_scc() {
//     let e = vec![
//         vec![1],
//         vec![2, 3],
//         vec![3],
//         vec![4],
//         vec![2, 5],
//         vec![6],
//         vec![7, 8, 9],
//         vec![5],
//         vec![9, 11],
//         vec![10],
//         vec![9],
//         vec![],
//     ];
//     let n = e.len();
//     let mut g = SCC::new(n);
//     for u in 0..n {
//         for v in e[u].clone() {
//             g.add_edge(u, v);
//         }
//     }
//     g.build();

//     assert_eq!(g.order, [0, 1, 2, 2, 2, 3, 3, 3, 4, 6, 6, 5]);
// }

fn solve() {
    // verify https://atcoder.jp/contests/typical90/tasks/typical90_u
    let (n, m) = readuu();
    let mut graph = vec![vec![(0 as usize, 0 as usize); (0) as usize]; (n) as usize];
    for i in 0..m {
        let (mut a, mut b) = readuu();
        a -= 1;
        b -= 1;
        graph[a].push((b, 1));
    }
    let scc = scc_if(&graph);
    let mut res = 0;
    let mut bt = BTreeMap::new();
    for i in 0..n {
        *bt.entry(scc.0[i]).or_insert(0) += 1;
    }
    for i in bt {
        res += (i.1) * (i.1 - 1) / 2;
    }
    println!("{:?}", res);
    return;
}
fn main() {
    solve();
}
