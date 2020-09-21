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
// const MOD: i64 = 998244353;

// pub mod vector_lcm;

fn prime_factorization(x: usize) -> BTreeMap<usize, usize> {
    let mut res: BTreeMap<usize, usize> = BTreeMap::new();
    let mut xx = x;
    let mut p: usize = 2;
    while p * p <= xx {
        while xx % p == 0 {
            // println!("{:?}", p);
            let t = res.get_mut(&p);
            if t.is_none() {
                res.insert(p, 1);
            } else {
                *t.unwrap() += 1;
            }
            xx /= p;
        }
        println!("{:?} {:?}", p, res);
        p += 1;
    }

    if xx != 1 {
        let t = res.get_mut(&xx);
        if t.is_none() {
            res.insert(xx, 1);
        } else {
            *t.unwrap() += 1;
        }
    }
    res
}

pub fn vector_lcm(vec: &Vec<usize>) -> usize {
    let mut res = 1;
    let mut alcm: BTreeMap<usize, usize> = BTreeMap::new();
    for i in 0..vec.len() {
        let pf = prime_factorization(vec[i]);
        // println!("{:?}", vec[i]);
        // println!("{:?}", pf);
        for i in pf {
            let t = alcm.get_mut(&i.0);
            if t.is_none() {
                alcm.insert(i.0, i.1);
            } else {
                let val = *t.unwrap();
                *alcm.get_mut(&i.0).unwrap() = max(val, i.1);
            }
        }
    }
    for i in alcm {
        for j in 0..i.1 {
            res *= i.0;
        }
        // println!("{:?}", i);
    }

    res
}

// #[cfg(test)]
// mod test_vetor_lcm {
#[test]
fn test_vector_lcm() {
    let mut vec = vec![0; 3];
    vec[0] = 3;
    vec[1] = 4;
    vec[2] = 5;
    // println!("{:?}", vec);
    // println!("{:?}", vector_lcm(&vec));
    assert_eq!(vector_lcm(&vec), 60);
}
// }
