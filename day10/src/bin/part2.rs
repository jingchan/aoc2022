#![feature(iter_advance_by)]
use std::io;

fn main() -> io::Result<()> {
    let mut val = 0;
    let mut register = 1;
    let mut cycles = 0;
    let mut crt_x = 0;
    for line in io::stdin().lines() {
        let line = line.unwrap();
        advance_cycle(&mut crt_x, register);
        if line.starts_with("n") {
            cycles += 1;
        } else {
            cycles += 1;
            advance_cycle(&mut crt_x, register);
            cycles += 1;

            let mut words = line.split(" ");
            words.advance_by(1).unwrap();
            register += words.next().unwrap().parse::<i32>().unwrap();
        }
    }
    Ok(())
}

fn advance_cycle(crt_x: &mut i32, register: i32) {
    if i32::abs(*crt_x - register) <= 1 {
        print!("#");
    } else {
        print!(".");
    }

    *crt_x += 1;
    if *crt_x >= 40 {
        println!();
        *crt_x = 0;
    }
}
