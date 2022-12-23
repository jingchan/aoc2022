use std::io;

fn main() -> io::Result<()> {
    println!("Waiting on input...");
    let mut max_calories: u32 = 0;
    let mut total_calories = 0;
    for line in io::stdin().lines() {
        let line = line.unwrap();
        println!("{}", line);
        if line.is_empty() {
            max_calories = std::cmp::max(max_calories, total_calories);
            total_calories = 0;
        } else {
            let calories: u32 = line.parse().unwrap();
            total_calories += calories;
        }
    }
    println!("{}", max_calories);
    Ok(())
}
