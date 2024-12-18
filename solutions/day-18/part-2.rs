use std::{collections::VecDeque, io::stdin};

const DX: [i32; 4] = [-1, 0, 0, 1];
const DY: [i32; 4] = [0, -1, 1, 0];
fn main() {
    let n = 71;
    let m = 71;
    let mut input = String::new();
    let mut grid: Vec<Vec<char>> = vec![vec!['.'; m]; n];
    while stdin().read_line(&mut input).unwrap() != 0 {
        let x_y = input
            .trim()
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        assert!(x_y.len() == 2);
        grid[x_y[1]][x_y[0]] = '#';
        if !is_end_reachable(&grid) {
            println!("{} {}", x_y[0], x_y[1]);
            return;
        }
        input.clear();
    }
}

fn is_end_reachable(grid: &Vec<Vec<char>>) -> bool {
    let n = grid.len();
    let m = grid[0].len();
    let mut q: VecDeque<(i32, i32)> = VecDeque::new();
    let mut dist: Vec<Vec<i32>> = vec![vec![i32::MAX; m]; n];
    dist[0][0] = 0;
    q.push_back((0, 0));
    while !q.is_empty() {
        let (x, y) = q.pop_front().unwrap();
        for i in 0..4 {
            let nx = x + DX[i];
            let ny = y + DY[i];
            if nx >= 0
                && nx < n as i32
                && ny >= 0
                && ny < m as i32
                && grid[nx as usize][ny as usize] == '.'
                && dist[nx as usize][ny as usize] == i32::MAX
            {
                dist[nx as usize][ny as usize] = dist[x as usize][y as usize] + 1;
                q.push_back((nx, ny));
            }
        }
    }
    dist[n - 1][m - 1] != i32::MAX
}
