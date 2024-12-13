use std::io::stdin;

fn main() {
    let mut ans: i64 = 0;
    for _ in 0..850 {
        let mut input = String::new();
        let _ = stdin().read_line(&mut input);
        let (target, nums) = {
            input = input.trim().to_string();
            let mut s = input.split(':');
            let r: i64 = s.next().unwrap().trim().parse().unwrap();
            let k = s
                .next()
                .unwrap()
                .trim()
                .split(' ')
                .map(|x| x.parse().unwrap())
                .collect::<Vec<i64>>();
            (r, k)
        };
        let base: i32 = 3;
        let mask_up = base.pow((nums.len() - 1) as u32);
        for m in 0..mask_up {
            let mut mask = m;
            let mut res = nums[0];
            let mut i = 0;
            while i < nums.len() - 1 {
                let cur = mask % 3;
                if cur == 0 {
                    res += nums[i + 1];
                } else if cur == 1 {
                    res *= nums[i + 1];
                } else {
                    let num_digs = (nums[i + 1] as f64).log(10.0) as i64 + 1;
                    let x = (10i64).pow(num_digs as u32) as i64;
                    assert!(x > 0);
                    res = res * x + nums[i + 1];
                }
                i += 1;
                mask /= 3;
            }

            if res == target {
                ans += target;
                break;
            }
        }
    }
    println!("{}", ans);
}
