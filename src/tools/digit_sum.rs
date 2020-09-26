// -*- coding:utf-8-unix -*-

#![allow(dead_code)]
#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::fs::File;
use std::io::prelude::*;
use std::io::*;
use std::mem;
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

fn digit_sum(mut x: i64) -> i64 {
    let mut res = 0;
    while x > 0 {
        res += x % 10;
        x /= 10;
    }
    res
}

#[test]
fn test_digitsum() {
    assert_eq!(digit_sum(12345), 15);
}
