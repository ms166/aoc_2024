use std::{cmp::min, collections::BTreeSet, i32, io::stdin};

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
        if grid[nx][ny] != '#' && dist[x][y][d] + TURN_COST < dist[nx][ny][nd] {
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
        if grid[nx][ny] != '#' && dist[x][y][d] + TURN_COST < dist[nx][ny][nd] {
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
        if grid[nx][ny] != '#' && dist[x][y][d] + 1 < dist[nx][ny][nd] {
            ts.remove(&(dist[nx][ny][nd], nx, ny, nd));
            dist[nx][ny][nd] = dist[x][y][d] + 1;
            ts.insert((dist[x][y][d] + 1, nx, ny, nd));
        }
    }
    let mut ans = i32::MAX;
    for i in 0..4 {
        ans = min(ans, dist[ex][ey][i]);
    }
    println!("{}", ans);
}
