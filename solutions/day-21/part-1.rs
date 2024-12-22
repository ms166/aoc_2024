use std::{cmp::min, i32, io::stdin};

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

fn process(input: &str) -> i32 {
    let g1: Vec<Vec<char>> = vec![
        vec!['7', '8', '9'],
        vec!['4', '5', '6'],
        vec!['1', '2', '3'],
        vec!['-', '0', 'A'],
    ];
    let g2: Vec<Vec<char>> = vec![vec!['-', '^', 'A'], vec!['<', 'v', '>']];
    let (mut cx, mut cy) = (3, 2);
    let mut paths_1: Vec<Vec<Vec<char>>> = vec![];
    for x in input.chars() {
        paths_1.push(find_paths(&g1, cx, cy, x));
        update_cur(&g1, &mut cx, &mut cy, x);
    }
    drop(g1);
    let c1 = cartesian_product(&paths_1);

    let mut mn = i32::MAX;
    for comb in c1 {
        (cx, cy) = (0, 2);
        let mut paths_2: Vec<Vec<Vec<char>>> = vec![];
        for x in comb {
            paths_2.push(find_paths(&g2, cx, cy, x));
            update_cur(&g2, &mut cx, &mut cy, x);
        }
        let c2 = cartesian_product(&paths_2);
        for comb_2 in c2 {
            (cx, cy) = (0, 2);
            let mut paths_3: Vec<Vec<Vec<char>>> = vec![];
            for y in comb_2 {
                paths_3.push(find_paths(&g2, cx, cy, y));
                update_cur(&g2, &mut cx, &mut cy, y);
            }
            let c3 = cartesian_product(&paths_3);
            for kk in c3 {
                mn = min(mn, kk.len() as i32);
            }
        }
    }
    let numeric_part = *&input[..input.len() - 1].parse::<i32>().unwrap();
    return mn * numeric_part;
}

fn update_cur(grid: &Vec<Vec<char>>, cx: &mut usize, cy: &mut usize, target_char: char) {
    let mut found = false;
    for u in 0..grid.len() {
        for v in 0..grid[0].len() {
            if grid[u][v] == target_char {
                *cx = u;
                *cy = v;
                found = true;
                break;
            }
        }
        if found {
            break;
        }
    }
    assert!(found);
}

const DX: [i32; 4] = [-1, 0, 0, 1];
const DY: [i32; 4] = [0, -1, 1, 0];
fn find_paths(grid: &Vec<Vec<char>>, sx: usize, sy: usize, target_char: char) -> Vec<Vec<char>> {
    let n = grid.len();
    let m = grid[0].len();

    let mut all_paths: Vec<Vec<usize>> = vec![];
    let mut _path: Vec<usize> = vec![];
    let mut vis: Vec<Vec<bool>> = vec![vec![false; m]; n];
    dfs(
        sx,
        sy,
        target_char,
        grid,
        &mut vis,
        &mut _path,
        &mut all_paths,
    );
    all_paths.sort_by(|a, b| a.len().cmp(&b.len()));
    let smallest_len = all_paths[0].len();
    let mut char_paths = vec![];
    for path in all_paths {
        if path.len() > smallest_len {
            break;
        }
        let mut char_path = vec![];
        for mm in &path {
            char_path.push(match mm {
                0 => '^',
                1 => '<',
                2 => '>',
                3 => 'v',
                _ => panic!(),
            });
        }
        char_path.push('A');
        char_paths.push(char_path);
    }
    return char_paths;
}

fn dfs(
    x: usize,
    y: usize,
    target_char: char,
    grid: &Vec<Vec<char>>,
    vis: &mut Vec<Vec<bool>>,
    cur_path: &mut Vec<usize>,
    all_paths: &mut Vec<Vec<usize>>,
) {
    if grid[x][y] == target_char {
        all_paths.push(cur_path.clone());
        return;
    }
    let n = grid.len();
    let m = grid[0].len();
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
            cur_path.push(i);
            dfs(
                nx as usize,
                ny as usize,
                target_char,
                grid,
                vis,
                cur_path,
                all_paths,
            );
            cur_path.pop().unwrap();
        }
    }
    vis[x][y] = false;
}

fn cartesian_product(vecs: &Vec<Vec<Vec<char>>>) -> Vec<Vec<char>> {
    let mut res = vec![vec![]];
    gogo(0, vecs, &mut res);
    return res;
}

fn gogo(x: usize, vecs: &Vec<Vec<Vec<char>>>, res: &mut Vec<Vec<char>>) {
    if x == vecs.len() {
        return;
    }
    let mut new_results = vec![];
    for u in vecs[x].iter() {
        for v in res.iter() {
            let mut cv = v.clone();
            cv.extend(u);
            new_results.push(cv);
        }
    }
    res.clear();
    res.extend(new_results);
    gogo(x + 1, &vecs, res);
}
