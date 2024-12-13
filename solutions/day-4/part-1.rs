use std::io::stdin;

fn main() {
    let num_lines = 140;
    let mut grid: Vec<Vec<char>> = vec![];
    for _ in 0..num_lines {
        let mut input = String::new();
        let _ = stdin().read_line(&mut input);
        input = input.trim().to_string();
        grid.push(input.chars().collect());
    }
    let n = grid.len();
    let m = grid[0].len();
    let mut ans = 0;
    for i in 0..n {
        for j in 0..m {
            if j + 3 < m && grid[i][j..j + 4] == vec!['X', 'M', 'A', 'S'] {
                ans += 1;
            }
            if j as i32 - 3 >= 0 && grid[i][j - 3..j + 1] == vec!['S', 'A', 'M', 'X'] {
                ans += 1;
            }
            if i + 3 < n
                && grid[i][j] == 'X'
                && grid[i + 1][j] == 'M'
                && grid[i + 2][j] == 'A'
                && grid[i + 3][j] == 'S'
            {
                ans += 1;
            }
            if i as i32 - 3 >= 0
                && grid[i][j] == 'X'
                && grid[i - 1][j] == 'M'
                && grid[i - 2][j] == 'A'
                && grid[i - 3][j] == 'S'
            {
                ans += 1;
            }
            if i as i32 - 3 >= 0
                && j as i32 - 3 >= 0
                && grid[i][j] == 'X'
                && grid[i - 1][j - 1] == 'M'
                && grid[i - 2][j - 2] == 'A'
                && grid[i - 3][j - 3] == 'S'
            {
                ans += 1;
            }
            if i + 3 < n
                && j as i32 - 3 >= 0
                && grid[i][j] == 'X'
                && grid[i + 1][j - 1] == 'M'
                && grid[i + 2][j - 2] == 'A'
                && grid[i + 3][j - 3] == 'S'
            {
                ans += 1;
            }
            if i as i32 - 3 >= 0
                && j + 3 < m
                && grid[i][j] == 'X'
                && grid[i - 1][j + 1] == 'M'
                && grid[i - 2][j + 2] == 'A'
                && grid[i - 3][j + 3] == 'S'
            {
                ans += 1;
            }
            if i + 3 < n
                && j + 3 < m
                && grid[i][j] == 'X'
                && grid[i + 1][j + 1] == 'M'
                && grid[i + 2][j + 2] == 'A'
                && grid[i + 3][j + 3] == 'S'
            {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
