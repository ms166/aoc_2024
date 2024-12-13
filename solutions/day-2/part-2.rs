use std::io::stdin;

fn main() {
    let num_lines = 1000;
    let mut ans: i32 = 0;
    for _ in 0..num_lines {
        let mut input = String::new();
        let _ = stdin().read_line(&mut input);
        input = input.trim().to_string();
        let original_nums = input
            .split(' ')
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let original_len = original_nums.len();
        for k in 0..original_len {
            let mut nums = original_nums.clone();
            nums.remove(k);

            let mut increasing = true;
            let mut decreasing = true;
            let mut diff_ok = true;
            for j in 1..nums.len() {
                if nums[j] > nums[j - 1] {
                    decreasing = false;
                }
                if nums[j] < nums[j - 1] {
                    increasing = false;
                }
                let diff_abs = (nums[j] - nums[j - 1]).abs();
                if !(diff_abs >= 1 && diff_abs <= 3) {
                    diff_ok = false;
                }
            }
            if (increasing || decreasing) && diff_ok {
                ans += 1;
                break;
            }
        }
    }
    println!("{}", ans);
}
