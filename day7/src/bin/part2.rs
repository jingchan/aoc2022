use std::{cmp::min, io};

fn main() -> io::Result<()> {
    let mut lines = io::stdin().lines();
    let mut acc = 999_999_999;
    let total_size = parse_path(&mut lines, &mut acc);
    println!("{}", total_size);
    println!("{}", acc);
    Ok(())
}

fn parse_path(lines: &mut io::Lines<io::StdinLock>, acc: &mut u32) -> u32 {
    let mut dir_size = 0;
    while let Some(line) = lines.next() {
        let line = line.unwrap();
        if line.starts_with("$ cd .") {
            break;
        } else if line.starts_with("$ c") {
            dir_size += parse_path(lines, acc);
        } else if line.starts_with("$ l") {
        } else if line.starts_with("dir") {
        } else {
            dbg!(&line);
            let next = line.split(" ").next().unwrap();
            dir_size += next.parse::<u32>().unwrap();
        }
    }

    // 44965705 space used.
    if dir_size >= 30000000 - (70000000 - 44965705) {
        *acc = min(*acc, dir_size);
    }
    dir_size
}
