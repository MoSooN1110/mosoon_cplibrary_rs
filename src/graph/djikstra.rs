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

use std::cmp::*;
use std::collections::*;
use std::io::stdin;
use std::io::stdout;
use std::io::Write;
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

macro_rules! notnan((
    :literal $(,)?) => (NotNan::new(

).unwrap()));
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

fn djikstra(graph: &Vec<Vec<(usize, usize)>>, start: usize) -> Vec<usize> {
    let mut dist = vec![INF as usize; graph.len()];
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0 as usize, start)));
    dist[start] = 0;
    while let Some(Reverse(x)) = heap.pop() {
        let cost = x.0;
        let v = x.1;
        if cost > dist[v] {
            continue;
        }
        for edge in &graph[v] {
            let nc = cost + edge.1;
            let nv = edge.0;
            if nc < dist[nv] {
                heap.push(Reverse((nc, nv)));
                dist[nv] = nc;
            }
        }
    }
    return dist;
}

#[test]
fn test_djikstra() {
    let mut graph = vec![vec![(0 as usize, 0 as usize); (0) as usize]; (10) as usize];
    for i in 0..9 {
        let (mut a, mut b, mut c) = (i + 1, i + 2, 1);
        a -= 1;
        b -= 1;
        graph[a].push((b, c));
        graph[b].push((a, c));
    }
    let dist = djikstra(&graph, 0);
    assert_eq!(dist[9], 9);
}
