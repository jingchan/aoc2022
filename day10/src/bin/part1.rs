#![feature(iter_advance_by)]
use std::io;

fn main() -> io::Result<()> {
    let mut val = 0;
    let mut register = 1;
    let mut cycles = 0;
    for line in io::stdin().lines() {
        let mut to_diff: Option<i32> = None;
        let line = line.unwrap();
        if line.starts_with("n") {
            cycles += 1;
        } else {
            let mut words = line.split(" ");
            words.advance_by(1);
            to_diff = Some(words.next().unwrap().parse().unwrap());

            cycles += 1;
        }
        if cycles == 20 || (cycles - 20) % 40 == 0 {
            println!("Add: {}", register * cycles);
            val += register * cycles;
        }

        if let Some(diff) = to_diff {
            cycles += 1;

            if cycles == 20 || (cycles - 20) % 40 == 0 {
                println!("Add: {}", register * cycles);
                val += register * cycles;
            }
            register += diff;

            to_diff = None;
        }
    }

    println!("{}", val);

    Ok(())
}
