use std::io::stdin;

fn main() {
    let n = 103;
    let m = 101;
    let mut input = String::new();
    let mut positions: Vec<Vec<(i32, i32)>> = vec![];
    while stdin().read_line(&mut input).unwrap() != 0 {
        let mut it = input.trim().split(' ');

        let mut pos_it = it.next().unwrap().trim()[2..].split(',');
        let py: i32 = pos_it.next().unwrap().parse().unwrap();
        let px: i32 = pos_it.next().unwrap().parse().unwrap();

        let mut vel_it = it.next().unwrap().trim()[2..].split(',');
        let vy: i32 = vel_it.next().unwrap().parse().unwrap();
        let vx: i32 = vel_it.next().unwrap().parse().unwrap();

        positions.push(process(n, m, px, py, vx, vy));
        input.clear();
    }
    for second in 0..=10000 {
        let mut grid: Vec<Vec<char>> = vec![vec!['.'; m as usize]; n as usize];
        for j in 0..positions.len() {
            let (x, y) = positions[j][second];
            grid[x as usize][y as usize] = 'X';
        }
        println!("{}", second);
        for j in 0..grid.len() {
            println!("{}", grid[j].iter().collect::<String>())
        }
        println!("");
    }
}

fn process(n: i32, m: i32, px: i32, py: i32, vx: i32, vy: i32) -> Vec<(i32, i32)> {
    let mut res = vec![];
    for i in 0..=10000 {
        let nx = (n + (px + vx * i) % n) % n;
        let ny = (m + (py + vy * i) % m) % m;
        res.push((nx, ny))
    }
    return res;
}
