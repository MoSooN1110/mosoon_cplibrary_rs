// -*- coding:utf-8-unix -*-

#![allow(dead_code)]
#![allow(unused_imports)]
use std::cmp::Ordering::*;

use std::cmp;
use std::cmp::Ordering;
use std::collections::*;
use std::convert::*;
use std::convert::{From, Into};
use std::fmt::Debug;
use std::fs::File;
use std::io::prelude::*;
use std::io::*;
use std::marker::Copy;
use std::mem::*;
use std::ops::{Add, Mul, Neg, Sub};
use std::str;
use std::vec;
const INF: i64 = 1223372036854775807;
const MEM_SIZE: usize = 202020;
const MOD: i64 = 1000000007;
// const MOD: i64 = 998244353;

fn topological_sort(graph: &Vec<Vec<(usize, usize)>>) -> Vec<usize> {
    let n = graph.len();
    let mut root = 0;
    let mut res = vec![0; 0];
    let mut cnt = vec![0; n];
    for i in 0..n {
        for j in 0..graph[i].len() {
            let x = graph[i][j].0;
            cnt[x] += 1;
        }
    }
    for i in 0..n {
        if cnt[i] == 0 {
            root = i;
        }
    }

    let mut q = VecDeque::new();
    q.push_back(root);
    while !q.is_empty() {
        let v = q.pop_front().unwrap();
        res.push(v);
        for i in 0..graph[v].len() {
            let nv = graph[v][i].0;
            cnt[nv] -= 1;
            if cnt[nv] == 0 {
                q.push_back(nv);
            }
        }
    }

    return res;
}

#[test]
fn test_topological_sort() {
    assert_eq!(1, 0); //未検証
}
