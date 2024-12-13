use std::io::stdin;

fn main() {
    let mut grid: Vec<Vec<char>> = vec![];
    for _ in 0..140 {
        let mut input = String::new();
        let _ = stdin().read_line(&mut input);
        let input = input.trim().to_string();
        grid.push(input.chars().collect());
    }
    let n = grid.len();
    let m = grid[0].len();
    let mut vis: Vec<Vec<bool>> = vec![vec![false; m]; n];
    let mut ans = 0i64;
    for i in 0..n {
        for j in 0..m {
            if !vis[i][j] {
                let res = go(i as i32, j as i32, &mut vis, &grid);
                ans += res.0 as i64 * res.1 as i64;
            }
        }
    }
    println!("{}", ans);
}

const DX: [i32; 4] = [-1, 0, 0, 1];
const DY: [i32; 4] = [0, -1, 1, 0];

fn go(x: i32, y: i32, vis: &mut Vec<Vec<bool>>, grid: &Vec<Vec<char>>) -> (i32, i32){
    let n = grid.len();
    let m = grid[0].len();
    vis[x as usize][y as usize] = true;
    let mut area = 1;
    let mut perimeter = 0;
    for i in 0..4 {
        let nx = x + DX[i];
        let ny = y + DY[i];
        if nx >= 0 && nx < n as i32 && ny >= 0 && ny < m as i32 
            && !vis[nx as usize][ny as usize] 
            && grid[x as usize][y as usize] == grid[nx as usize][ny as usize] {
            let (n_area, n_perimeter) = go(nx, ny, vis, grid);
            area += n_area;
            perimeter += n_perimeter;
        }
        if !(nx >= 0 && nx < n as i32 && ny >= 0 && ny < m as i32) || grid[x as usize][y as usize] != grid[nx as usize][ny as usize] {
            perimeter += 1;
        }
    }
    return (area, perimeter);
}