use std::io::stdin;

fn main() {
    let mut input = String::new();
    let _ = stdin().read_line(&mut input).unwrap();
    let patterns = input
        .split(',')
        .map(|x| x.trim().chars().collect())
        .collect::<Vec<Vec<char>>>();
    let _ = stdin().read_line(&mut input).unwrap();
    input.clear();

    let mut ans = 0;
    while stdin().read_line(&mut input).unwrap() != 0 {
        input = input.trim().to_string();
        let mut dp = vec![-1; input.len()];
        if rec(0, &input.chars().collect::<Vec<char>>(), &patterns, &mut dp) {
            ans += 1;
        }
        input.clear();
    }
    println!("{}", ans);
}

fn rec(x: usize, input: &Vec<char>, patterns: &Vec<Vec<char>>, dp: &mut Vec<i32>) -> bool {
    if x == input.len() {
        return true;
    }
    if dp[x] != -1 {
        return if dp[x] == 0 { false } else { true };
    }
    let mut found = false;
    for u in 0..patterns.len() {
        let mut matches = true;
        if x + patterns[u].len() <= input.len() {
            for j in 0..patterns[u].len() {
                if input[x + j] != patterns[u][j] {
                    matches = false;
                    break;
                }
            }
            if matches {
                found = found || rec(x + patterns[u].len(), input, patterns, dp);
                if found {
                    dp[x] = 1;
                    return true;
                }
            }
        }
    }
    dp[x] = 0;
    return false;
}
