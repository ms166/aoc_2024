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
            grid.push(input_chars.clone());
            let p = input_chars.iter().position(|x| *x == '@');
            if let Some(p) = p {
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
                        for k in sy + 2..=j {
                            grid[sx][k] = 'O';
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
                        for k in j..=sy - 2 {
                            grid[sx][k] = 'O';
                        }
                        sy -= 1;
                        break;
                    }
                }
            }
            '^' => {
                for j in (0..=sx - 1).rev() {
                    if grid[j][sy] == '#' {
                        break;
                    }
                    if grid[j][sy] == '.' {
                        grid[sx][sy] = '.';
                        grid[sx - 1][sy] = '@';
                        for k in j..=sx - 2 {
                            grid[k][sy] = 'O';
                        }
                        sx -= 1;
                        break;
                    }
                }
            }
            'v' => {
                for j in sx + 1..n {
                    if grid[j][sy] == '#' {
                        break;
                    }
                    if grid[j][sy] == '.' {
                        grid[sx][sy] = '.';
                        grid[sx + 1][sy] = '@';
                        for k in sx + 2..=j {
                            grid[k][sy] = 'O';
                        }
                        sx += 1;
                        break;
                    }
                }
            }
            _ => panic!("unexpected value of instruction char"),
        }
    }
    let mut ans = 0;
    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == 'O' {
                ans += 100 * i + j;
            }
        }
    }
    println!("{}", ans);
}
