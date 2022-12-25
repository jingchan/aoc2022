use std::{array, cmp::Ordering, io, iter::Peekable, str::Chars};

#[derive(PartialEq, Debug)]
enum Token {
    ListStart,
    ListEnd,
    Value(i32),
}

#[derive(PartialEq, Debug)]
enum Item {
    List(Vec<Item>),
    Value(i32),
}
impl Item {
    fn parse_str(str: &String) -> Self {
        let mut chars = str.chars().peekable();

        assert_eq!(get_next_token(&mut chars), Token::ListStart);
        let it = Item::parse_impl(&mut chars);
        // assert_eq!(get_next_token(&mut chars), Token::ListEnd);
        it
    }

    fn parse_impl(chars: &mut Peekable<Chars>) -> Self {
        let mut vec = Vec::new();

        loop {
            match get_next_token(chars) {
                Token::ListStart => vec.push(Item::parse_impl(chars)),
                Token::ListEnd => break,
                Token::Value(v) => vec.push(Item::Value(v)),
            };
        }

        Item::List(vec)
    }

    fn compare(&self, other: &Item) -> std::cmp::Ordering {
        match (self, other) {
            (Item::Value(v0), Item::Value(v1)) => {
                if v0 < v1 {
                    return Ordering::Less;
                } else if v0 > v1 {
                    return Ordering::Greater;
                } else {
                    return Ordering::Equal;
                }
            }
            (Item::List(l0), Item::List(l1)) => {
                let i0 = l0.iter();
                let mut i1 = l1.iter();

                for v0 in i0 {
                    if let Some(v1) = i1.next() {
                        let res = v0.compare(v1);
                        match res {
                            Ordering::Greater => return Ordering::Greater,
                            Ordering::Less => return Ordering::Less,
                            _ => continue,
                        }
                    } else {
                        return Ordering::Greater;
                    }
                }
                if let Some(v1) = i1.next() {
                    return Ordering::Less;
                }
            }
            (Item::List(l0), Item::Value(v1)) => {
                let res =
                    self.compare(&Item::List(vec![Item::Value(v1.clone())]));
                match res {
                    Ordering::Greater => return Ordering::Greater,
                    Ordering::Less => return Ordering::Less,
                    _ => {}
                }
            }
            (Item::Value(v0), Item::List(l1)) => {
                let res =
                    Item::List(vec![Item::Value(v0.clone())]).compare(other);
                match res {
                    Ordering::Greater => return Ordering::Greater,
                    Ordering::Less => return Ordering::Less,
                    _ => {}
                }
            }
        };

        Ordering::Equal
    }
}

fn main() -> io::Result<()> {
    let depth = [0; 2];

    let mut iter = io::stdin().lines();
    let mut score = 1;
    let mut index = 0;

    let mut items = Vec::new();
    for line in iter {
        let line = line.unwrap();
        if line.is_empty() {
            continue;
        } else {
            items.push(Item::parse_str(&line));
        }
    }
    items.push(Item::List(vec![Item::List(vec![Item::Value(2)])]));
    items.push(Item::List(vec![Item::List(vec![Item::Value(6)])]));
    items.sort_by(|a, b| a.compare(b));

    let score = items
        .iter()
        .enumerate()
        .filter_map(|(i, item)| {
            if (*item == Item::List(vec![Item::List(vec![Item::Value(2)])]))
                || (*item == Item::List(vec![Item::List(vec![Item::Value(6)])]))
            {
                return Some(i + 1);
            }
            return None;
        })
        .fold(1, |acc, i| acc * i);

    println!("{}", score);

    Ok(())
}

// fn compare_tokens(
//     t0: Token,
//     t1: Token,
//     l0: &mut Peekable<Chars>,
//     l1: &mut Peekable<Chars>,
// ) -> bool {
//     match (token0, token1) {
//         (Token::Value(v0), Token::Value(v1)) => {
//             if v0 < v1 {
//                 true
//             }
//         }
//         (Token::ListStart, Token::Value(v1)) => t0.get,
//         (Token::Value(v0), Token::ListStart) => compare_tokens(l0, l1),
//     }
// }

fn get_next_token(chars: &mut Peekable<Chars>) -> Token {
    match chars.next() {
        Some('[') => Token::ListStart,
        Some(']') => {
            if let Some(c) = chars.peek() {
                if *c == ',' {
                    chars.next().unwrap();
                }
            }
            Token::ListEnd
        }
        Some(c) => {
            let mut str = c.to_string();
            while chars.peek().unwrap().is_numeric() {
                str.push(chars.next().unwrap());
            }
            let num = str.parse().unwrap();
            if *chars.peek().unwrap() == ',' {
                chars.next().unwrap();
            }
            Token::Value(num)
        }
        c => panic!("{:?}", c),
    }
}
