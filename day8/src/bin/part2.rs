use std::io;

struct Dir {
    step: [i32; 2],
}

impl Dir {
    fn new(step_x: i32, step_y: i32) -> Self {
        Self {
            step: [step_x, step_y],
        }
    }
}

fn main() -> io::Result<()> {
    let mut grid = Vec::new();
    for line in io::stdin().lines() {
        let line = line.unwrap();
        let mut row = Vec::new();
        for char in line.chars() {
            row.push((char.to_digit(10).unwrap() as i32, false));
        }
        grid.push(row);
    }

    let height = grid.len() as i32;
    let width = grid[0].len() as i32;

    let dirs = [
        // Start, step, next path
        Dir::new(1, 0),  // to right
        Dir::new(0, 1),  // to bottom
        Dir::new(-1, 0), // to left
        Dir::new(0, -1), // to top
    ];

    let mut highscore = 0;
    for x in 0..width {
        for y in 0..height {
            let score = dirs
                .iter()
                .map(|dir| get_score(x, y, &mut grid, dir))
                .fold(1, |acc, score| acc * score);
            highscore = std::cmp::max(score, highscore);
        }
    }

    println!("{}", highscore);

    Ok(())
}

fn get_elem<T: Copy>(x: i32, y: i32, grid: &Vec<Vec<T>>) -> T {
    grid[y as usize][x as usize]
}

fn set_elem<T>(x: i32, y: i32, value: T, grid: &mut Vec<Vec<T>>) {
    grid[y as usize][x as usize] = value;
}

fn in_bounds(x: i32, y: i32, grid: &Vec<Vec<(i32, bool)>>) -> bool {
    x >= 0 && x < grid[0].len() as i32 && y >= 0 && y < grid.len() as i32
}

fn get_score(x: i32, y: i32, grid: &Vec<Vec<(i32, bool)>>, dir: &Dir) -> u32 {
    let height = get_elem(x, y, grid).0;
    let mut pos_x = x + dir.step[0];
    let mut pos_y = y + dir.step[1];

    let mut score = 0;
    while in_bounds(pos_x, pos_y, grid) {
        score += 1;

        if get_elem(pos_x, pos_y, grid).0 >= height {
            break;
        }

        pos_x = pos_x + dir.step[0];
        pos_y = pos_y + dir.step[1];
    }

    score
}
