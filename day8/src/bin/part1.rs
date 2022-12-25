use std::io;

struct Dir {
    start: [i32; 2],
    step: [i32; 2],

    // How to change start for next iteration.
    start_inc: [i32; 2],
}

impl Dir {
    fn new(
        start_x: i32,
        start_y: i32,
        step_x: i32,
        step_y: i32,
        start_inc_x: i32,
        start_inc_y: i32,
    ) -> Self {
        Self {
            start: [start_x, start_y],
            step: [step_x, step_y],
            start_inc: [start_inc_x, start_inc_y],
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
        Dir::new(0, 0, 1, 0, 0, 1),           // left to right
        Dir::new(0, 0, 0, 1, 1, 0),           // top to bottom
        Dir::new(width - 1, 0, -1, 0, 0, 1),  // right to left
        Dir::new(0, height - 1, 0, -1, 1, 0), // bottom to top
    ];

    for dir in dirs.iter() {
        mark_visibile(&mut grid, dir);
    }

    println!("{}", count_visibile(&grid));

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

fn mark_visibile(grid: &mut Vec<Vec<(i32, bool)>>, dir: &Dir) {
    let mut start_x = dir.start[0];
    let mut start_y = dir.start[1];

    while in_bounds(start_x, start_y, grid) {
        let mut next_x = start_x;
        let mut next_y = start_y;
        let mut highest = -1;
        while in_bounds(next_x, next_y, grid) {
            let (height, _) = get_elem(next_x, next_y, grid);

            if height > highest {
                set_elem(next_x, next_y, (height, true), grid)
            }

            highest = std::cmp::max(highest, height);
            next_x += dir.step[0];
            next_y += dir.step[1];
        }

        start_x += dir.start_inc[0];
        start_y += dir.start_inc[1];
    }
}

fn count_visibile(grid: &Vec<Vec<(i32, bool)>>) -> u32 {
    let mut count = 0;
    for row in grid {
        for tree in row {
            if tree.1 {
                count += 1;
            }
        }
    }
    count
}
