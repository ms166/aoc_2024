use std::collections::HashMap;
use std::io::stdin;

fn main() {
    let mut grid: Vec<Vec<char>> = vec![];
    let mut input = String::new();
    while stdin().read_line(&mut input).unwrap() != 0 {
        input = input.trim().to_string();
        assert!(!input.is_empty());
        grid.push(input.chars().collect());
        input.clear();
    }
    let n = grid.len();
    let m = grid[0].len();
    let mut vis: Vec<Vec<bool>> = vec![vec![false; m]; n];
    let mut ans = 0i64;
    // side position and type
    let mut pos: Vec<(i32, i32, i32)> = vec![];
    for i in 0..n {
        for j in 0..m {
            if !vis[i][j] {
                pos.clear();
                let area = go(i as i32, j as i32, &mut vis, &grid, &mut pos);
                let num_sides = cal_sides(&pos);
                ans += area as i64 * num_sides as i64;
                // println!("{:?}", pos);
                println!("{} {} {}", grid[i][j], area, num_sides);
            }
        }
    }
    println!("{}", ans);
}

fn cal_sides(pos: &Vec<(i32, i32, i32)>) -> i32 {
    let mut hm: HashMap<(i32, i32, i32), i32> = HashMap::new();
    for p in pos {
        hm.entry(*p).and_modify(|f| *f += 1).or_insert(1);
    }
    let mut res = 0;
    for p in pos {
        let (cx, cy, t) = {
            if hm.contains_key(&p) {
                *p
            } else {
                (i32::MIN, i32::MIN, i32::MIN)
            }
        };
        if cx == i32::MIN && cy == i32::MIN && t == i32::MIN {
            continue;
        }

        let mut x = cx;
        let mut y = cy;
        loop {
            let (nx, ny) = if t == 0 || t == 2 {
                (x, y + 1)
            } else {
                (x + 1, y)
            };
            if hm.contains_key(&(nx, ny, t)) {
                let entry = hm.get_mut(&(nx, ny, t)).unwrap();
                *entry = *entry - 1;
                if *entry == 0 {
                    hm.remove_entry(&(nx, ny, t));
                }
            } else {
                break;
            }
            x = nx;
            y = ny;
        }
        let mut x = cx;
        let mut y = cy;
        loop {
            let (nx, ny) = if t == 0 || t == 2 {
                (x, y - 1)
            } else {
                (x - 1, y)
            };
            if hm.contains_key(&(nx, ny, t)) {
                let entry = hm.get_mut(&(nx, ny, t)).unwrap();
                *entry = *entry - 1;
                if *entry == 0 {
                    hm.remove_entry(&(nx, ny, t));
                }
            } else {
                break;
            }
            x = nx;
            y = ny;
        }
        res += 1;
        let entry = hm.get_mut(&(cx, cy, t)).unwrap();
        *entry = *entry - 1;
        if *entry == 0 {
            hm.remove_entry(&(cx, cy, t));
        }
    }
    assert!(hm.is_empty());
    return res;
}

const DX: [i32; 4] = [-1, 0, 1, 0];
const DY: [i32; 4] = [0, 1, 0, -1];

fn go(
    x: i32,
    y: i32,
    vis: &mut Vec<Vec<bool>>,
    grid: &Vec<Vec<char>>,
    pos: &mut Vec<(i32, i32, i32)>,
) -> i32 {
    let n = grid.len();
    let m = grid[0].len();
    vis[x as usize][y as usize] = true;
    let mut area = 1;
    for i in 0..4 {
        let nx = x + DX[i];
        let ny = y + DY[i];
        if nx >= 0
            && nx < n as i32
            && ny >= 0
            && ny < m as i32
            && !vis[nx as usize][ny as usize]
            && grid[x as usize][y as usize] == grid[nx as usize][ny as usize]
        {
            let n_area = go(nx, ny, vis, grid, pos);
            area += n_area;
        }
        if !(nx >= 0 && nx < n as i32 && ny >= 0 && ny < m as i32)
            || grid[x as usize][y as usize] != grid[nx as usize][ny as usize]
        {
            let t = match i {
                0 => 0,
                1 => 1,
                2 => 2,
                3 => 3,
                _ => panic!("unexpected value of i"),
            };
            pos.push((nx, ny, t));
        }
    }
    return area;
}
