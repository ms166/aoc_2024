use std::{collections::HashMap, io::stdin};

#[derive(Debug)]
struct Child {
    left: String,
    right: String,
    op: String,
}

fn main() {
    let mut input = String::new();

    let mut adj: HashMap<String, Child> = HashMap::new();
    let mut init_values: HashMap<String, i32> = HashMap::new();
    let mut f = true;
    while stdin().read_line(&mut input).unwrap() != 0 {
        if input.len() == 1 {
            input.clear();
            f = false;
            continue;
        }
        if f {
            let split_inputs: Vec<String> = input
                .trim()
                .split(':')
                .map(|x| x.trim().to_string())
                .collect();
            init_values
                .entry(split_inputs[0].clone())
                .and_modify(|_| panic!("shouldn't exist"))
                .or_insert(split_inputs[1].parse::<i32>().unwrap());
        } else {
            let split_inputs: Vec<String> = input
                .trim()
                .split("->")
                .map(|x| x.trim().to_string())
                .collect();
            let split_inputs_2: Vec<String> = split_inputs[0]
                .split(' ')
                .map(|x| x.trim().to_string())
                .collect::<Vec<String>>();
            adj.entry(split_inputs[1].clone())
                .and_modify(|_| panic!("shouldn't exist"))
                .or_insert(Child {
                    left: split_inputs_2[0].clone(),
                    right: split_inputs_2[2].clone(),
                    op: split_inputs_2[1].clone(),
                });
        }

        input.clear();
    }

    let mut dec = 0u64;
    let mut i = 0;
    loop {
        let s = format!("z{:0>2}", i);
        if adj.contains_key(&s) {
            let res = dfs(&s, &adj, &init_values);
            dec = dec | (((res & 1) as u64) << i);
        } else {
            break;
        }
        i += 1;
    }
    println!("{}", dec);
}

fn dfs(cur: &String, adj: &HashMap<String, Child>, init_values: &HashMap<String, i32>) -> i32 {
    if init_values.contains_key(cur) {
        return *init_values.get(cur).unwrap();
    }
    let child = adj.get(cur).unwrap();
    match child.op.as_str() {
        "AND" => {
            return dfs(&child.left, adj, init_values) & dfs(&child.right, adj, init_values);
        }
        "OR" => {
            return dfs(&child.left, adj, init_values) | dfs(&child.right, adj, init_values);
        }
        "XOR" => {
            return dfs(&child.left, adj, init_values) ^ dfs(&child.right, adj, init_values);
        }
        _ => panic!("unexpected op value"),
    }
}
