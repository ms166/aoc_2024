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
    let mut r = f.len() - 1;
    loop {
        while f[r].1 == -1 {
            if r == 0 {
                break;
            }
            r -= 1;
        }
        if r == 0 && f[r].1 == -1 {
            break;
        }
        for i in 0..r {
            if f[i].1 == -1 {
                if f[i].0 >= f[r].0 {
                    if f[i].0 == f[r].0 {
                        f[i].1 = f[r].1;
                        f[r].1 = -1;
                    } else {
                        let rem = f[i].0 - f[r].0; 
                        f[i] = f[r];
                        f[r].1 = -1;

                        // insert empty space
                        f.insert(i+1, (rem, -1));
                        r += 1;
                    }
                    break;
                }
            }
        }
        if r == 0 {
            break;
        }
        r -= 1;
    }
    let res = &f;

    let mut ans = 0;
    let mut last = 0;
    for (_, v) in res.iter().enumerate() {
        if last == 0 {
            if v.1 != -1 {
                ans += v.1 as i64 * nc2((v.0 - 1) as i64) as i64;
            }
        }
        else {
            if v.1 != -1 {
                ans += v.1 as i64 * (nc2((last + (v.0-1) as i64) as i64) - nc2(last-1));
            }
        }
        last += v.0 as i64;
    }
    println!("{}", ans);
}
