use std::io;

fn main() -> io::Result<()> {
    println!("Waiting on input...");
    let mut max_calories: u32 = 0;
    let mut top_calories = Vec::new();
    let mut total_calories = 0;
    for line in io::stdin().lines() {
        let line = line.unwrap();
        println!("{}", line);
        if line.is_empty() {
            top_calories.push(total_calories);
            top_calories.sort_by(|a, b| {
                if a > b {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Greater
                }
            });
            top_calories.resize(3, 0);
            total_calories = 0;
        } else {
            let calories: u32 = line.parse().unwrap();
            total_calories += calories;
        }
    }
    println!(
        "{}",
        top_calories
            .into_iter()
            .reduce(|acc, cals| { acc + cals })
            .unwrap()
    );
    Ok(())
}
