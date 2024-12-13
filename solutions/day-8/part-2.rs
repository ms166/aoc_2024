use std::collections::HashMap;
use std::collections::HashSet;
use std::io::stdin;
use std::mem::swap;


fn gcd(a: i32, b: i32) -> i32 {
    let mut a = a;
    let mut b = b;
    while b > 0 {
        a %= b;
        swap(&mut a, &mut b);
    }
    a
}

fn main() {
    let mut grid: Vec<Vec<char>> = Vec::new();
    for _ in 0..50 {
        let mut input = String::new();
        let _ = stdin().read_line(&mut input);
        input = input.trim().to_string();
        grid.push(input.chars().collect());
    }
    let mut pos: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let n = grid.len();
    let m = grid[0].len();
    for i in 0..n {
        for j in 0..m {
            if grid[i][j] != '.' {
                pos
                    .entry(grid[i][j])
                    .and_modify(|f| {
                        f.push((i as i32, j as i32));
                    })
                    .or_insert(vec![(i as i32, j as i32)]);
            }
        }
    }
    let mut hs: HashSet<(i32, i32)> = HashSet::new();
    for (_, vec) in pos.iter() {
        for i in 0..vec.len()-1 {
            for j in i+1..vec.len() {
                let mut dx = vec[i].0 - vec[j].0;
                let mut dy = vec[i].1 - vec[j].1;
                let g = gcd(dx.abs(), dy.abs());
                dx /= g;
                dy /= g;

                assert!(dx <= 0);
                let (mut u, mut v) = (vec[i].0, vec[i].1);
                while u >= 0 && u < n as i32 && v >= 0 && v < m as i32  {
                    hs.insert((u, v));
                    u += dx;
                    v += dy;
                }

                let (mut u, mut v) = (vec[i].0, vec[i].1);
                while u >= 0 && u < n as i32 && v >= 0 && v < m as i32  {
                    hs.insert((u, v));
                    u -= dx;
                    v -= dy;
                }
            }
        }
    }
    println!("{}", hs.len());
}
