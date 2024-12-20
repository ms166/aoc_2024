use std::{collections::VecDeque, i32, io::stdin};

fn main() {
    let mut input = String::new();
    let mut grid: Vec<Vec<char>> = vec![];
    let (mut sx, mut sy, mut ex, mut ey) = (usize::MAX, usize::MAX, usize::MAX, usize::MAX);
    while stdin().read_line(&mut input).unwrap() != 0 {
        let input_chars: Vec<char> = input.trim().to_string().chars().collect();
        if let Some(p) = input_chars.iter().position(|x| *x == 'S') {
            sx = grid.len();
            sy = p;
        }
        if let Some(p) = input_chars.iter().position(|x| *x == 'E') {
            ex = grid.len();
            ey = p;
        }
        grid.push(input_chars);
        input.clear();
    }
    assert!(sx != usize::MAX);
    assert!(sy != usize::MAX);
    assert!(ex != usize::MAX);
    assert!(ey != usize::MAX);

    let n = grid.len();
    let m = grid[0].len();
    let original_time = bfs(&grid, sx, sy, ex, ey, sx, sy);
    let mut ans = 0;
    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == '#' {
                grid[i][j] = '.';
                let t = bfs(&grid, sx, sy, ex, ey, i, j);
                if original_time - t >= 100 {
                    ans += 1;
                }
                grid[i][j] = '#';
            }
        }
    }
    println!("{}", ans);
}

const DX: [i32; 4] = [-1, 0, 0, 1];
const DY: [i32; 4] = [0, -1, 1, 0];

fn bfs(
    grid: &Vec<Vec<char>>,
    sx: usize,
    sy: usize,
    ex: usize,
    ey: usize,
    kx: usize,
    ky: usize,
) -> i32 {
    let mut q = VecDeque::new();
    let n = grid.len();
    let m = grid[0].len();
    let mut dist: Vec<Vec<i32>> = vec![vec![i32::MAX; m]; n];
    let mut par: Vec<Vec<(usize, usize)>> = vec![vec![(usize::MAX, usize::MAX); m]; n];
    q.push_back((sx, sy));
    dist[sx][sy] = 0;
    while !q.is_empty() {
        let (x, y) = q.pop_front().unwrap();
        for i in 0..4 {
            let nx = x as i32 + DX[i];
            let ny = y as i32 + DY[i];
            if nx >= 0
                && nx < n as i32
                && ny >= 0
                && ny < m as i32
                && grid[nx as usize][ny as usize] != '#'
                && dist[nx as usize][ny as usize] == i32::MAX
            {
                par[nx as usize][ny as usize] = (x, y);
                dist[nx as usize][ny as usize] = dist[x][y] + 1;
                q.push_back((nx as usize, ny as usize))
            }
        }
    }
    let mut cx = ex;
    let mut cy = ey;
    let mut found = false;
    loop {
        let (px, py) = par[cx][cy];
        cx = px;
        cy = py;
        if cx == kx && cy == ky {
            found = true;
            break;
        }
        if cx == sx && cy == sy {
            break;
        }
    }
    if found {
        return dist[ex][ey];
    }
    return i32::MAX;
}
