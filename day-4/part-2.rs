use std::cmp::min;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::i32;
use std::io::stdin;

fn main() {
    let num_lines = 140;
    let mut grid: Vec<Vec<char>> = vec![];
    for _ in 0..num_lines {
        let mut input = String::new();
        let _ = stdin().read_line(&mut input);
        input = input.trim().to_string();
        grid.push(input.chars().collect());
    }
    let n = grid.len();
    let m = grid[0].len();
    let mut ans = 0;
    for i in 0..n-2 {
        for j in 0..m-2 {
            let mut check_count = 0;
            if check(&grid, i, j, 0) {
                check_count += 1;
            }
            if check(&grid, i, j + 2, 1) {
                check_count += 1;
            }
            if check(&grid, i+2, j + 2, 2) {
                check_count += 1;
            }
            if check(&grid, i+2, j, 3) {
                check_count += 1;
            }
            assert!(check_count <= 2);
            if check_count == 2 {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}

fn check(grid: &Vec<Vec<char>>, i: usize, j: usize, dir: i32) -> bool {
    let n = grid.len();
    let m = grid[0].len();
    return match dir {
        0 => i + 2 < n && j + 2 < m && grid[i][j] == 'M' && grid[i+1][j+1] == 'A' && grid[i+2][j+2] == 'S',
        1 => i + 2 < n && j as i32 - 2 >= 0 && grid[i][j] == 'M' && grid[i+1][j-1] == 'A' && grid[i+2][j-2] == 'S',
        2 => i as i32 - 2 >= 0 && j as i32 - 2 >= 0 && grid[i][j] == 'M' && grid[i-1][j-1] == 'A' && grid[i-2][j-2] == 'S',
        3 => i as i32 - 2 >= 0 && j + 2 < m && grid[i][j] == 'M' && grid[i-1][j+1] == 'A' && grid[i-2][j+2] == 'S',
        _ => panic!("Unexpected dir value"),
    }
}