use std::{collections::HashMap, hash::Hash, io};

#[derive(Debug, Hash, PartialEq, PartialOrd, Clone, Copy)]
enum Op {
    Mul,
    Add,
    Sub,
    Div,
}

#[derive(Debug, Hash, PartialEq, PartialOrd, Clone)]
enum Item {
    Calc(String, String, Op),
    Num(i128),
}

fn main() -> io::Result<()> {
    // let hash = HashMap<String, Item>;
    let mut hash = HashMap::new();
    for line in io::stdin().lines() {
        let line = line.unwrap();

        // root: pppw + sjmn
        // dbpl: 5
        let mut words = line.split(" ");

        let word = words.next().unwrap();
        let name = &word[0..word.len() - 1];
        let word = words.next().unwrap();
        if word.parse::<i128>().is_ok() {
            let num = word.parse().unwrap();
            hash.insert(name.to_string(), Item::Num(num));
        } else {
            let left = word;
            let op = match words.next().unwrap() {
                "+" => Op::Add,
                "-" => Op::Sub,
                "*" => Op::Mul,
                "/" => Op::Div,
                x => {
                    println!("{}", x);
                    panic!()
                }
            };
            let right = words.next().unwrap();

            hash.insert(
                name.to_string(),
                Item::Calc(left.to_string(), right.to_string(), op),
            );
        }
    }

    let start = hash.get(&"root".to_string()).unwrap();
    let (l, r) = if let Item::Calc(l, r, _) = start {
        (hash.get(l).unwrap(), hash.get(r).unwrap())
    } else {
        panic!();
    };

    let (first, second) = if find(l, &"humn".to_string(), &hash) {
        (r, l)
    } else if find(r, &"humn".to_string(), &hash) {
        (l, r)
    } else {
        panic!();
    };

    let answer = solve(first, &hash);

    // binary search don't wor
    // trying eulers -  remember now

    // 81_075_092_088_442
    let step = 0.1;
    let mut dx = 10;
    // let mut x0 = rand::random::<i32>() as i128;
    let mut x0 = 3349136384451;

    for x in 0..300000 {
        let mut x1 = x0 - dx;
        let mut x2 = x0 + dx;
        let mut y1 = try_solve(second, x1, &hash) - answer;
        let mut y2 = try_solve(second, x2, &hash) - answer;
        let ymid = (y2 + y1) / 2;
        let dydx = (y2 - y1) / (x2 - x1);

        println!(
            "p: {:20?}
    x1: {:20?} x2: {:20?}
    y1: {:20?}  y2: {:20?}
    dydx: {:20?}\nresult: {:20?}",
            x0, x1, x2, y1, y2, dydx, ymid
        );

        // Move x0
        let deltax = -(ymid as f64 / dydx as f64 * step) as i128;
        println!("deltx: {:20?}", deltax);
        // if ymid.signum() != dydx.signum() {
        x0 += deltax;
        dx = (ymid.abs() / 10).max(1);
        // } else {
        // x0 -= deltax;
        // }
    }

    Ok(())
}

fn find(it: &Item, name: &String, hash: &HashMap<String, Item>) -> bool {
    match it {
        Item::Calc(a, b, op) => {
            if a == name || b == name {
                true
            } else {
                find(hash.get(a).unwrap(), name, hash)
                    || find(hash.get(b).unwrap(), name, hash)
            }
        }
        _ => false,
    }
}

fn solve(it: &Item, hash: &HashMap<String, Item>) -> i128 {
    match it {
        Item::Calc(a, b, op) => {
            let l = solve(hash.get(a).unwrap(), hash);
            let r = solve(hash.get(b).unwrap(), hash);
            match op {
                Op::Add => l + r,
                Op::Sub => l - r,
                Op::Div => l / r,
                Op::Mul => l * r,
            }
        }
        Item::Num(n) => *n,
    }
}

fn try_solve(it: &Item, num: i128, hash: &HashMap<String, Item>) -> i128 {
    match it {
        Item::Calc(a, b, op) => {
            let l = if a == "humn" {
                num
            } else {
                try_solve(hash.get(a).unwrap(), num, hash)
            };
            let r = if b == "humn" {
                num
            } else {
                try_solve(hash.get(b).unwrap(), num, hash)
            };
            match op {
                Op::Add => l + r,
                Op::Sub => l - r,
                Op::Div => l / r,
                Op::Mul => l * r,
            }
        }
        Item::Num(n) => *n,
    }
}
