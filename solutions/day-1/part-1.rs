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
    left_elements.sort();
    right_elements.sort();
    let mut ans: i64 = 0;
    for i in 0..left_elements.len() {
        ans += (left_elements[i]  - right_elements[i]).abs();
    }
    println!("{}", ans);
}