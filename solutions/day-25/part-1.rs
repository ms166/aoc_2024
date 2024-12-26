use std::io::stdin;

fn main() {
    let mut input = String::new();
    let mut grid: Vec<Vec<char>> = vec![];
    let mut locks: Vec<Vec<i32>> = vec![];
    let mut keys: Vec<Vec<i32>> = vec![];
    while stdin().read_line(&mut input).unwrap() != 0 {
        if input.len() == 1 {
            input.clear();

            if grid[0]
                .iter()
                .filter(|x| **x == '#')
                .collect::<Vec<&char>>()
                .len()
                == 5
            {
                // lock
                let mut new_lock = vec![];
                for i in 0..grid[0].len() {
                    let mut x = 0;
                    for j in 0..grid.len() {
                        x += if grid[j][i] == '#' { 1 } else { 0 };
                    }
                    new_lock.push(x);
                }
                locks.push(new_lock);
            } else {
                // key
                assert!(
                    grid[grid.len() - 1]
                        .iter()
                        .filter(|x| **x == '#')
                        .collect::<Vec<&char>>()
                        .len()
                        == 5
                );
                let mut new_key = vec![];
                for i in 0..grid[0].len() {
                    let mut x = 0;
                    for j in 0..grid.len() {
                        x += if grid[j][i] == '#' { 1 } else { 0 };
                    }
                    new_key.push(x);
                }
                keys.push(new_key);
            }

            grid.clear();
            continue;
        }
        grid.push(input.trim().to_string().chars().collect());
        input.clear();
    }
    if grid[0]
        .iter()
        .filter(|x| **x == '#')
        .collect::<Vec<&char>>()
        .len()
        == 5
    {
        // lock
        let mut new_lock = vec![];
        for i in 0..grid[0].len() {
            let mut x = 0;
            for j in 0..grid.len() {
                x += if grid[j][i] == '#' { 1 } else { 0 };
            }
            new_lock.push(x);
        }
        locks.push(new_lock);
    } else {
        // key
        assert!(
            grid[grid.len() - 1]
                .iter()
                .filter(|x| **x == '#')
                .collect::<Vec<&char>>()
                .len()
                == 5
        );
        let mut new_key = vec![];
        for i in 0..grid[0].len() {
            let mut x = 0;
            for j in 0..grid.len() {
                x += if grid[j][i] == '#' { 1 } else { 0 };
            }
            new_key.push(x);
        }
        keys.push(new_key);
    }

    let mut ans = 0;
    for lock in locks {
        for key in keys.iter() {
            assert!(lock.len() == key.len());
            let mut ok = true;
            for i in 0..lock.len() {
                if lock[i] + key[i] > 7 {
                    ok = false;
                    break;
                }
            }
            if ok {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
