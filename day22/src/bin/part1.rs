#![feature(is_some_and)]
use core::panic;
use std::io;
use utils;

type Point = utils::Point<i64>;

#[derive(PartialEq, Debug, Clone, Copy)]
enum Wrap {
    Unmapped,
    Teleport(Point),
    Wall,
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum Tile {
    Empty,
    Floor,
    Wall,

    // Destination point caching
    Wrap([Wrap; 4]),
}

const GRID_WIDTH: u32 = 150;
const GRID_HEIGHT: u32 = 200;
fn main() -> io::Result<()> {
    let mut pos = Point::new(0, 0);
    let mut dir: u32 = 0; // right down left, up
    let mut map = [[Tile::Empty; GRID_WIDTH as usize]; GRID_HEIGHT as usize];
    let mut lines = io::stdin().lines();

    let mut j = 0;
    loop {
        let line = lines.next().unwrap().unwrap();
        if line.is_empty() {
            break;
        }
        for (i, char) in line.chars().enumerate() {
            map[j][i] = match char {
                '.' => Tile::Floor,
                '#' => Tile::Wall,
                _ => Tile::Empty,
            }
        }
        j += 1;
    }

    for (i, t) in map[0].iter().enumerate() {
        if *t == Tile::Floor {
            pos = Point::new(i as _, 0);
            break;
        }
    }

    let line = lines.next().unwrap();
    let line = line.unwrap();
    let mut chars = line.chars().peekable();
    loop {
        if let Some(c) = chars.next() {
            if c.is_digit(10) {
                let mut st = c.to_string();
                while chars.peek().is_some_and(|x| x.is_digit(10)) {
                    st.push(chars.next().unwrap());
                }

                let moves: u32 = st.parse().unwrap();
                println!("Move {} {:?} {:?}", moves, pos, dir);
                do_moves(&mut pos, &mut dir, moves, &mut map);
                println!("After: {} {:?} {:?}", moves, pos, dir);
            } else if c == 'L' {
                dir = (dir + 4 - 1) % 4;
            } else if c == 'R' {
                dir = (dir + 1) % 4;
            } else {
                // nothing other than digits and L and R
                panic!()
            }
        } else {
            break;
        }
    }
    println!("{:?} {:?}", pos, dir);
    let score = (pos.y + 1) * 1000 + 4 * (pos.x + 1) + dir as i64;
    println!("Score: {:?}", score);

    Ok(())
}
fn do_moves(
    pos: &mut Point,
    dir: &mut u32,
    moves: u32,
    map: &mut [[Tile; GRID_WIDTH as usize]; GRID_HEIGHT as usize],
) {
    let d = match dir {
        0 => Point::new(1, 0),
        1 => Point::new(0, 1),
        2 => Point::new(-1, 0),
        3 => Point::new(0, -1),
        _ => panic!(),
    };

    for i in 0..moves {
        let mut new_pos = Point::new(
            (pos.x + d.x + i64::try_from(GRID_WIDTH).unwrap())
                % i64::try_from(GRID_WIDTH).unwrap(),
            (pos.y + d.y + i64::try_from(GRID_HEIGHT).unwrap())
                % i64::try_from(GRID_HEIGHT).unwrap(),
        );

        if new_pos.x < 0 {
            new_pos.x = i64::try_from(GRID_WIDTH).unwrap() - 1;
        }
        if new_pos.y < 0 {
            new_pos.y = i64::try_from(GRID_HEIGHT).unwrap() - 1;
        }
        match map[usize::try_from(new_pos.y).unwrap()]
            [usize::try_from(new_pos.x).unwrap()]
        {
            Tile::Floor => {
                *pos = new_pos;
            }
            Tile::Wall => {
                break;
            }
            Tile::Empty => {
                // trace and then either we turn this into a wall or a teleport
                trace_and_cache_wrap(pos, new_pos, dir, map);
            }
            Tile::Wrap(wraps) => match wraps[usize::try_from(*dir).unwrap()] {
                Wrap::Teleport(p) => {
                    *pos = p;
                }
                Wrap::Wall => {
                    break;
                }
                Wrap::Unmapped => {
                    trace_and_cache_wrap(pos, new_pos, dir, map);
                }
            },
        }
    }
}

fn trace_and_cache_wrap(
    pos: &mut Point,
    new_pos: Point,
    dir: &mut u32,
    map: &mut [[Tile; GRID_WIDTH as usize]; GRID_HEIGHT as usize],
) {
    // Original dir
    let d = match dir {
        0 => Point::new(1, 0),
        1 => Point::new(0, 1),
        2 => Point::new(-1, 0),
        3 => Point::new(0, -1),
        _ => panic!(),
    };

    let mut trace_pos = match dir {
        0 => Point::new(0, new_pos.y),
        1 => Point::new(new_pos.x, 0),
        2 => Point::new(i64::try_from(GRID_WIDTH).unwrap() - 1, new_pos.y),
        3 => Point::new(new_pos.x, i64::try_from(GRID_HEIGHT).unwrap() - 1),
        _ => panic!(),
    };
    let mut prev_pos_in_trace = trace_pos;
    loop {
        match map[usize::try_from(trace_pos.y).unwrap()]
            [usize::try_from(trace_pos.x).unwrap()]
        {
            Tile::Floor => {
                let mut wraps = [Wrap::Unmapped; 4];
                wraps[usize::try_from(*dir).unwrap()] =
                    Wrap::Teleport(trace_pos.clone());
                map[usize::try_from(new_pos.y).unwrap()]
                    [usize::try_from(new_pos.x).unwrap()] = Tile::Wrap(wraps);
                println!("Add Tele: pos: {:?}, newpos: {:?}, tracepos: {:?}, prevposintrace: {:?}, ",*pos, new_pos, trace_pos, prev_pos_in_trace);

                *pos = trace_pos;
                break;
            }
            Tile::Wall => {
                let mut wraps = [Wrap::Unmapped; 4];
                wraps[usize::try_from(*dir).unwrap()] = Wrap::Wall;
                map[usize::try_from(new_pos.y).unwrap()]
                    [usize::try_from(new_pos.x).unwrap()] = Tile::Wrap(wraps);
                break;
            }
            Tile::Empty => {
                println!("Emptytraced: pos: {:?}, newpos: {:?}, tracepos: {:?}, prevposintrace: {:?}, ",*pos, new_pos, trace_pos, prev_pos_in_trace);
            }
            Tile::Wrap(wraps) => match wraps[usize::try_from(*dir).unwrap()] {
                Wrap::Teleport(p) => {
                    // I don't think you can get here.
                    panic!();
                    println!("Telefromtrace: pos: {:?}, newpos: {:?}, tracepos: {:?}, prevposintrace: {:?}, ",*pos, new_pos, trace_pos, prev_pos_in_trace);
                    *pos = p;
                    break;
                }
                Wrap::Wall => {
                    println!("wall");
                    break;
                }
                Wrap::Unmapped => {
                    // println!("Unmap: pos: {:?}, newpos: {:?}, tracepos: {:?}, prevposintrace: {:?}, ",*pos, new_pos, trace_pos, prev_pos_in_trace);
                    // dbg!(d);
                    // if not from floor, ignore
                    // dbg!(*pos, trace_pos, prev_pos_in_trace);

                    // dbg!(tile_at_position(map, prev_pos_in_trace));
                    if tile_at_position(map, prev_pos_in_trace) != Tile::Floor {
                    } else {
                        // I don't think you can get here.
                        panic!();
                    }
                    // trace_and_cache_wrap(pos, new_pos, dir, map);
                    // break;
                }
            },
            // Tile::Wrap(_wraps) => {
            //     // if not from floor, ignore
            //     if tile_at_position(map, *pos) != Tile::Floor {
            //         continue;
            //     }
            //     // I don't think you can get here.
            //     panic!();
            // }
        }

        prev_pos_in_trace = trace_pos;
        trace_pos.x = ((trace_pos.x + d.x)
            + i64::try_from(GRID_WIDTH).unwrap())
            % i64::try_from(GRID_WIDTH).unwrap();
        trace_pos.y = ((trace_pos.y + d.y)
            + i64::try_from(GRID_HEIGHT).unwrap())
            % i64::try_from(GRID_HEIGHT).unwrap();
    }
}

fn tile_at_position(
    map: &mut [[Tile; GRID_WIDTH as usize]; GRID_HEIGHT as usize],
    pos: Point,
) -> Tile {
    let wrapped_x =
        usize::try_from((pos.x + GRID_WIDTH as i64) % GRID_WIDTH as i64)
            .unwrap();
    let wrapped_y =
        usize::try_from((pos.y + GRID_HEIGHT as i64) % GRID_HEIGHT as i64)
            .unwrap();
    map[wrapped_y][wrapped_x]
}
