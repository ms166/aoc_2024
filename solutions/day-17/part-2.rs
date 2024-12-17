use std::{io::stdin, thread};

/// Runtime is roughly 1 hour
/// Will only work for my input set, with `reg_b`=`0`, `reg_c`=`0`, `program`=`[2,4,1,7,7,5,1,7,4,6,0,3,5,5,3,0]`
fn main() {
    let _reg_a = {
        let mut input = String::new();
        let _ = stdin().read_line(&mut input);
        input
            .trim()
            .split(':')
            .next_back()
            .unwrap()
            .trim()
            .parse::<i64>()
            .unwrap()
    };
    let reg_b = {
        let mut input = String::new();
        let _ = stdin().read_line(&mut input);
        input
            .trim()
            .split(':')
            .next_back()
            .unwrap()
            .trim()
            .parse::<i64>()
            .unwrap()
    };
    let reg_c = {
        let mut input = String::new();
        let _ = stdin().read_line(&mut input);
        input
            .trim()
            .split(':')
            .next_back()
            .unwrap()
            .trim()
            .parse::<i64>()
            .unwrap()
    };

    let program: Vec<i64> = {
        let mut input = String::new();
        let _ = stdin().read_line(&mut input);
        let _ = stdin().read_line(&mut input);
        input
            .trim()
            .split(':')
            .next_back()
            .unwrap()
            .trim()
            .split(',')
            .map(|x| x.parse::<i64>().unwrap())
            .collect()
    };

    let mut threads = vec![];

    let l: i64 = 16;
    let k = (l - 1) * 3;
    // fix last nth element
    let e = 1 << (k + 3);
    let s = e - (1 << k);

    // fix (n-1)th element
    let _s1 = s + ((1 << (k - 3)) * 2);
    let s2 = s + ((1 << (k - 3)) * 3);

    // fix (n-2)th element
    let cloned_program = program.clone();
    threads.push(thread::spawn(move || {
        let s3 = s2 - (2 * (1 << (k - 6)));
        let s4 = s2 - (1 * (1 << (k - 6)));
        for i in s3..s4 {
            let res = run_program(&cloned_program, i, reg_b, reg_c);
            // println!("{} -> {:?}", i, res);
            if res == cloned_program {
                println!("Found value: {}", i);
                return;
            }
        }
    }));

    // fix (n-1)th element
    let _s1 = s + ((1 << (k - 3)) * 4);
    let s2 = s + ((1 << (k - 3)) * 5);

    // fix (n-2)th element
    let cloned_program = program.clone();
    threads.push(thread::spawn(move || {
        let s3 = s2 - (6 * (1 << (k - 6)));
        let s4 = s2 - (4 * (1 << (k - 6)));
        for i in s3..s4 {
            let res = run_program(&cloned_program, i, reg_b, reg_c);
            // println!("{} -> {:?}", i, res);
            if res == cloned_program {
                println!("Found value: {}", i);
                return;
            }
        }
    }));

    // fix (n-2)th element
    let cloned_program = program.clone();
    threads.push(thread::spawn(move || {
        let s3 = s2 - (2 * (1 << (k - 6)));
        let s4 = s2 - (1 * (1 << (k - 6)));
        for i in s3..s4 {
            let res = run_program(&cloned_program, i, reg_b, reg_c);
            // println!("{} -> {:?}", i, res);
            if res == cloned_program {
                println!("Found value: {}", i);
                return;
            }
        }
    }));
    for thread in threads {
        thread.join().unwrap();
    }
}

fn run_program(program: &Vec<i64>, mut reg_a: i64, mut reg_b: i64, mut reg_c: i64) -> Vec<i64> {
    let mut i = 0;
    let mut ans: Vec<i64> = vec![];
    while i < program.len() {
        match program[i] {
            0 => {
                let operand = get_operand(program[i + 1], reg_a, reg_b, reg_c);
                let res = reg_a / (1 << operand);
                reg_a = res;
                i += 2;
            }
            1 => {
                reg_b = reg_b ^ program[i + 1];
                i += 2;
            }
            2 => {
                reg_b = get_operand(program[i + 1], reg_a, reg_b, reg_c) % 8;
                i += 2;
            }
            3 => {
                if reg_a != 0 {
                    i = program[i + 1] as usize;
                } else {
                    i += 2;
                }
            }
            4 => {
                reg_b = reg_b ^ reg_c;
                i += 2;
            }
            5 => {
                ans.push(get_operand(program[i + 1], reg_a, reg_b, reg_c) % 8);
                if ans.len() > program.len() || ans[ans.len() - 1] != program[ans.len() - 1] {
                    return vec![];
                }
                i += 2;
            }
            6 => {
                let operand = get_operand(program[i + 1], reg_a, reg_b, reg_c);
                let res = reg_a / (1 << operand);
                reg_b = res;
                i += 2;
            }
            7 => {
                let operand = get_operand(program[i + 1], reg_a, reg_b, reg_c);
                let res = reg_a / (1 << operand);
                reg_c = res;
                i += 2;
            }
            _ => panic!("Something terrible has happened"),
        };
    }
    return ans;
}

fn get_operand(x: i64, reg_a: i64, reg_b: i64, reg_c: i64) -> i64 {
    match x {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => reg_a,
        5 => reg_b,
        6 => reg_c,
        y => panic!("{} 😭", y),
    }
}
