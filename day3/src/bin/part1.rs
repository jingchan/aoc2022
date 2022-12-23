use std::io;

fn main() -> io::Result<()> {
    let mut total_priority = 0;
    for line in io::stdin().lines() {
        let line = line.unwrap();
        let mut chars = [false; 52];
        let first = &line[0..line.len() / 2];
        let second = &line[line.len() / 2..line.len()];

        for char in first.chars() {
            let char_priority = priority_from_char(char);

            println!("{}", char_priority);
            chars[char_priority as usize - 1] = true;
        }
        for char in second.chars() {
            let char_priority = priority_from_char(char);
            if chars[char_priority as usize - 1] {
                total_priority += char_priority;
                break;
            }
        }

        println!("{}: {} {}", line, first, second);
        println!("{:?}", chars);
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
