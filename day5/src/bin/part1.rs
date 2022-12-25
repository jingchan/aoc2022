use std::io;

fn main() -> io::Result<()> {
    println!("Waiting on input...");
    let mut stacks: [Vec<char>; 9] = std::array::from_fn(|_| Vec::<char>::new());
    let mut stack_lines = Vec::new();
    for line in io::stdin().lines() {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }
        stack_lines.push(line.as_bytes().to_owned());
    }
    println!("{:?}", stack_lines);
    let stack_height = stack_lines.len() - 1;
    for i in 0..stack_height {
        let index = stack_lines.len() - 2 - i;
        let spots = (stack_lines[index].len() - 2) / 4 + 1;
        println!("{}", spots);
        for j in 0..spots {
            let pos = 1 + j * 4;
            // println!("index: {}", index);
            // println!("pos: {}", pos);
            // println!("line: {:?}", stack_lines[index as usize]);
            // println!("char: {:?}", stack_lines[index as usize][pos as usize]);
            let c = stack_lines[index as usize][pos as usize];
            if c.is_ascii_whitespace() {
                continue;
            }
            stacks[j].push(char::from(c));
        }
        println!("stacks: {:?}", stacks);
    }
    for line in io::stdin().lines() {
        // move 1 from 2 to 1
        // move 3 from 1 to 3
        // move 2 from 2 to 1
        // move 1 from 1 to 2
        println!("{:?}", line);
        let line = line.unwrap();
        let words = line.split(" ").collect::<Vec<_>>();
        let count: u32 = words[1].parse().unwrap();
        let from_stack = words[3].parse::<usize>().unwrap() - 1;
        let to_stack = words[5].parse::<usize>().unwrap() - 1;

        for i in 0..count {
            let movec = stacks[from_stack].pop().unwrap();
            stacks[to_stack].push(movec);
        }
    }

    for s in &stacks {
        if !s.is_empty() {
            print!("{}", s.last().unwrap().to_string());
        }
    }
    println!();
    Ok(())
}
