use std::collections::HashMap;
use std::collections::HashSet;
use std::io::stdin;

fn main() {
    let mut rules: HashMap<i32, HashSet<i32>> = HashMap::new();
    for _ in 0..1176 {
        let mut input = String::new();
        let _ = stdin().read_line(&mut input);
        input = input.trim().to_string();
        let nums = input.split('|').map(|x| x.parse().unwrap()).collect::<Vec<i32>>();

        if rules.contains_key(&nums[0]) {
            let entry = rules.get_mut(&nums[0]);
            entry.unwrap().insert(nums[1]);
        }
        else {
            let mut hs = HashSet::new();
            hs.insert(nums[1]);
            rules.insert(nums[0], hs);
        }
    }
    {
        let mut input = String::new();
        let _ = stdin().read_line(&mut input);
    }
    
    let mut ans = 0;
    for _ in 0..202 {
        let mut input = String::new();
        let _ = stdin().read_line(&mut input);
        input = input.trim().to_string();
        let nums = input.split(',').map(|x| x.parse().unwrap()).collect::<Vec<i32>>();
        let mut ok = true;
        for i in 1..nums.len() {
            let cr = rules.get(&nums[i]);
            if cr.is_none() {
                continue;
            }
            for j in 0..i {
                if cr.unwrap().contains(&nums[j]) {
                    ok = false;
                    break;
                }
            }  
            if !ok {
                break;
            }
        }
        if ok {
            ans += nums[nums.len()/2];
        }
    }
    println!("{}", ans);
}
