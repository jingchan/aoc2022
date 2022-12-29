use std::{io, slice::Windows};

fn main() -> io::Result<()> {
    // #.######
    // #>>.<^<#
    // #.<..<<#
    // #>v.><>#
    // #<^v^^>#
    // ######.#

    let mut start_x = 0;
    let mut end_x = 0;
    let mut wall = [[false; 150]; 40];
    let mut left = [[false; 150]; 40];
    let mut right = [[false; 150]; 40];
    let mut up = [[false; 150]; 40];
    let mut down = [[false; 150]; 40];
    let mut grid = [[false; 150]; 40];
    let mut width = 0;
    let mut height = 0;

    for (i, line) in io::stdin().lines().enumerate() {
        let mut last = false;
        let line = line.unwrap();
        width = line.len();
        for (j, c) in line.chars().enumerate() {
            if i == 0 {
                if c == '.' {
                    start_x = j;
                    grid[i][j] = true;
                }
            }
            if j > 0 && c == '#' {
                last = true;
                height = i + 1;
            }
            if last {
                if c == '.' {
                    end_x = j;
                }
            }
            if c == '>' {
                right[i][j] = true;
            }
            if c == '<' {
                left[i][j] = true;
            }
            if c == '^' {
                up[i][j] = true;
            }
            if c == 'v' {
                down[i][j] = true;
            }
            if c == '#' {
                wall[i][j] = true;
            }
        }
    }

    let mut finished = false;
    let mut iters = 0;
    let dirs: [[i32; 2]; 5] = [[-1, 0], [1, 0], [0, -1], [0, 1], [0, 0]];
    // until end reached
    loop {
        iters += 1;
        // if iters == 30 {
        //     break;
        // }
        let mut nl = [[false; 150]; 40];
        let mut nr = [[false; 150]; 40];
        let mut nu = [[false; 150]; 40];
        let mut nd = [[false; 150]; 40];
        let mut ng = [[false; 150]; 40];
        for j in 0..height {
            for i in 0..width {
                if left[j][i] {
                    // println!(
                    //     "{},{}, {}",
                    //     i,
                    //     (i - 2 + (width - 2)) % (width - 2) + 1
                    // );
                    nl[j][(i + width - 2 - 2) % (width - 2) + 1] = true;
                }
                if right[j][i] {
                    nr[j][(i) % (width - 2) + 1] = true;
                }
                if up[j][i] {
                    nu[(j + height - 2 - 2) % (height - 2) + 1][i] = true;
                }
                if down[j][i] {
                    nd[(j) % (height - 2) + 1][i] = true;
                }
            }
        }
        for j in 0..height {
            for i in 0..width {
                if grid[j][i] {
                    for d in dirs {
                        if (i as i32 + d[0] < 0) {
                            continue;
                        }
                        if (j as i32 + d[1] < 0) {
                            continue;
                        }
                        let td = [
                            (i as i32 + d[0]) as usize,
                            (j as i32 + d[1]) as usize,
                        ];

                        if !nl[td[1]][td[0]]
                            && !nr[td[1]][td[0]]
                            && !nu[td[1]][td[0]]
                            && !nd[td[1]][td[0]]
                            && !wall[td[1]][td[0]]
                        {
                            ng[td[1]][td[0]] = true;

                            if td[0] == end_x && td[1] == height - 1 {
                                finished = true;
                                break;
                            }
                        }
                        if finished {
                            break;
                        }
                    }
                    if finished {
                        break;
                    }
                }
                if finished {
                    break;
                }
            }
            if finished {
                break;
            }
        }

        grid = ng;
        left = nl;
        right = nr;
        up = nu;
        down = nd;

        for j in 0..height {
            for i in 0..width {
                if grid[j][i] {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            print!(" ");
            for i in 0..width {
                if up[j][i] {
                    print!("^");
                } else {
                    print!(".");
                }
            }
            print!(" ");
            for i in 0..width {
                if down[j][i] {
                    print!("v");
                } else {
                    print!(".");
                }
            }
            print!(" ");
            for i in 0..width {
                if left[j][i] {
                    print!("<");
                } else {
                    print!(".");
                }
            }
            print!(" ");
            for i in 0..width {
                if right[j][i] {
                    print!(">");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();

        if finished {
            break;
        }
    }

    println!("found exit in :: {}", iters);

    Ok(())
}
