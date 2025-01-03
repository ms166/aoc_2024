use std::io::stdin;

fn main() {
    let mut input = String::new();
    let (mut ax, mut ay) = (-1, -1);
    let (mut bx, mut by) = (-1, -1);
    let (mut tx, mut ty);
    let mut ans = 0;
    while stdin().read_line(&mut input).unwrap() != 0 {
        if input.len() == 1 {
            input.clear();
        } else {
            input = input.trim().to_string();
            assert!(!input.is_empty());

            let input_chars: Vec<char> = input.chars().collect();
            if input_chars[7] == 'A' {
                let ax_ay: Vec<i64> = input
                    .split(':')
                    .next_back()
                    .unwrap()
                    .split(',')
                    .map(|x| {
                        x.split('+')
                            .next_back()
                            .unwrap()
                            .trim()
                            .to_string()
                            .parse()
                            .unwrap()
                    })
                    .collect();
                ax = ax_ay[0];
                ay = ax_ay[1];
            } else if input_chars[7] == 'B' {
                let bx_by: Vec<i64> = input
                    .split(':')
                    .next_back()
                    .unwrap()
                    .split(',')
                    .map(|x| {
                        x.split('+')
                            .next_back()
                            .unwrap()
                            .trim()
                            .to_string()
                            .parse()
                            .unwrap()
                    })
                    .collect();
                bx = bx_by[0];
                by = bx_by[1];
            } else {
                let tx_ty: Vec<i64> = input
                    .split(':')
                    .next_back()
                    .unwrap()
                    .split(',')
                    .map(|x| {
                        x.split('=')
                            .next_back()
                            .unwrap()
                            .trim()
                            .to_string()
                            .parse()
                            .unwrap()
                    })
                    .collect();
                tx = tx_ty[0];
                ty = tx_ty[1];
                let res = process(ax, ay, bx, by, 10000000000000 + tx, 10000000000000 + ty);
                ans += res;
            }

            input.clear();
        }
    }
    println!("{}", ans);
}

fn process(ax: i64, ay: i64, bx: i64, by: i64, tx: i64, ty: i64) -> i64 {
    let determinant = ax * by - bx * ay;
    if determinant == 0 {
        return 0;
    }
    if (by * tx - bx * ty) % determinant != 0 {
        return 0;
    }
    if (-ay * tx + ax * ty) % determinant != 0 {
        return 0;
    }
    let (u, v) = (
        (by * tx - bx * ty) / determinant,
        (-ay * tx + ax * ty) / determinant,
    );
    return u * 3 + v;
}
