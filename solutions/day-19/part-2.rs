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
        ans += rec(0, &input.chars().collect::<Vec<char>>(), &patterns, &mut dp);
        input.clear();
    }
    println!("{}", ans);
}

fn rec(x: usize, input: &Vec<char>, patterns: &Vec<Vec<char>>, dp: &mut Vec<i64>) -> i64 {
    if x == input.len() {
        return 1;
    }
    if dp[x] != -1 {
        return dp[x];
    }
    dp[x] = 0;
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
                dp[x] += rec(x + patterns[u].len(), input, patterns, dp);
            }
        }
    }
    return dp[x];
}
