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

pub struct SegmentSet {
    len: usize,
    // root node: -1 * component size
    // otherwise: parent
    data: BTreeSet<(i64, i64)>,
}

// / Equivalent to std::lowerbound and std::upperbound in c++

use std::ops::Bound::*;
impl SegmentSet {
    // 0 <= size <= 10^8 is constrained.

    pub fn new() -> Self {
        Self {
            len: 0,
            data: BTreeSet::new(),
        }
    }

    pub fn init(&mut self) {
        // let mut st = BTreeSet::new();
        self.data.insert((-1 * INF, -1 * INF));
        self.data.insert((INF, INF));
    }

    fn binary_search(&mut self, val: (i64, i64)) -> ((i64, i64), (i64, i64)) {
        let mut before = self.data.range((Unbounded, Included(val)));
        let mut after = self.data.range((Included(val), Unbounded));

        (*before.next_back().unwrap(), *after.next().unwrap())
    }

    fn binary_search_after_iter(
        &self,
        val: &(i64, i64),
    ) -> std::collections::btree_set::Range<(i64, i64)> {
        let mut before = self.data.range((Unbounded, Included(val)));
        let mut after = self.data.range((Included(val), Unbounded));

        (after)
    }
    fn binary_search_nth(&self, val: &(i64, i64), nth: usize) -> (i64, i64) {
        let mut before = self.data.range((Unbounded, Included(val)));
        let mut after = self.data.range((Included(val), Unbounded));

        let mut res = *after.nth(nth).unwrap_or(&(INF, INF));

        res
    }

    fn range_remove(&mut self, val: (i64, i64)) {
        self.data.remove(&val);
    }

    pub fn add(&mut self, l: i64, r: i64) {
        //[l,r)
        let mut val = (l, r);

        let (before, after) = self.binary_search(val);

        if before.0 <= val.0 && val.0 <= before.1 {
            val.0 = min(val.0, before.0);
            val.1 = max(val.1, before.1);
            {
                self.range_remove(before);
            }
        }

        for x in 0..INF as usize {
            let xx = self.binary_search_nth(&val, x);
            // println!("ckpt {:?}", (val, xx));
            if val.1 >= xx.0 {
                val.1 = max(val.1, xx.1);
                {
                    println!("{:?}", xx);
                    // self.range_remove(xx);
                    self.data.remove(&xx);
                }
            } else {
                break;
            }
        }
        // println!("{:?}", val);
        self.data.insert(val);
    }

    pub fn out(&mut self) -> Vec<(i64, i64)> {
        let mut vp = vec![(0, 0); 0];
        for i in self.data.iter() {
            vp.push(*i);
        }
        vp.pop();
        vp.reverse();
        vp.pop();
        vp.reverse();
        return vp;
    }
    pub fn len(&mut self) -> usize {
        let res = self.data.len();
        return res - 2;
    }
}

#[test]
fn test_segment_set() {
    let mut x = SegmentSet::new();
    x.init();
    x.add(1, 2);
    x.add(2, 3);
    x.add(3, 4);
    assert_eq!(x.out()[0], (1, 4));
}
