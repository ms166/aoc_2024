use std::io::stdin;

fn main() {
    let mut input = String::new();
    let _ = stdin().read_line(&mut input);
    let input = input.trim().to_string();
    let nums: Vec<i64> = input.split(' ').map(|x| x.parse().unwrap()).collect();

    let mut ans = 0;
    for x in nums {
        ans += go(x, 0);
    }
    println!("{}", ans);
}
fn go(mut n: i64, d: i32) -> i64 {
    let num_digs = ((n as f64).log(10.0) as i64) + 1;
    if d == 25 {
        return 1;
    }
    if n == 0 {
        return go(1, d + 1);
    } else if num_digs % 2 == 0 {
        let mut right = 0;
        let mut p = 1;
        for _ in 0..num_digs / 2 {
            right = right + p * (n % 10);
            n /= 10;
            p *= 10;
        }
        return go(n, d + 1) + go(right, d + 1);
    } else {
        return go(n * 2024, d + 1);
    }
}
