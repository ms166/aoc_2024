use std::io::stdin;

fn main() {
    let num_lines = 6;
    let mut total_input = String::new();
    for _ in 0..num_lines {
        let mut input = String::new();
        let _ = stdin().read_line(&mut input);
        input = input.trim().to_string();
        total_input.push_str(&input);
    }
    total_input = total_input.trim().to_string();
    let s: Vec<char> = total_input.chars().collect();
    let mut ans = 0;

    let mul_string: Vec<char> = "mul(".chars().collect();
    let mut i = 0;
    while i < s.len() {
        let mut j = i;
        while j < s.len() && j - i < 4 && s[j] == mul_string[j - i] {
            j += 1;
        }
        if j - i != 4 {
            if i == j {
                i = i + 1;
            } else {
                i = j;
            }
            continue;
        }
        let mut num_1 = 0;
        while j < s.len() && s[j] >= '0' && s[j] <= '9' {
            num_1 = num_1 * 10 + (s[j] as i32 - '0' as i32);
            j += 1;
        }
        if j == s.len() || s[j] != ',' {
            i = j;
            continue;
        }
        j += 1;
        let mut num_2 = 0;
        while j < s.len() && s[j] >= '0' && s[j] <= '9' {
            num_2 = num_2 * 10 + (s[j] as i32 - '0' as i32);
            j += 1;
        }
        if j == s.len() || s[j] != ')' {
            i = j;
            continue;
        }
        ans += num_1 * num_2;
        i = j;
    }
    println!("{}", ans);
}
