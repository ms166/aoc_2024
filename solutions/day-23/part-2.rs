use std::{
    collections::{HashMap, HashSet},
    io::stdin,
};

fn main() {
    let mut input = String::new();
    let mut adj: Vec<HashSet<usize>> = vec![];
    let mut s_to_i: HashMap<String, usize> = HashMap::new();
    let mut i_to_s: HashMap<usize, String> = HashMap::new();

    let mut edges = vec![];
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
            adj.push(HashSet::new());
        }
        while adj.len() < v + 1 {
            adj.push(HashSet::new());
        }
        adj[u].insert(v);
        adj[v].insert(u);
        edges.push((u, v));
        input.clear();
    }
    let mut best_all_set = HashSet::new();
    for i in 0..adj.len() {
        if (adj[i].len() as i32) <= (best_all_set.len() as i32) - 1 {
            continue;
        }
        let mut cur_set = HashSet::new();
        let mut best_set = HashSet::new();
        cur_set.insert(i);
        rec(i, &mut cur_set, &adj, &mut best_set);
        if best_set.len() > best_all_set.len() {
            best_all_set.clear();
            best_all_set.extend(best_set);
        }
        // since it takes a while to compute each iteration, it's worth just trying the answer produced at each iteration rather than waiting till the end
        let mut res: Vec<String> = best_all_set
            .iter()
            .map(|f| i_to_s.get(&f).unwrap().to_string())
            .collect();
        res.sort();
        let ans = res.join(",");
        println!("{:?}", ans);
    }
    let mut res: Vec<String> = best_all_set
        .into_iter()
        .map(|f| i_to_s.get(&f).unwrap().to_string())
        .collect();
    res.sort();
    let ans = res.join(",");
    // final answer
    println!("{:?}", ans);
}

fn rec(
    x: usize,
    cur_set: &mut HashSet<usize>,
    adj: &Vec<HashSet<usize>>,
    best_set: &mut HashSet<usize>,
) {
    if cur_set.len() > best_set.len() {
        best_set.clear();
        best_set.extend(cur_set.clone());
    }
    for nxt in adj[x].iter() {
        if cur_set.contains(nxt) {
            continue;
        }
        if adj[*nxt].len() < cur_set.len() {
            continue;
        }
        let mut ok = true;
        for p in cur_set.iter() {
            if !adj[*nxt].contains(p) {
                ok = false;
                break;
            }
        }
        if ok {
            cur_set.insert(*nxt);
            rec(*nxt, cur_set, adj, best_set);
            cur_set.remove(nxt);
        }
    }
}
