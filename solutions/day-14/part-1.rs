use std::io::stdin;

fn main() {
    let n = 103;
    let m = 101;
    let mut input = String::new();
    let mut positions: Vec<(i32, i32)> = vec![];
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
    let (mut one, mut two, mut three, mut four) = (0, 0, 0, 0);
    for (x, y) in positions {
        if x < n / 2 && y < m / 2 {
            one += 1;
        } else if x > n / 2 && y < m / 2 {
            three += 1;
        } else if x < n / 2 && y > m / 2 {
            two += 1
        } else if x > n / 2 && y > m / 2 {
            four += 1;
        }
    }
    println!("{}", one * two * three * four);
}

fn process(n: i32, m: i32, px: i32, py: i32, vx: i32, vy: i32) -> (i32, i32) {
    let nx = (n + (px + vx * 100) % n) % n;
    let ny = (m + (py + vy * 100) % m) % m;
    (nx, ny)
}
