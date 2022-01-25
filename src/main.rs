use std::io::{stdin, stdout, Write};

fn main() {
    loop {
        print!("> ");
        stdout().flush().ok();

        let mut input = String::new();
        if let Err(err) = stdin().read_line(&mut input) {
            eprintln!("{err}");
            continue;
        }

        let (num, mode) = match input.trim().split_whitespace().collect::<Vec<&str>>()[..] {
            [num, mode] => (num, mode),
            [num] => (num, ""),
            _ => ("", ""),
        };

        match num.parse::<f64>() {
            Ok(num) => {
                let fraction = continued_fraction(num)
                    .take_while(|x| *x < 100)
                    .take(16)
                    .collect::<Vec<u64>>();
                let (p, q) = find_convergents(&fraction);
                match mode {
                    "" | "f" | "fr" | "fraction" => {
                        println!("{p}/{q}");
                    }
                    "c" | "conv" | "convergents" | "cf" => {
                        println!("{p}/{q}");
                        println!("{fraction:?}");
                    }
                    _ => eprintln!("unknown mode: '{mode}'"),
                }
            }
            Err(err) => {
                eprintln!("{err}");
                continue;
            }
        };
    }
}

fn continued_fraction(mut n: f64) -> impl Iterator<Item = u64> {
    std::iter::from_fn(move || {
        let i = n.floor();
        let f = n - i;
        n = 1.0 / f;
        Some(i as u64)
    })
}

fn find_convergents(vec: &[u64]) -> (u64, u64) {
    fn p_n(vec: &[u64]) -> u64 {
        let len = vec.len();
        match vec {
            [] => 1,
            [x] => *x,
            [.., x] => x * p_n(&vec[..(len - 1)]) + p_n(&vec[..(len - 2)]),
        }
    }
    fn q_n(vec: &[u64]) -> u64 {
        let len = vec.len();
        match vec {
            [] => 0,
            [_] => 1,
            [.., x] => x * q_n(&vec[..(len - 1)]) + q_n(&vec[..(len - 2)]),
        }
    }
    (p_n(vec), q_n(vec))
}
