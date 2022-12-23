#![feature(iter_array_chunks)]
use std::io;

fn main() -> io::Result<()> {
    let mut total_priority = 0;
    let mut stdin_lines_chunks = io::stdin().lines().array_chunks::<3>();
    for lines in &mut stdin_lines_chunks {
        let lines: [String; 3] = std::array::from_fn(|i| lines[i].as_ref().unwrap().clone());
        println!("{:?}", lines);
        let common_item = find_common_item_priority(&lines);
        println!("Common: {}", common_item);
        total_priority += common_item + 1;
    }
    println!("{}", total_priority);

    Ok(())
}

fn priority_from_char(c: char) -> u32 {
    if c.is_uppercase() {
        u32::try_from(c).unwrap() - u32::try_from('A').unwrap() + 27
    } else {
        u32::try_from(c).unwrap() - u32::try_from('a').unwrap() + 1
    }
}

fn find_common_item_priority(lines: &[String]) -> u32 {
    let rucksacks: [[bool; 52]; 3] = std::array::from_fn(|i| {
        let mut rucksack = [false; 52];
        lines[i]
            .chars()
            .for_each(|c| rucksack[priority_from_char(c) as usize - 1] = true);
        rucksack
    });

    let common_item = (0..52)
        .into_iter()
        .find(|&i| rucksacks[0][i as usize] && rucksacks[1][i as usize] && rucksacks[2][i as usize])
        .unwrap();

    common_item
}
