use std::io;

#[derive(Debug)]
enum Operation {
    Add(i64),
    Mult(i64),
    Square,
}

#[derive(Debug)]
struct Monkey {
    items: Vec<Vec<i64>>,
    op: Operation,
    div: i64,
    pass: i64,
    fail: i64,
    inspects: u64,
}
fn main() -> io::Result<()> {
    let mut lines = io::stdin().lines();
    let mut monkeys = Vec::new();

    while lines.next().is_some() {
        // Start with a placeholder for worry level.
        let items: Vec<Vec<i64>> = lines
            .next()
            .unwrap()
            .unwrap()
            .split(&[':', ','][..])
            .skip(1)
            .map(|worry_string| {
                let mut vec = Vec::new();
                let worry = worry_string.trim().parse().unwrap();
                vec.push(worry);
                vec
            })
            .collect();

        let op = parse_operation(&lines.next().unwrap().unwrap());
        let div = extract_num(&lines.next().unwrap().unwrap());
        let pass = extract_num(&lines.next().unwrap().unwrap());
        let fail = extract_num(&lines.next().unwrap().unwrap());
        lines.next();

        monkeys.push(Monkey {
            items,
            op,
            div,
            pass,
            fail,
            inspects: 0,
        });
    }

    // Fill out worry level vec for each item
    for m_ind in 0..monkeys.len() {
        for i_ind in 0..monkeys[m_ind].items.len() {
            let worry = monkeys[m_ind].items[i_ind][0];
            let mut vec = Vec::new();
            for m_inner_ind in 0..monkeys.len() {
                vec.push(worry % monkeys[m_inner_ind].div);
            }
            std::mem::swap(&mut monkeys[m_ind].items[i_ind], &mut vec);
        }
    }

    for _round in 0..10000 {
        for m_ind in 0..monkeys.len() {
            let mut items = Vec::new();

            {
                std::mem::swap(&mut items, &mut monkeys[m_ind].items);
            }

            let new_items: Vec<Vec<i64>> = items
                .into_iter()
                .map(|it| {
                    monkeys[m_ind].inspects += 1;
                    match monkeys[m_ind].op {
                        Operation::Square => it
                            .into_iter()
                            .enumerate()
                            .map(|(i, it)| (it * it) % monkeys[i].div)
                            .collect(),
                        Operation::Mult(x) => it
                            .into_iter()
                            .enumerate()
                            .map(|(i, it)| (it * x) % monkeys[i].div)
                            .collect(),
                        Operation::Add(x) => it
                            .into_iter()
                            .enumerate()
                            .map(|(i, it)| (it + x) % monkeys[i].div)
                            .collect(),
                    }
                })
                .collect();

            new_items.into_iter().for_each(|it| {
                if it[m_ind] == 0 {
                    let pass = monkeys[m_ind].pass as usize;
                    monkeys[pass].items.push(it);
                } else {
                    let fail = monkeys[m_ind].fail as usize;
                    monkeys[fail as usize].items.push(it);

                    monkeys[m_ind].items.clear();
                }
            });
        }
    }

    monkeys.sort_by(|a, b| a.inspects.cmp(&b.inspects).reverse());
    println!("{} {}", monkeys[0].inspects, monkeys[1].inspects);
    println!("{}", monkeys[0].inspects * monkeys[1].inspects);

    Ok(())
}
fn parse_operation(str: &String) -> Operation {
    // println!("String to parse: {}", str);
    if str.starts_with("  Operation: new = old * old") {
        Operation::Square
    } else if str.starts_with("  Operation: new = old * ") {
        let num = extract_num(str);
        Operation::Mult(num)
    } else if str.starts_with("  Operation: new = old + ") {
        let num = extract_num(str);
        Operation::Add(num)
    } else {
        panic!()
    }
}

fn extract_num(str: &String) -> i64 {
    let extract = str.matches(char::is_numeric).collect::<String>();
    // dbg!(&extract);
    extract.parse().unwrap()
}
