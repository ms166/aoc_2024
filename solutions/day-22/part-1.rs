use std::io::stdin;

fn main() {
    let mut input = String::new();
    let mut ans = 0i64;
    while stdin().read_line(&mut input).unwrap() != 0 {
        let mut s = input.trim().parse::<i64>().unwrap();
        let mut num = s;
        for _ in 0..2000 {
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
        }
        ans += num;
        input.clear();
    }
    println!("{}", ans)
}
