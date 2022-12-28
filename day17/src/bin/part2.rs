use std::io;

type Grid = utils::Grid<i128, i128>;
type Point = utils::Point<i128>;
// [day17/src/bin/part1.rs:140] tot = 1000000000000
// [day17/src/bin/part1.rs:140] cycle = 353185
// [day17/src/bin/part1.rs:140] tot / cycle = 2831377
// [day17/src/bin/part1.rs:140] tot % cycle = 114255
// 1000000000000
// 0 to 2*(7 * 10091 * 5)
// [day17/src/bin/part1.rs:140] cycle = 353185
// [day17/src/bin/part1.rs:140] tot / cycle = 2831377
// [day17/src/bin/part1.rs:140] tot % cycle = 114255
// const NUM_ROUNDS: u32 = 353185 * 2 + 114255;
// const CYCLE_SIZE: i128 = 353185; // 50455 * 7
// const CYCLE_SIZE: i128 = 50455; // 10091*5
// const CYCLE_SIZE: i128 = 10470; // 10091*5
// const CYCLE_SIZE: i128 = 1047;
const CYCLE_SIZE: i128 = 349 * 5;

const PRELUDE_CYCLES: i128 = 2;
const CYCLE_HEIGHT: i128 = 2785;
const TARGET_ROUNDS: i128 = 1000000000000;
const REMAINDER_ROUNDS: i128 = TARGET_ROUNDS % CYCLE_SIZE;
const SKIPPED_CYCLES: i128 = TARGET_ROUNDS / CYCLE_SIZE - PRELUDE_CYCLES;

const NUM_ROUNDS: i128 = PRELUDE_CYCLES * CYCLE_SIZE + REMAINDER_ROUNDS;
// const NUM_ROUNDS: i128 = 1_000_000;
const COMPUTED_SKIPPED_HEIGHT: i128 = SKIPPED_CYCLES * CYCLE_HEIGHT;
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
fn check_point(grid: &Grid, p: Point, sh: i128) -> bool {
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

fn get_shape(sh: i128) -> Vec<Point> {
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
    let mut deltas = Vec::new();
    let mut grid = Grid::new_with_value(7, 200000000, 0);
    let mut height: i128 = 0;
    let mut lines = io::stdin().lines();
    let lines = lines.next().unwrap().unwrap();
    let chars = lines.chars().collect::<Vec<char>>();
    let mut windi = 0;
    let mut lastheight = 0;

    // println!("Len:{:?}", chars.len());
    // Len:40
    // Len:10091
    // let tot = 1000000000000i128;
    // let cycle = 7 * 10091 * 5;
    // dbg!(tot, cycle, tot / cycle, tot % cycle);
    // let cycle = 10091 * 5;
    // dbg!(cycle);

    dbg!(NUM_ROUNDS);

    for i in 0..NUM_ROUNDS {
        let mut pos = Point::new(2, (height + 4) as i128);
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
        // 0 to 2 * (7 * 10091 * 5)

        // height per cycle: 534823
        // Cycles skipped: 353185

        // let cycle_size = 7 * 10091 * 5;
        let cycle_size = CYCLE_SIZE;
        if i % cycle_size == 0 {
            println!(
                "[Round {:10?}]: Height:{:10?}, Delta:{:10?}, cycle:{:10?}",
                i,
                height,
                height - lastheight,
                cycle_size
            );
            lastheight = height;
        }

        if heights.len() == 0 {
            deltas.push(height);
        } else {
            deltas.push(height - heights[heights.len() - 1]);
        }
        heights.push(height);
    }

    // Stride: 10470: Skip: 200000:
    //             avg ddh:                            0.0
    //             abs avg ddh:                        0.0
    // Check for cycles
    // let skip = 100000;
    // let skip_from = 200000;
    // let skip_count = 1;
    // let mut iters = 0;
    // for stride in 10000..100000 {
    //     for skip in 200000..200001 {
    //         let mut sum: f64 = 0.0;
    //         let mut abs_sum: f64 = 0.0;
    //         let mut ddh_count = 0;

    //         let mut last_dh = (heights[usize::try_from(skip + stride).unwrap()]
    //             - heights[usize::try_from(skip).unwrap()])
    //             as f64;
    //         for j in skip..NUM_ROUNDS - stride {
    //             let dh = (heights[usize::try_from(j + stride).unwrap()]
    //                 - heights[usize::try_from(j).unwrap()])
    //                 as f64;
    //             let ddh = dh - last_dh as f64;
    //             sum += ddh;
    //             abs_sum += ddh.abs();
    //             ddh_count += 1;
    //             last_dh = dh;
    //         }

    //         if abs_sum == 0.0 || iters % 100 == 0 {
    //             println!(
    //                 "Stride: {:4?}: Skip: {:4?}:
    //             avg ddh: {:30?}
    //             abs avg ddh: {:30?}",
    //                 stride,
    //                 skip,
    //                 sum / ddh_count as f64,
    //                 abs_sum / ddh_count as f64
    //             );
    //         }
    //         iters += 1;
    //     }
    // }

    println!("{}", height);

    let computed_answer = height + COMPUTED_SKIPPED_HEIGHT;
    println!("{}", computed_answer);

    // 1514285714288
    // let known_answer = 1514285714288i128;
    // println!("Diff: {}", computed_answer - known_answer);

    Ok(())
}
fn print_grid(grid: &Grid, p1: Point, p2: Point) {
    for y in p1.y..p2.y {
        for x in p1.x..p2.x {
            if grid.get(x, y) == 1 {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}
