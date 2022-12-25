use std::{
    collections::HashSet,
    io,
    ops::{Add, Sub},
};

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> Sub for Point<T>
where
    T: std::ops::Sub + std::ops::Sub<Output = T>,
{
    type Output = Point<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> Add for Point<T>
where
    T: std::ops::Add + std::ops::Add<Output = T>,
{
    type Output = Point<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn main() -> io::Result<()> {
    let mut head = Point::new(0, 0);
    let mut tail = Point::new(0, 0);
    let mut visited = HashSet::new();
    visited.insert(tail);
    for line in io::stdin().lines() {
        let line = line.unwrap();
        let mut words = line.split(" ");
        let dir = match words.next().unwrap() {
            "L" => Point::new(-1, 0),
            "R" => Point::new(1, 0),
            "U" => Point::new(0, 1),
            "D" => Point::new(0, -1),
            _ => panic!(),
        };
        let stride: u32 = words.next().unwrap().parse().unwrap();
        for _ in 0..stride {
            head = head + dir;

            let diff = head - tail;

            if i32::abs(diff.x) > 1 || i32::abs(diff.y) > 1 {
                if diff.x > 0 {
                    tail.x += 1;
                }
                if diff.x < 0 {
                    tail.x -= 1;
                }
                if diff.y > 0 {
                    tail.y += 1;
                }
                if diff.y < 0 {
                    tail.y -= 1;
                }
                visited.insert(tail);
            }
        }
    }

    println!("{}", visited.len());

    Ok(())
}
