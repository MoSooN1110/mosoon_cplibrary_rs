#![allow(dead_code)]
#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::fs::File;
use std::io::prelude::*;
use std::io::*;
use std::vec;

const INF: i64 = 1223372036854775807;
const MEM_SIZE: usize = 202020;
const MOD: i64 = 1000000007;
/// Equivalent to std::lowerbound and std::upperbound in c++

pub struct Doubling {
    n: usize,
    log: usize,
    table: Vec<Vec<i64>>,
}

impl Doubling {
    fn new(n: usize, max_loop: usize) -> Doubling {
        Doubling {
            n,
            log: ((max_loop as f64).log2().floor()) as usize + 1,
            table: vec![vec![-1; n]; (max_loop as f64).log2().floor() as usize + 2],
        }
    }
    fn set_next(&mut self, i: usize, x: usize) {
        self.table[0][i] = x as i64;
        return;
    }
    fn build(&mut self) {
        for k in 0..self.log {
            for i in 0..self.table[k].len() {
                if self.table[k][i] == -1 {
                    self.table[k + 1][i] = -1;
                } else {
                    self.table[k + 1][i] = self.table[k][self.table[k][i] as usize];
                }
            }
        }
        return;
    }

    fn query(&mut self, mut k: usize, t: usize) -> usize {
        for i in (0..self.log).rev() {
            if (t >> i) & 1 == 1 {
                k = self.table[i][k] as usize;
            }
        }
        return k;
    }
}

#[test]
fn test_doubling() {
    //https://atcoder.jp/contests/abc013/tasks/abc013_4 passed
    let mut n = 16 as usize;
    let mut d = Doubling::new(n, 30);
    for i in 0..n {
        d.set_next(i, (i * 2) % n);
    }
    d.build();
    assert_eq!(d.query(3, 3), 8);
}
