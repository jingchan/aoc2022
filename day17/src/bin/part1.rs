use std::io;
use utils::*;

type Grid = utils::Grid<u32, u32>;
type Point = utils::Point<i32>;
const NUM_ROUNDS: u32 = 2000022;
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
fn check_point(grid: &Grid, p: Point, sh: u32) -> bool {
    let shape = get_shape(sh);

    for s in shape {
        if !grid.check_in_bounds((p.x + s.x) as u32, (p.y + s.y) as u32) {
            return false;
        }
        if grid.get((p.x + s.x) as u32, (p.y + s.y) as u32) != 0 {
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
    let mut grid = Grid::new_with_value(7, 200000, 0);
    let mut height = 0;
    let mut lines = io::stdin().lines();
    let lines = lines.next().unwrap().unwrap();
    let chars = lines.chars().collect::<Vec<char>>();
    let mut windi = 0;

    println!("Len:{:?}", chars.len());
    // Len:40
    // Len:10091

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
            grid.set((pos.x + s.x) as u32, (pos.y + s.y) as u32, 1);
            height = height.max((pos + s).y + 1);
        }

        // print_grid(&grid, Point::new(0, 0), Point::new(7, 40));
        // println!("NewHight: {}", height);
    }

    println!("{}", height);
    heights.push(height);

    Ok(())
}
fn print_grid(grid: &Grid, p1: Point, p2: Point) {
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
