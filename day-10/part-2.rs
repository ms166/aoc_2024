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
    let mut grid: Vec<Vec<char>> = vec![];
    for _ in 0..47 {
        let mut input = String::new();
        let _ = stdin().read_line(&mut input);
        let input = input.trim().to_string();
        grid.push(input.chars().collect());
    }
    let n = grid.len();
    let m = grid[0].len();
    let mut ans = 0;
    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == '0'{
                let mut vis: Vec<Vec<bool>> = vec![vec![false; m]; n];
                ans += dfs(i, j, &grid, &mut vis);
            }
        }
    }
    println!("{}", ans);
}

const dx: [i32; 4] = [-1, 0, 0, 1];
const dy: [i32; 4] = [0, -1, 1, 0];

fn dfs(x: usize, y: usize, grid: &Vec<Vec<char>>, vis: &mut Vec<Vec<bool>>) -> i32{
    let n = grid.len();
    let m = grid[0].len();
    let mut cnt = if grid[x][y] == '9' { 1 } else { 0 };
    vis[x][y] = true;
    for i in 0..4 {
        let nx = x as i32 + dx[i];
        let ny = y as i32 + dy[i];
        if nx >= 0 && nx < n as i32 && ny >= 0 && ny < m as i32 
            && grid[nx as usize][ny as usize] as i32 == grid[x][y] as i32 + 1 
            && !vis[nx as usize][ny as usize] {
            cnt += dfs(nx as usize, ny as usize, grid, vis);
        }
    }
    // vis[x][y] = false;
    return cnt;
}
