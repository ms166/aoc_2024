use std::collections::HashMap;
use std::io::stdin;

fn main() {
    let mut input = String::new();
    let _ = stdin().read_line(&mut input);
    let input = input.trim().to_string();
    let nums: Vec<i64> = input.split(' ').map(|x| x.parse().unwrap()).collect();

    let mut ans = 0;
    let mut dp: HashMap<(i64, i64), i64> = HashMap::new();
    for x in nums {
        ans += go(x, 0, &mut dp);
    }
    println!("{}", ans);
}
fn go(mut n: i64, d: i32, dp: &mut HashMap<(i64, i64), i64>) -> i64 {
    let num_digs = ((n as f64).log(10.0) as i64) + 1;
    if dp.contains_key(&(n, d as i64)) {
        return *dp.get(&(n, d as i64)).unwrap();
    }
    if d == 75 {
        dp.insert((n, d as i64), 1);
        return 1;
    }
    if n == 0 {
        let res = go(1, d + 1, dp);
        dp.insert((n, d as i64), res);
        return res;
    } else if num_digs % 2 == 0 {
        let mut right = 0;
        let mut p = 1;
        let original_n = n;
        for _ in 0..num_digs / 2 {
            right = right + p * (n % 10);
            n /= 10;
            p *= 10;
        }
        let res = go(n, d + 1, dp) + go(right, d + 1, dp);
        dp.insert((original_n, d as i64), res);
        return res;
    } else {
        let res = go(n * 2024, d + 1, dp);
        dp.insert((n, d as i64), res);
        return res;
    }
}
