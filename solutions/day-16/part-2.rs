use std::{
    collections::{BTreeSet, HashSet},
    i32,
    io::stdin,
};

fn main() {
    const TURN_COST: i32 = 1001;
    let mut input = String::new();
    let mut grid: Vec<Vec<char>> = vec![];
    let (mut sx, mut sy) = (usize::MAX, usize::MAX);
    let (mut ex, mut ey) = (usize::MAX, usize::MAX);
    while stdin().read_line(&mut input).unwrap() != 0 {
        let input_chars: Vec<char> = input.trim().chars().collect();
        if let Some(p) = input_chars.iter().position(|x| *x == 'S') {
            (sx, sy) = (grid.len(), p);
        }
        if let Some(p) = input_chars.iter().position(|x| *x == 'E') {
            (ex, ey) = (grid.len(), p);
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

    // djikstra
    let mut dist = vec![vec![vec![i32::MAX; 4]; m]; n];
    let mut ts: BTreeSet<(i32, usize, usize, usize)> = BTreeSet::new();
    let mut par: Vec<Vec<Vec<HashSet<(usize, usize, usize)>>>> =
        vec![vec![vec![HashSet::new(); 4]; m]; n];

    dist[sx][sy][1] = 0;
    ts.insert((0, sx, sy, 1));

    while !ts.is_empty() {
        let (_, x, y, d) = ts.pop_first().unwrap();
        // clockwise
        let (nx, ny, nd) = match d {
            0 => (x, y + 1, 1),
            1 => (x + 1, y, 2),
            2 => (x, y - 1, 3),
            3 => (x - 1, y, 0),
            _ => panic!("unexpected dir value"),
        };
        if grid[nx][ny] != '#' && dist[x][y][d] + TURN_COST <= dist[nx][ny][nd] {
            par[nx][ny][nd].insert((x, y, d));
            ts.remove(&(dist[nx][ny][nd], nx, ny, nd));
            dist[nx][ny][nd] = dist[x][y][d] + TURN_COST;
            ts.insert((dist[x][y][d] + TURN_COST, nx, ny, nd));
        }

        // anti-clockwise
        let (nx, ny, nd) = match d {
            0 => (x, y - 1, 3),
            1 => (x - 1, y, 0),
            2 => (x, y + 1, 1),
            3 => (x + 1, y, 2),
            _ => panic!("unexpected dir value"),
        };
        if grid[nx][ny] != '#' && dist[x][y][d] + TURN_COST <= dist[nx][ny][nd] {
            par[nx][ny][nd].insert((x, y, d));
            ts.remove(&(dist[nx][ny][nd], nx, ny, nd));
            dist[nx][ny][nd] = dist[x][y][d] + TURN_COST;
            ts.insert((dist[x][y][d] + TURN_COST, nx, ny, nd));
        }

        // forward
        let (nx, ny, nd) = match d {
            0 => (x - 1, y, 0),
            1 => (x, y + 1, 1),
            2 => (x + 1, y, 2),
            3 => (x, y - 1, 3),
            _ => panic!("unexpected dir value"),
        };
        if grid[nx][ny] != '#' && dist[x][y][d] + 1 <= dist[nx][ny][nd] {
            par[nx][ny][nd].insert((x, y, d));
            ts.remove(&(dist[nx][ny][nd], nx, ny, nd));
            dist[nx][ny][nd] = dist[x][y][d] + 1;
            ts.insert((dist[x][y][d] + 1, nx, ny, nd));
        }
    }
    let mut ans = i32::MAX;
    for i in 0..4 {
        if dist[ex][ey][i] < ans {
            ans = dist[ex][ey][i];
        }
    }

    let mut all_vis: Vec<Vec<bool>> = vec![vec![false; m]; n];
    for i in 0..4 {
        if dist[ex][ey][i] == ans {
            let mut vis: Vec<Vec<Vec<bool>>> = vec![vec![vec![false; 4]; m]; n];
            dfs(ex, ey, i, &par, &mut vis);
            for x in 0..n {
                for y in 0..m {
                    all_vis[x][y] = all_vis[x][y]
                        || vis[x][y][0]
                        || vis[x][y][1]
                        || vis[x][y][2]
                        || vis[x][y][3];
                }
            }
        }
    }

    let mut vis_cells = 0;
    for i in 0..n {
        for j in 0..m {
            if all_vis[i][j] {
                vis_cells += 1;
            } else {
            }
        }
    }
    println!("{}", vis_cells);
}

fn dfs(
    x: usize,
    y: usize,
    d: usize,
    par: &Vec<Vec<Vec<HashSet<(usize, usize, usize)>>>>,
    vis: &mut Vec<Vec<Vec<bool>>>,
) {
    if vis[x][y][d] {
        return;
    }
    vis[x][y][d] = true;
    for (px, py, pd) in par[x][y][d].iter() {
        dfs(*px, *py, *pd, par, vis);
    }
}
