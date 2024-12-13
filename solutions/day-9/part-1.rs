use std::cmp::min;
use std::io::stdin;

fn nc2(n: i64) -> i64 {
    return n * (n + 1) / 2;
}

fn main() {
    let mut input = String::new();
    let _ = stdin().read_line(&mut input);
    let input = input.trim().to_string();

    // (value, id)
    let mut f: Vec<(i32, i32)> = vec![];
    for (index, c) in input.chars().enumerate() {
        if index % 2 == 0 {
            f.push((c as i32 - '0' as i32, index as i32 / 2));
        }
        else {
            f.push((c as i32 - '0' as i32, -1));
        }
    }
    let mut l = 0;
    let mut r = f.len();
    let mut res: Vec<(i32, i32)> = vec![];
    while l <= r {
        if l == r {
            res.push(f[l]);
            break;
        }
        if l % 2 == 0 {
           res.push(f[l]);
           l += 1; 
           continue;
        }
        else {
            if r % 2 == 0 {
                let mn = min(f[l].0, f[r].0);
                f[l].0 -= mn;
                f[r].0 -= mn;
                if f[l].0 == 0 {
                    res.push((mn, f[r].1));
                    l += 1;
                    continue;
                }
                else if f[r].0 == 0 {
                    res.push((mn, f[r].1));
                    r -= 1;
                    continue;
                } else {
                    panic!("asdf");
                }
            }
            else {
                r -= 1;
                continue;
            }
        }
    }
    let mut ans: i64 = 0;
    let mut last: i64= 0;
    for (_, v) in res.iter().enumerate() {
        if last == 0 {
            ans += v.1 as i64 * nc2((v.0 - 1) as i64) as i64;
        }
        else {
            ans += v.1 as i64 * (nc2((last + (v.0-1) as i64) as i64) - nc2(last-1));
        }
        last += v.0 as i64;
    }
    println!("{}", ans);
}
