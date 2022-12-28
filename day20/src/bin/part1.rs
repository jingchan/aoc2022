use std::{
    collections::{HashMap, HashSet, LinkedList, VecDeque},
    io,
};

fn main() -> io::Result<()> {
    let mut nums = Vec::new();
    let mut list = Vec::new();
    let mut hash = HashSet::<i32>::new();
    for (i, line) in io::stdin().lines().enumerate() {
        let line = line.unwrap();

        list.push((i, line.parse().unwrap()));

        nums.push((i, line.parse().unwrap()));
    }
    let count = nums.len();
    // println!("{:?}", list);
    println!("{:?}", count);

    for (i, v) in &nums {
        let position = list.iter().position(|(i2, _)| i2 == i).unwrap();
        assert_eq!(list.remove(position), (*i, *v));

        let newpos = usize::try_from(
            (position as i32 + v + 4 * list.len() as i32) % list.len() as i32,
        )
        .unwrap();

        list.insert(newpos, (*i, *v));
    }

    let position = list.iter().position(|(i, x)| *x == 0).unwrap();
    println!("{:?}", list[(position + 1000) % count]);
    println!("{:?}", list[(position + 2000) % count]);
    println!("{:?}", list[(position + 3000) % count]);
    let mut score = 0;
    score += list[(position + 1000) % count].1;
    score += list[(position + 2000) % count].1;
    score += list[(position + 3000) % count].1;
    println!("Sum: {:?}", score);

    // println!("{:?}", vec);
    // println!("{:?}", list);

    Ok(())
}
