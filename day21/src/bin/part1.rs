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
    Num(i64),
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
        if word.parse::<i64>().is_ok() {
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

    let soln = solve(start, &hash);
    println!("{}", soln);

    Ok(())
}

fn solve(it: &Item, hash: &HashMap<String, Item>) -> i64 {
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
