use std::io;
use utils::*;

// [day17/src/bin/part1.rs:140] cycle = 353185
// [day17/src/bin/part1.rs:140] tot / cycle = 2831377
// [day17/src/bin/part1.rs:140] tot % cycle = 114255
// const NUM_ROUNDS: u32 = 353185 * 2 + 114255;
const NUM_ROUNDS: u32 = 114255;
// const NUM_ROUNDS: u32 = 2000022;
// #### 0

// .#.  1
// ###
// .#.

// ..# 2
// ..#
// ###

// # 3
// #
// #
// #

// ##  4
// ##

/// represnts bottom right
fn check_point(grid: &Grid<u32>, p: Point, sh: u32) -> bool {
    let shape = get_shape(sh);

    for s in shape {
        if !grid.check_point_in_bounds(p + s) {
            return false;
        }
        if grid.get_at_point(p + s) != 0 {
            return false;
        }
    }
    return true;
}

fn get_shape(sh: u32) -> Vec<Point> {
    match sh % 5 {
        0 => vec![
            Point::new(0, 0),
            Point::new(1, 0),
            Point::new(2, 0),
            Point::new(3, 0),
        ],
        1 => vec![
            Point::new(1, 0),
            Point::new(0, 1),
            Point::new(1, 1),
            Point::new(2, 1),
            Point::new(1, 2),
        ],
        2 => vec![
            Point::new(0, 0),
            Point::new(1, 0),
            Point::new(2, 0),
            Point::new(2, 1),
            Point::new(2, 2),
        ],
        3 => vec![
            Point::new(0, 0),
            Point::new(0, 1),
            Point::new(0, 2),
            Point::new(0, 3),
        ],
        4 => vec![
            Point::new(0, 0),
            Point::new(0, 1),
            Point::new(1, 0),
            Point::new(1, 1),
        ],
        _ => panic!(),
    }
}

fn main() -> io::Result<()> {
    let mut heights = Vec::new();
    let mut grid = Grid::new_with_value(7, 200000000, 0);
    let mut height = 0;
    let mut lines = io::stdin().lines();
    let lines = lines.next().unwrap().unwrap();
    let chars = lines.chars().collect::<Vec<char>>();
    let mut windi = 0;
    let mut lastheight = 0;

    println!("Len:{:?}", chars.len());
    // Len:40
    // Len:10091
    let tot = 1000000000000u128;
    let cycle = 7 * 10091 * 5;
    dbg!(tot, cycle, tot / cycle, tot % cycle);

    for i in 0..NUM_ROUNDS {
        let mut pos = Point::new(2, height + 4);
        // println!("start pos: {:?}, shaep{}", pos, i);

        loop {
            //gravity
            if !check_point(&grid, pos + Point::new(0, -1), i) {
                // println!("break");
                break;
            }
            pos = pos + Point::new(0, -1);
            // println!("Pos: {:?}", pos);

            // dbg!(i % chars.len() as u32);
            let wind = chars[(windi % chars.len() as u32) as usize];
            windi += 1;
            let wind = match wind {
                '<' => {
                    // println!("Left");
                    Point::new(-1, 0)
                }
                '>' => {
                    // println!("Right");
                    Point::new(1, 0)
                }
                _ => panic!(),
            };
            if check_point(&grid, pos + wind, i) {
                pos = pos + wind;
                // println!("Pos: {:?}", pos);
            }
        }

        for s in get_shape(i) {
            grid.set_at_point(pos + s, 1);
            height = height.max((pos + s).y + 1);
        }

        // print_grid(&grid, Point::new(0, 0), Point::new(7, 40));
        // println!("NewHight: {}", height);

        //Len:10091
        //          1,          1
        //     534831,     534830
        //    1069654,     534823
        //    1604477,     534823
        //    2139300,     534823
        //    2674123,     534823

        // [day17/src/bin/part1.rs:140] tot = 1000000000000
        // [day17/src/bin/part1.rs:140] cycle = 353185
        // [day17/src/bin/part1.rs:140] tot / cycle = 2831377
        // [day17/src/bin/part1.rs:140] tot % cycle = 114255
        // 1000000000000
        // 0 to 2*(7 * 10091 * 5)

        // height per cycle: 534823
        // Cycles skipped: 353185
        if (i % (7 * 10091 * 5) == 0) {
            println!("{:10?}, {:10?}", height, height - lastheight);
            lastheight = height;
        }
        heights.push(height);
    }

    println!("{}", height);
    println!(
        "{}",
        height as u128 + (1069654u128 + 534823u128 * 2831375u128)
    );
    Ok(())
}
fn print_grid(grid: &Grid<u32>, p1: Point, p2: Point) {
    for y in p1.y..p2.y {
        for x in p1.x..p2.x {
            if grid.get(x as u32, y as u32) == 1 {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}
