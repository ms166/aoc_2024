use std::io::stdin;

fn main() {
    let reg_a = {
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

    run_program(&program, reg_a, reg_b, reg_c);
}

fn run_program(program: &Vec<i64>, mut reg_a: i64, mut reg_b: i64, mut reg_c: i64) {
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
    println!("Output: {:?}", ans);
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
