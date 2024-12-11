use std::cmp::max;
use std::cmp::min;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::future::pending;
use std::i32;
use std::io::stdin;

fn nc2(n: i64) -> i64 {
    return n * (n + 1) / 2;
}

fn main() {
    let mut input = String::new();
    let _ = stdin().read_line(&mut input);
    let input = input.trim().to_string();
    let nums: Vec<i64> = input.split(' ').map(|x| x.parse().unwrap()).collect();

    println!("{:?}", nums);
    let mut ans = 0;
    for x in nums {
        ans += go(x, 0);
    }
    println!("{}", ans);
}
fn go(mut n: i64, d: i32)-> i64{
    let num_digs = ((n as f64).log(10.0) as i64) + 1;
    if d == 25 {
        println!("LEAF: {} {}", d, n);
        return 1;
    }
    if n == 0 {
        println!("{} {}", d, n);
        return go(1, d + 1);
    }
    else if num_digs % 2 == 0 {
        println!("{} {}", d, n);
        let mut right = 0;
        let mut p = 1;
        for _ in 0..num_digs/2 {
            right = right + p * (n % 10);
            n /= 10;
            p *= 10;
        }
        return go(n, d + 1) + go(right, d + 1);
    }
    else {
        println!("{} {}", d, n);
        return go(n * 2024, d + 1);
    }
}