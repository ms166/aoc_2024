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

fn main() {
    let mut grid: Vec<Vec<char>> = Vec::new();
    for _ in 0..50 {
        let mut input = String::new();
        let _ = stdin().read_line(&mut input);
        input = input.trim().to_string();
        grid.push(input.chars().collect());
    }
    let mut pos: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let n = grid.len();
    let m = grid[0].len();
    for i in 0..n {
        for j in 0..m {
            if grid[i][j] != '.' {
                pos
                    .entry(grid[i][j])
                    .and_modify(|f| {
                        f.push((i as i32, j as i32));
                    })
                    .or_insert(vec![(i as i32, j as i32)]);
            }
        }
    }
    let mut hs: HashSet<(i32, i32)> = HashSet::new();
    for (c,v) in pos.iter() {
        for i in 0..v.len()-1 {
            for j in i+1..v.len() {
                let dx = v[i].0 - v[j].0;
                let dy = v[i].1 - v[j].1;
                let p1 = (v[i].0 + dx, v[i].1 + dy);
                let p2 = (v[j].0 - dx, v[j].1 - dy);

                assert!(dx <= 0);

                let mut cur_pair = format!("{} ", c);
                if p1.0 >= 0 && p1.0 < n as i32 && p1.1 >= 0 && p1.1 < m as i32 {
                    hs.insert(p1);
                    cur_pair.push_str(&format!("({},{})", p1.0, p1.1).to_string());
                }
                if p2.0 >= 0 && p2.0 < n as i32 && p2.1 >= 0 && p2.1 < m as i32{
                    hs.insert(p2);
                    cur_pair.push_str(&format!("--({},{})", p2.0, p2.1).to_string());
                }
                println!("{}", cur_pair);
            }
        }
    }
    println!("{}", hs.len());
}
