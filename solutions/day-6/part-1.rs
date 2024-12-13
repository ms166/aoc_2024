use std::io::stdin;

fn main() {
    let mut grid: Vec<Vec<char>> = Vec::new();
    let (mut sx, mut sy) = (usize::MAX, usize::MAX);
    for i in 0..130 {
        let mut input = String::new();
        let _ = stdin().read_line(&mut input);
        input = input.trim().to_string();
        grid.push(input.chars().collect());
        for (index, c) in grid[grid.len() - 1].iter().enumerate() {
            if *c == '^' {
                sx = i;
                sy = index;
                break;
            }
        }
    }

    let mut x = sx;
    let mut y = sy;
    let mut dir = 0;
    let mut vis: Vec<Vec<bool>> = vec![vec![false; 130]; 130];
    let n = grid.len();
    let m = grid[0].len();
    assert!(n == 130);
    assert!(m == 130);

    loop {
        vis[x][y] = true;
        let (nx, ny) = get_next(dir as i32, x as i32, y as i32);
        let next_enum = check_next(nx, ny, &grid);

        match next_enum {
            0 => {
                break;
            }
            1 => {
                let mut oob = false;
                for j in 1..=2 {
                    let new_dir = (dir + j) % 4;
                    let (nx, ny) = get_next(new_dir as i32, x as i32, y as i32);
                    let next_enum = check_next(nx, ny, &grid);
                    match next_enum {
                        0 => {
                            oob = true;
                            break;
                        }
                        1 => {
                            continue;
                        }
                        2 => {
                            dir = new_dir;
                            x = nx as usize;
                            y = ny as usize;
                            break;
                        }
                        _ => {
                            panic!("unexpected value");
                        }
                    }
                }
                if oob {
                    break;
                }
            }
            2 => {
                x = nx as usize;
                y = ny as usize;
            }
            _ => panic!("unexpected value"),
        }
    }
    let mut ans = 0;
    for x in vis {
        for y in x {
            if y {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}

fn check_next(nx: i32, ny: i32, grid: &Vec<Vec<char>>) -> i32 {
    let n = grid.len();
    let m = grid[0].len();

    // out of bounds
    if nx < 0 || nx >= n as i32 || ny < 0 || ny >= m as i32 {
        return 0;
    }

    // blocked
    let nx = nx as usize;
    let ny = ny as usize;
    if grid[nx][ny] == '#' {
        return 1;
    }
    return 2;
}

fn get_next(dir: i32, x: i32, y: i32) -> (i32, i32) {
    match dir {
        0 => (x - 1, y),
        1 => (x, y + 1),
        2 => (x + 1, y),
        3 => (x, y - 1),
        _ => panic!("unexpected dir value"),
    }
}
