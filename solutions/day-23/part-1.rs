use std::{collections::HashMap, io::stdin};

fn main() {
    let mut input = String::new();
    let mut adj: Vec<Vec<usize>> = vec![];
    let mut s_to_i: HashMap<String, usize> = HashMap::new();
    let mut i_to_s: HashMap<usize, String> = HashMap::new();

    let mut c = 0;
    while stdin().read_line(&mut input).unwrap() != 0 {
        let nodes: Vec<String> = input.trim().split('-').map(|x| x.to_string()).collect();
        for node in nodes.iter() {
            if !s_to_i.contains_key(node) {
                s_to_i.insert(node.clone(), c);
                i_to_s.insert(c, node.clone());
                c += 1;
            }
        }
        let u = *s_to_i.get(&nodes[0]).unwrap();
        let v = *s_to_i.get(&nodes[1]).unwrap();
        while adj.len() < u + 1 {
            adj.push(vec![]);
        }
        while adj.len() < v + 1 {
            adj.push(vec![]);
        }
        adj[u].push(v);
        adj[v].push(u);
        input.clear();
    }
    let mut ans = 0;
    for x in 0..adj.len() - 2 {
        for y in x + 1..adj.len() - 1 {
            for z in y + 1..adj.len() {
                if adj[x].contains(&y) && adj[y].contains(&z) && adj[x].contains(&z) {
                    if i_to_s.get(&x).unwrap().chars().collect::<Vec<char>>()[0] == 't'
                        || i_to_s.get(&y).unwrap().chars().collect::<Vec<char>>()[0] == 't'
                        || i_to_s.get(&z).unwrap().chars().collect::<Vec<char>>()[0] == 't'
                    {
                        ans += 1;
                    }
                }
            }
        }
    }
    println!("{}", ans);
}
