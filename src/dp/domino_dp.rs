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
// const MOD: i64 = 998244353;
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

macro_rules! pp {
    ($x:expr) => {
        println!("{:?}", $x);
    };
}

macro_rules! d {
    ($x:expr) => {
        dbg!($x);
    };
}

// use str::Chars;
#[allow(dead_code)]
fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

// #[allow(dead_code)]
// fn readi() -> (i64) {
//     let mut str = String::new();
//     let _ = stdin().read_line(&mut str).unwrap();
//     let mut iter = str.split_whitespace();
//     iter.next().unwrap().parse::<i64>().unwrap()
// }

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

fn dfs(h: usize, w: usize, b: usize, mut mat: Vec<Vec<usize>>, colors: &mut Vec<Vec<Vec<usize>>>) {
    let mut m = mat.clone();
    let mut m2 = mat.clone();
    let mut hi = mat.len();
    // m[h][w] = 0;
    let mut wi = mat[0].len();
    m[h][w] = 0;

    if w == wi - 1 && h == hi - 1 {
        let mut cnt = 0 as usize;
        for i in 0..hi {
            for j in 0..wi {
                if m[i][j] == 1 {
                    cnt += 1;
                }
            }
        }
        if cnt == b {
            colors.push(m);
        }
    } else if w == wi - 1 {
        dfs(h + 1, 0, b, m.clone(), colors);
    } else {
        dfs(h, w + 1, b, m.clone(), colors);
    }

    m2[h][w] = 1;
    if w == wi - 1 && h == hi - 1 {
        let mut cnt = 0 as usize;
        for i in 0..hi {
            for j in 0..wi {
                if m2[i][j] == 1 {
                    cnt += 1;
                }
            }
        }
        if cnt == b {
            colors.push(m2);
        }
    } else if w == wi - 1 {
        dfs(h + 1, 0, b, m2, colors);
    } else {
        dfs(h, w + 1, b, m2, colors);
    }
}

fn solve() {
    let (h, w, a, b) = readuuuu();
    // let mut dp = vec![vec![vec![vec![0 as usize; h + 1]; w + 1]; (1 << w)]; a + 1];
    // dp[0][0][0][0] = 1;
    // for i in 0..h {
    //     for j in 0..w {
    //         for s in 0..((1 as usize) << w) {
    //             if s & (1 << j) > 0 {

    //             }
    //         }
    //     }
    // }
    // if b == h * w {
    //     println!("{:?}", 1);
    //     return;
    // }

    // if b == h * w - 2 {
    //     println!("{:?}", (h - 1) * w + (w - 1) * h);
    //     return;
    // }

    let mut res = 0 as usize;
    // let mut colors = vec![vec![0 as usize;(w)as usize];(h)as usize];
    let mut colors = vec![vec![vec![0 as usize; w]; h]; 0];
    let mut vv = vec![vec![0 as usize; (w) as usize]; (h) as usize];
    dfs(0, 0, b, vv, &mut colors);
    // d!(colors.clone());
    // d!(colors.len());
    let mut res = 0 as usize;

    let mut dp = vec![vec![vec![vec![0 as usize; 1 << w]; w + 1]; h + 1]; b + 2];
    dp[0][0][0][0] = 1;
    for ii in 0..1 {
        let mut color = colors[ii].clone();
        // let mut color = ;
        let mut color = vec![vec![0 as usize; (w) as usize]; (h) as usize];
        for i in 0..h {
            for j in 0..w {
                for bb in 0..b + 1 {
                    for used in 0..(1 << w) {
                        if ((used & (1 << j)) > 0) || color[i][j] > 0 {
                            let mut next = used & (!(1 << j));
                            if j + 1 < w {
                                // pp!((i, j, next, used));
                                dp[bb][i][j + 1][next] += dp[bb][i][j][used];
                            } else {
                                dp[bb][i + 1][0][next] += dp[bb][i][j][used];
                            }
                        } else {
                            if (j + 1 < w) && (color[i][j + 1] == 0) && !(used & (1 << (j + 1)) > 0)
                            {
                                let mut next = used | (1 << (j + 1));
                                dp[bb][i][j + 1][next] += dp[bb][i][j][used];
                            }
                            if i + 1 < h && color[i + 1][j] == 0 {
                                let mut next = used | (1 << (j));
                                if j + 1 < w {
                                    dp[bb][i][j + 1][next] += dp[bb][i][j][used];
                                } else {
                                    dp[bb][i + 1][0][next] += dp[bb][i][j][used];
                                }
                            }

                            if true {
                                let mut next = used;
                                if j + 1 < w {
                                    dp[bb + 1][i][j + 1][next] += dp[bb][i][j][used];
                                } else {
                                    dp[bb + 1][i + 1][0][next] += dp[bb][i][j][used];
                                }
                            }
                        }
                    }
                }
            }
        }
        // println!("{:?}", dp);
        // println!("{:?}", color);
        res += dp[b][h][0][0];
        // for i in 0..h + 1 {
        //     for j in 0..w + 1 {
        //         for k in 0..1 << w {
        //             dp[i][j][k] = 0;
        //         }
        //     }
        // }
    }
    println!("{:?}", res);
    return;
}

fn main() {
    solve()
}
