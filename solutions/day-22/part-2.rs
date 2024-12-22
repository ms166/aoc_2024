use std::{
    cmp::max,
    collections::{HashMap, HashSet},
    io::stdin,
};

fn main() {
    let mut input = String::new();
    let mut hm: HashMap<(i64, i64, i64, i64), i64> = HashMap::new();
    while stdin().read_line(&mut input).unwrap() != 0 {
        let mut s = input.trim().parse::<i64>().unwrap();
        let mut num = s;
        let mut p: i32 = 0;
        let mut v: Vec<i64> = vec![-1; 5];
        let mut cur_seqs: HashSet<(i64, i64, i64, i64)> = HashSet::new();
        for i in 0..2000 {
            num = num * 64;

            // mix
            num = num ^ s;

            // prune
            num = num % 16777216;
            s = num;

            num = num / 32;

            // mix
            num = num ^ s;

            // prune
            num = num % 16777216;
            s = num;

            num = num * 2048;

            // mix
            num = num ^ s;

            // prune
            num = num % 16777216;
            s = num;

            v[p as usize] = num % 10;
            if i >= 4 {
                let a = v[((5 + (p - 3) % 5) % 5) as usize] - v[((5 + (p - 4) % 5) % 5) as usize];
                let b = v[((5 + (p - 2) % 5) % 5) as usize] - v[((5 + (p - 3) % 5) % 5) as usize];
                let c = v[((5 + (p - 1) % 5) % 5) as usize] - v[((5 + (p - 2) % 5) % 5) as usize];
                let d = v[p as usize] - v[((5 + (p - 1) % 5) % 5) as usize];
                if !cur_seqs.contains(&(a, b, c, d)) {
                    cur_seqs.insert((a, b, c, d));

                    hm.entry((a, b, c, d))
                        .and_modify(|f| *f += num % 10)
                        .or_insert(num % 10);
                }
            }
            p = (p + 1) % 5;
        }
        input.clear();
    }
    let mut ans = -1;
    for x in hm.values() {
        ans = max(ans, *x);
    }
    println!("{}", ans)
}
