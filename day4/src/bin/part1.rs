use std::io;

fn main() -> io::Result<()> {
    println!("Waiting on input...");
    let mut contained_count = 0;
    for line in io::stdin().lines() {
        let line = line.unwrap();
        let mut assignments = line.split(",");
        let first_assignment = assignments.next().unwrap();
        let mut sections = first_assignment.split("-");
        let first_start: u32 = sections.next().unwrap().parse().unwrap();
        let first_end: u32 = sections.next().unwrap().parse().unwrap();
        let second_assignment = assignments.next().unwrap();
        let mut sections = second_assignment.split("-");
        let second_start: u32 = sections.next().unwrap().parse().unwrap();
        let second_end: u32 = sections.next().unwrap().parse().unwrap();

        // print!("{} {} - ", first_start, first_end);
        // print!("{} {}", second_start, second_end);
        // println!();

        if first_start <= second_start && first_end >= second_end {
            contained_count += 1;
        } else if first_start >= second_start && first_end <= second_end {
            contained_count += 1;
        } else if first_start <= second_start && first_end >= second_start {
            contained_count += 1;
        } else if first_start <= second_end && first_end >= second_end {
            contained_count += 1;
        }
    }

    println!("{}", contained_count);
    Ok(())
}
