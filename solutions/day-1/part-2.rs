use std::collections::HashMap;
use std::fs;

fn main() {
    let contents = fs::read_to_string("./in")
        .expect("Should have been able to read the file");

    let mut split_by_line = contents.split("\n").collect::<Vec<&str>>();
    split_by_line.pop();
    let mut left_elements = vec![];
    let mut right_elements = vec![];
    for e in split_by_line {
        let c = e.split(" ").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        left_elements.push(c[0]);
        right_elements.push(c[1]);
    }

    let mut hm: HashMap<i64, i64> = HashMap::new();
    for x in right_elements {
        hm.entry(x).and_modify(|x| *x += 1).or_insert(1);
    }

    let mut ans: i64 = 0;
    for x in left_elements {
        let rr = hm.get(&x).unwrap_or(&0i64);
        ans += x * rr;
    }
    println!("{}", ans);
}