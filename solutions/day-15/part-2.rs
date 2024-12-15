use core::panic;
use std::io::stdin;

fn main() {
    let mut input = String::new();
    let mut grid: Vec<Vec<char>> = vec![];
    let mut inst: Vec<char> = vec![];
    let (mut sx, mut sy): (usize, usize) = (usize::MAX, usize::MAX);
    while stdin().read_line(&mut input).unwrap() != 0 {
        if input.len() == 1 {
            continue;
        }
        let input_chars: Vec<char> = input.trim().chars().collect();

        if input_chars[0] == '<'
            || input_chars[0] == '>'
            || input_chars[0] == '^'
            || input_chars[0] == 'v'
        {
            inst.extend(input_chars);
        } else {
            let mut new_grid_line: Vec<char> = vec![];
            for x in input_chars {
                if x == '#' {
                    new_grid_line.push('#');
                    new_grid_line.push('#');
                } else if x == 'O' {
                    new_grid_line.push('[');
                    new_grid_line.push(']');
                } else if x == '.' {
                    new_grid_line.push('.');
                    new_grid_line.push('.');
                } else {
                    new_grid_line.push('@');
                    new_grid_line.push('.');
                }
            }
            grid.push(new_grid_line.clone());
            let p = new_grid_line.iter().position(|x| *x == '@');
            if let Some(p) = p {
                assert!(sx == usize::MAX);
                assert!(sy == usize::MAX);
                (sx, sy) = ((grid.len() - 1), p);
            }
        }
        input.clear();
    }
    assert!(sx != usize::MAX);
    assert!(sy != usize::MAX);
    let n = grid.len();
    let m = grid[0].len();
    for x in inst {
        match x {
            '>' => {
                for j in sy + 1..m {
                    if grid[sx][j] == '#' {
                        break;
                    }
                    if grid[sx][j] == '.' {
                        grid[sx][sy] = '.';
                        grid[sx][sy + 1] = '@';
                        let mut c = 0;
                        for k in sy + 2..=j {
                            grid[sx][k] = if c % 2 == 0 { '[' } else { ']' };
                            c += 1;
                        }
                        sy += 1;
                        break;
                    }
                }
            }
            '<' => {
                for j in (0..=sy - 1).rev() {
                    if grid[sx][j] == '#' {
                        break;
                    }
                    if grid[sx][j] == '.' {
                        grid[sx][sy] = '.';
                        grid[sx][sy - 1] = '@';
                        let mut c = 0;
                        for k in j..=sy - 2 {
                            grid[sx][k] = if c % 2 == 0 { '[' } else { ']' };
                            c += 1;
                        }
                        sy -= 1;
                        break;
                    }
                }
            }
            '^' => {
                if process_vert(sx, sy, 0, &mut grid) {
                    sx -= 1;
                }
            }
            'v' => {
                if process_vert(sx, sy, 1, &mut grid) {
                    sx += 1;
                }
            }
            _ => panic!("unexpected value of instruction char"),
        }
    }
    let mut ans = 0;
    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == '[' {
                ans += 100 * i + j;
            }
        }
    }
    println!("{}", ans);
}

fn process_vert(sx: usize, sy: usize, dir: i32, grid: &mut Vec<Vec<char>>) -> bool {
    let nx = if dir == 0 { sx - 1 } else { sx + 1 };
    if grid[nx][sy] == '#' {
        return false;
    } else if grid[nx][sy] == '.' {
        grid[sx][sy] = '.';
        grid[nx][sy] = '@';
        return true;
    } else if grid[nx][sy] == '[' {
        if check_vert(nx, sy, dir, &grid) {
            move_vert(nx, sy, dir, grid);
            grid[nx][sy] = '@';
            grid[sx][sy] = '.';
            return true;
        }
    } else {
        if grid[nx][sy] == ']' {
            if check_vert(nx, sy - 1, dir, &grid) {
                move_vert(nx, sy - 1, dir, grid);
                grid[nx][sy] = '@';
                grid[sx][sy] = '.';
                return true;
            }
        } else if grid[nx][sy + 1] == '[' {
            if check_vert(nx, sy + 1, dir, &grid) {
                move_vert(nx, sy + 1, dir, grid);
                grid[nx][sy] = '@';
                grid[sx][sy] = '.';
                return true;
            }
        } else {
            panic!("unexpected value");
        }
    }
    return false;
}

fn check_vert(lx: usize, ly: usize, dir: i32, grid: &Vec<Vec<char>>) -> bool {
    let mut ok = true;
    let nx = if dir == 0 { lx - 1 } else { lx + 1 };
    if grid[nx][ly] == '#' || grid[nx][ly + 1] == '#' {
        ok = false;
    } else {
        if grid[nx][ly] == '.' && grid[nx][ly + 1] == '.' {
            ok = true;
        } else {
            if grid[nx][ly] == '[' {
                ok = ok && check_vert(nx, ly, dir, grid);
            } else {
                if grid[nx][ly] == ']' {
                    ok = ok && check_vert(nx, ly - 1, dir, grid);
                }
                if grid[nx][ly + 1] == '[' {
                    ok = ok && check_vert(nx, ly + 1, dir, grid);
                }
            }
        }
    }
    ok
}

fn move_vert(lx: usize, ly: usize, dir: i32, grid: &mut Vec<Vec<char>>) {
    let nx = if dir == 0 { lx - 1 } else { lx + 1 };
    if grid[nx][ly] == '#' || grid[nx][ly + 1] == '#' {
        panic!("should not have encountered #");
    } else {
        if grid[nx][ly] == '.' && grid[nx][ly + 1] == '.' {
            grid[nx][ly] = '[';
            grid[nx][ly + 1] = ']';
            grid[lx][ly] = '.';
            grid[lx][ly + 1] = '.';
        } else {
            if grid[nx][ly] == '[' {
                move_vert(nx, ly, dir, grid);
                grid[nx][ly] = '[';
                grid[nx][ly + 1] = ']';
                grid[lx][ly] = '.';
                grid[lx][ly + 1] = '.';
            } else {
                if grid[nx][ly] == ']' && grid[nx][ly + 1] == '[' {
                    move_vert(nx, ly - 1, dir, grid);
                    move_vert(nx, ly + 1, dir, grid);
                    grid[nx][ly] = '[';
                    grid[nx][ly + 1] = ']';
                    grid[lx][ly] = '.';
                    grid[lx][ly + 1] = '.';
                } else if grid[nx][ly] == ']' {
                    move_vert(nx, ly - 1, dir, grid);
                    grid[nx][ly] = '[';
                    grid[nx][ly + 1] = ']';
                    grid[lx][ly] = '.';
                    grid[lx][ly + 1] = '.';
                } else if grid[nx][ly + 1] == '[' {
                    move_vert(nx, ly + 1, dir, grid);
                    grid[nx][ly] = '[';
                    grid[nx][ly + 1] = ']';
                    grid[lx][ly] = '.';
                    grid[lx][ly + 1] = '.';
                } else {
                    panic!("unexpected value {} {}", grid[nx][ly], grid[nx][ly + 1]);
                }
            }
        }
    }
}
