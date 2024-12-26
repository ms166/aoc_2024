use std::{cmp::min, collections::HashMap, i32, io::stdin, u64};

fn main() {
    let mut input = String::new();
    let mut ans = 0;
    while stdin().read_line(&mut input).unwrap() != 0 {
        input = input.trim().to_string();
        ans += process(&input);
        input.clear();
    }
    println!("{}", ans);
}

fn process(input: &str) -> u64 {
    let g1: Vec<Vec<char>> = vec![
        vec!['7', '8', '9'],
        vec!['4', '5', '6'],
        vec!['1', '2', '3'],
        vec!['-', '0', 'A'],
    ];
    let g2: Vec<Vec<char>> = vec![vec!['-', '^', 'A'], vec!['<', 'v', '>']];

    let g1_paths = generate_all_paths(&g1);
    let g2_paths = generate_all_paths(&g2);

    let mut len = 0;
    let mut prev_char = 'A';
    let mut dp = HashMap::new();
    for x in input.chars() {
        let paths = g1_paths.get(&(prev_char, x)).unwrap();
        let mut best_min = u64::MAX;
        for path in paths {
            best_min = min(best_min, rec(path, 0, &g2_paths, &mut dp));
        }
        len += best_min;
        prev_char = x;
    }

    let numeric_value = input[..input.len() - 1].parse::<usize>().unwrap() as u64;
    return len * numeric_value;
}

fn rec(
    seq: &Vec<char>,
    depth: u64,
    g2_paths: &HashMap<(char, char), Vec<Vec<char>>>,
    dp: &mut HashMap<(Vec<char>, u64), u64>,
) -> u64 {
    if depth == 25 {
        return seq.len() as u64;
    }
    if dp.contains_key(&(seq.clone(), depth)) {
        return *dp.get(&(seq.clone(), depth)).unwrap();
    }
    let paths = get_all_possible_paths_for_seq(seq, g2_paths);
    let mut ans = u64::MAX;
    for path in paths {
        let split_paths_by_a: Vec<Vec<char>> = split_and_keep_delimiter(&path);
        let mut cur_path_total = 0;
        for split_path in split_paths_by_a {
            cur_path_total += rec(&split_path, depth + 1, g2_paths, dp);
        }
        ans = min(ans, cur_path_total);
    }
    dp.insert((seq.clone(), depth), ans);
    return ans;
}

fn get_all_possible_paths_for_seq(
    seq: &Vec<char>,
    g2_paths: &HashMap<(char, char), Vec<Vec<char>>>,
) -> Vec<Vec<char>> {
    let mut res: Vec<Vec<char>> = vec![vec![]];
    let mut prev_char = 'A';
    for x in seq {
        let paths = g2_paths.get(&(prev_char, *x)).unwrap();
        let mut new_res = vec![];
        for p1 in paths.into_iter() {
            for j in 0..res.len() {
                new_res.push([res[j].clone(), p1.to_vec()].concat());
            }
        }
        res = new_res;
        prev_char = *x;
    }
    return res;
}

fn generate_all_paths(grid: &Vec<Vec<char>>) -> HashMap<(char, char), Vec<Vec<char>>> {
    let n = grid.len();
    let m = grid[0].len();
    let mut shortest_path_map = HashMap::new();
    for i in 0..n {
        for j in 0..m {
            for u in 0..n {
                for v in 0..m {
                    if grid[i][j] == '-' || grid[u][v] == '-' {
                        continue;
                    }
                    let mut all_paths: Vec<Vec<char>> = vec![];
                    let mut path: Vec<char> = vec![];
                    let mut vis: Vec<Vec<bool>> = vec![vec![false; m]; n];
                    dfs(i, j, u, v, grid, &mut vis, &mut path, &mut all_paths);
                    all_paths.sort_by(|a, b| a.len().cmp(&b.len()));
                    let mut shortest_paths = vec![];
                    for p in 0..all_paths.len() {
                        if all_paths[p].len() != all_paths[0].len() {
                            break;
                        }
                        shortest_paths.push(all_paths[p].clone());
                    }
                    shortest_path_map.insert((grid[i][j], grid[u][v]), shortest_paths);
                }
            }
        }
    }
    return shortest_path_map;
}

fn dfs(
    x: usize,
    y: usize,
    tx: usize,
    ty: usize,
    grid: &Vec<Vec<char>>,
    vis: &mut Vec<Vec<bool>>,
    path: &mut Vec<char>,
    all_paths: &mut Vec<Vec<char>>,
) {
    const DX: [i32; 4] = [-1, 0, 0, 1];
    const DY: [i32; 4] = [0, -1, 1, 0];
    let n = grid.len();
    let m = grid[0].len();
    if x == tx && y == ty {
        let mut res = path.clone();
        res.push('A');
        all_paths.push(res);
        return;
    }
    vis[x][y] = true;
    for i in 0..4 {
        let nx = x as i32 + DX[i];
        let ny = y as i32 + DY[i];
        if nx >= 0
            && nx < n as i32
            && ny >= 0
            && ny < m as i32
            && grid[nx as usize][ny as usize] != '-'
            && !vis[nx as usize][ny as usize]
        {
            let dir = match i {
                0 => '^',
                1 => '<',
                2 => '>',
                3 => 'v',
                _ => panic!("unexpected value"),
            };
            path.push(dir);
            dfs(nx as usize, ny as usize, tx, ty, grid, vis, path, all_paths);
            path.pop();
        }
    }
    vis[x][y] = false;
}

fn split_and_keep_delimiter(v: &Vec<char>) -> Vec<Vec<char>> {
    let mut res = vec![];
    let mut cur = vec![];
    for c in v {
        cur.push(*c);
        if *c == 'A' {
            res.push(cur.clone());
            cur.clear();
        }
    }
    return res;
}
