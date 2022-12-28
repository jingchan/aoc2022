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

    // UDLRFB up down left right Forward Back
    //        -y +y    -x . +x .  +z .   -z

    // DDRRUULL
    // BB
    // UU
    // FF

    // - Cycles at 4
    // - Can join with adjacent diagonal
    // - To figure out the turn, just need to figure out which diagonal we are
    //     ending up on.
    // - There should only be 1 unique solution that folds into
    //     (externally-facing) cube
    //
    //    - In addition to normal connected movement between faces:
    //    - [Diagonals]: If adjacent exactly 1 adjacent square connects to
    //    the diagonal, we can single-turn (fold) into it.
    //    - [Knights-moves]: If connected to two squares in one direction, we can
    //    double turn.  (Anything connected 2 away will be the opposite side of
    //    the cube, so anything adjacent to that should also be adjacent to us.)
    //    - The above should cover all possibilities (when a side doesn't
    //    already exist).  Any unreachable points from here would be where the
    //    opposite side of the cube would be able to be placed.
    //
    //   D2  D3   SS: start
    //     SS     DN: dest  N corresponds with direction index if CW rotation
    //   D1  D0
    //
    //   - Clockwise -> Based on our direction (0: right, then CW), we will end
    //   up in the above diagonals, left turn, otherwise right turn

    //     XX
    // XXXXXX
    //     XXXX

    //   UURR          1122
    //   BB            33
    // LLDD          5544
    // FF            66

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
                (pos, dir) = do_moves(pos, dir, moves, &mut map);
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
    pos: Point,
    dir: u32,
    moves: u32,
    map: &mut [[Tile; GRID_WIDTH as usize]; GRID_HEIGHT as usize],
) -> (Point, u32) {
    let mut pos = pos;
    let mut dir = dir;
    for i in 0..moves {
        println!("At: {:?} Facing:{} Moves:{}", pos, dir, moves - i);
        let d = dir_num_to_point(dir);

        let (new_pos, new_dir) = if leaving_tile(pos, dir) {
            println!("------------Leaving Tile-------------------");
            let to = jump_to_face(pos, dir, map);
            println!("To: {:?}", to);
            to
        } else {
            (pos + d, dir)
        };

        dbg!(
            map[usize::try_from(new_pos.y).unwrap()]
                [usize::try_from(new_pos.x).unwrap()]
        );
        match map[usize::try_from(new_pos.y).unwrap()]
            [usize::try_from(new_pos.x).unwrap()]
        {
            Tile::Floor => {
                pos = new_pos;
                dir = new_dir;
            }
            Tile::Wall => {
                break;
            }
            _ => {
                dbg!(pos, dir);
                panic!();
            }
        }
    }
    (pos, dir)
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

/// Just hardcode this, annoying thing that we can figure it out later.
fn face_width() -> i64 {
    4
    // if GRID_WIDTH < 99 {
    //     4
    // } else {
    //     50
    // }
}

fn face_coord_from_map_coord(map_coord: Point) -> Point {
    let facex = map_coord.x / face_width();
    let facey = map_coord.y / face_width();

    Point::new(facex as i64, facey as i64)
}

fn jump_to_face(
    pos: Point,
    dir: u32,
    map: &[[Tile; GRID_WIDTH as usize]; GRID_HEIGHT as usize],
) -> (Point, u32) {
    let face_coord = face_coord_from_map_coord(pos);
    let d = match dir {
        0 => Point::new(1, 0),
        1 => Point::new(0, 1),
        2 => Point::new(-1, 0),
        3 => Point::new(0, -1),
        _ => panic!(),
    };

    if face_exists(face_coord + d, map) {
        // normal move
        return (pos + d, dir);
    }

    // CW diagonal move
    if face_exists(face_coord + dir_num_to_point(dir + 1), map) {
        let to_dir = dir_num_to_point(dir) + dir_num_to_point(dir + 1);
        let to_face = face_coord + to_dir;
        if face_exists(to_face, map) {
            let cur_face = face_coord_from_map_coord(pos);
            let next_face = face_coord_from_map_coord(pos + to_dir);

            let face_local = map_coord_to_wrapped_face_local_coord(pos);
            // Rotate CW
            let dest_face_local = rotate_cw_in_face_local(face_local);
            let dest_dir = (dir + 1) % 4;
            // add dir with wrapping
            let dest_local_pos = wrap_face_local_coord(
                dest_face_local + dir_num_to_point(dest_dir),
            );
            let dest_map_coord =
                face_local_coord_to_map_coord(dest_local_pos, to_face);
            return (dest_map_coord + dir_num_to_point(dest_dir), dest_dir);
        }
    }

    // CCW diagonal move
    if face_exists(face_coord + dir_num_to_point((dir - 1 + 4) % 4), map) {
        // ccw
        let to_dir =
            dir_num_to_point(dir) + dir_num_to_point((dir - 1 + 4) % 4);
        let to_face = face_coord + to_dir;
        if face_exists(to_face, map) {
            let cur_face = face_coord_from_map_coord(pos);
            let next_face = face_coord_from_map_coord(pos + to_dir);

            let face_local = map_coord_to_wrapped_face_local_coord(pos);
            // Rotate CCW
            let dest_face_local = rotate_ccw_in_face_local(face_local);
            let dest_dir = (dir - 1 + 4) % 4;
            // add dir with wrapping
            let dest_local_pos = wrap_face_local_coord(
                dest_face_local + dir_num_to_point(dest_dir),
            );
            let dest_map_coord =
                face_local_coord_to_map_coord(dest_local_pos, to_face);
            return (dest_map_coord + dir_num_to_point(dest_dir), dest_dir);
        }
    }

    // CW knights move
    if true
    // facing right: down down right
    //     face_exists(face_coord + dir_num_to_point(dir + 1), map)
    //     && face_exists(
    //         face_coord + dir_num_to_point(dir + 1) + dir_num_to_point(dir + 1),
    //         map,
    //     ) ||

    // face_exists(face_coord + dir_num_to_point(dir + 2), map)
    //     && face_exists(
    //         face_coord + dir_num_to_point(dir + 2) + dir_num_to_point(dir + 1),
    //         map,
    //     )
    {
        let to_dir = dir_num_to_point(dir)
            + dir_num_to_point(dir + 1)
            + dir_num_to_point(dir + 1);
        let to_face = face_coord + to_dir;
        if face_exists(to_face, map) {
            let cur_face = face_coord_from_map_coord(pos);
            let next_face = face_coord_from_map_coord(pos + to_dir);

            let face_local = map_coord_to_wrapped_face_local_coord(pos);
            // Rotate CW Twice
            let dest_face_local =
                rotate_cw_in_face_local(rotate_cw_in_face_local(face_local));
            let dest_dir = (dir + 2) % 4;
            // add dir with wrapping
            let dest_local_pos = wrap_face_local_coord(
                dest_face_local + dir_num_to_point(dest_dir),
            );
            let dest_map_coord =
                face_local_coord_to_map_coord(dest_local_pos, to_face);
            return (dest_map_coord + dir_num_to_point(dest_dir), dest_dir);
        }
    }
    // CCW knights move
    if true
    /*face_exists(face_coord + dir_num_to_point((dir - 1 + 4) % 4), map)
    && face_exists(
        face_coord
            + dir_num_to_point((dir - 1 + 4) % 4)
            + dir_num_to_point((dir - 1 + 4) % 4),
        map,
    )*/
    {
        let to_dir = dir_num_to_point(dir)
            + dir_num_to_point((dir - 1 + 4) % 4)
            + dir_num_to_point((dir - 1 + 4) % 4);
        let to_face = face_coord + to_dir;
        if face_exists(to_face, map) {
            let face_local = map_coord_to_wrapped_face_local_coord(pos);
            // Rotate CCW Twice
            let dest_face_local =
                rotate_ccw_in_face_local(rotate_ccw_in_face_local(face_local));
            let dest_dir = (dir - 2 + 4) % 4;
            // add dir with wrapping
            let dest_local_pos = wrap_face_local_coord(
                dest_face_local + dir_num_to_point(dest_dir),
            );

            let dest_map_coord =
                face_local_coord_to_map_coord(dest_local_pos, to_face);
            return (dest_map_coord + dir_num_to_point(dest_dir), dest_dir);
        }
    }
    // No more moves should be possible.
    panic!();
}

fn face_exists(
    face_coord: Point,
    map: &[[Tile; GRID_WIDTH as usize]; GRID_HEIGHT as usize],
) -> bool {
    let mapx = usize::try_from((face_coord.x + 4) % 4 * face_width()).unwrap();
    let mapy = usize::try_from((face_coord.y + 4) % 4 * face_width()).unwrap();

    map[mapy][mapx] != Tile::Empty
}

fn dir_num_to_point(dir: u32) -> Point {
    match dir {
        0 => Point::new(1, 0),
        1 => Point::new(0, 1),
        2 => Point::new(-1, 0),
        3 => Point::new(0, -1),
        _ => panic!(),
    }
}
fn map_coord_to_wrapped_face_local_coord(map_coord: Point) -> Point {
    Point::new(
        (map_coord.x + face_width()) % face_width(),
        (map_coord.y + face_width()) % face_width(),
    )
}

fn face_local_coord_to_map_coord(
    face_local_coord: Point,
    face_coord: Point,
) -> Point {
    wrap_map_coord(Point::new(
        face_coord.x * face_width() + face_local_coord.x,
        face_coord.y * face_width() + face_local_coord.y,
    ))
}

fn wrap_map_coord(coord: Point) -> Point {
    Point::new(
        (coord.x + GRID_WIDTH as i64) % GRID_WIDTH as i64,
        (coord.y + GRID_HEIGHT as i64) % GRID_HEIGHT as i64,
    )
}
fn wrap_face_local_coord(coord: Point) -> Point {
    Point::new(
        (coord.x + face_width() as i64) % face_width() as i64,
        (coord.y + face_width() as i64) % face_width() as i64,
    )
}
fn rotate_cw_in_face_local(p: Point) -> Point {
    Point::new(face_width() - 1 - p.y, p.x)
}
fn rotate_ccw_in_face_local(p: Point) -> Point {
    Point::new(p.y, p.x)
}

fn leaving_tile(pos: Point, dir: u32) -> bool {
    let pos = map_coord_to_wrapped_face_local_coord(pos);
    let new_pos = pos + dir_num_to_point(dir);

    new_pos.x < 0
        || new_pos.y < 0
        || new_pos.x >= face_width()
        || new_pos.y >= face_width()
}
