#![feature(is_some_and)]
#![feature(map_many_mut)]
use core::panic;
use std::{
    array,
    collections::{HashMap, HashSet},
    io,
};
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

const GRID_WIDTH: u32 = 250;
const GRID_HEIGHT: u32 = 300;

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

    // NM im dumb
    let mut faces: HashMap<Point, [Option<(Point, u32)>; 4]> = HashMap::new();
    for i in 0..4 {
        for j in 0..4 {
            let face_coord = Point::new(i, j);
            if face_exists(face_coord, &map) {
                faces.insert(face_coord, [None; 4]);
            }
        }
    }

    faces = faces
        .iter()
        .map(|(k, v)| {
            (
                *k,
                std::array::from_fn(|i| {
                    if v[i].is_some() {
                        v[i]
                    } else {
                        let face_coord = *k + dir_num_to_point(i as u32);

                        if face_exists(face_coord, &map) {
                            Some((face_coord, 0))
                        } else {
                            None
                        }
                    }
                }),
            )
        })
        .collect();

    // if any faces are missing a link
    // for _ in 0..5 {

    // Any face that has a missing face but a connected diagonal
    // in the following form:
    //   XX
    //   XXXX
    // you know will be connected in the final cube.
    // If you repeatedly connect such arrangements you can determine all faces + rotations

    loop {
        if !faces.iter().any(|(_, v)| v.iter().any(|f| f.is_none())) {
            break;
        }
        let mut new_faces = faces.clone();
        let allfaces = faces.keys().cloned();
        allfaces.for_each(|k| {
            let connections = faces.get(&k).unwrap();

            for i in 0..4 {
                if connections[i].is_none() {
                    // println!("no connn from {:?} to dir {:?}", k, i);
                    let dir = i as u32;
                    let cwdir = (i as u32 + 1 + 4) % 4;
                    let ccwdir = (i as u32 + 4 - 1) % 4;

                    if let Some(sidec) = connections[cwdir as usize] {
                        let other_connection = faces.get(&sidec.0).unwrap();
                        if let Some(sideforwardc) =
                            other_connection[((dir + sidec.1) % 4) as usize]
                        {
                            let con = new_faces.get_mut(&k).unwrap();

                            con[dir as usize] = Some((
                                sideforwardc.0,
                                (sidec.1 + sideforwardc.1 + 1) % 4,
                            ));
                            if (k == Point::new(2, 2) && dir == 1) {
                                let chk = con[cwdir as usize];
                            }
                        }
                    }
                    if let Some(sidec) = connections[ccwdir as usize] {
                        let other_connection = faces.get(&sidec.0).unwrap();
                        if let Some(sideforwardc) =
                            other_connection[((dir + sidec.1) % 4) as usize]
                        {
                            let con = new_faces.get_mut(&k).unwrap();

                            con[dir as usize] = Some((
                                sideforwardc.0,
                                (sidec.1 + sideforwardc.1 + 3) % 4,
                            ));
                        }
                    }

                    // if let Some(c) = connections[ccwdir as usize] {
                    //     let conn2 = faces.get(&c.0).unwrap();
                    //     if let Some(c) = conn2[dir as usize] {
                    //         return Some((c.0, c.1 + 3));
                    //     }
                    // }
                }
            }
        });
        faces = new_faces;
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
                (pos, dir) = do_moves(pos, dir, moves, &mut map, &faces);
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
    faces: &HashMap<Point, [Option<(Point, u32)>; 4]>,
) -> (Point, u32) {
    let mut pos = pos;
    let mut dir = dir;
    for i in 0..moves {
        let d = dir_num_to_point(dir);

        let (new_pos, new_dir) = if leaving_tile(pos, dir) {
            let to = jump_to_face2(pos, dir, map, &faces);
            to
        } else {
            (pos + d, dir)
        };

        match map[usize::try_from(new_pos.y).unwrap()]
            [usize::try_from(new_pos.x).unwrap()]
        {
            Tile::Floor => {
                println!(
                    "Moving from P: {:?}, D: {:?} to P: {:?}, D: {:?} [FLOOR]",
                    pos, dir, new_pos, new_dir
                );
                pos = new_pos;
                dir = new_dir;
            }
            Tile::Wall => {
                println!(
                    "Moving from P: {:?}, D: {:?} to P: {:?}, D: {:?} [WALL]",
                    pos, dir, new_pos, new_dir
                );
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
    // for sample
    // 4
    // for real problem
    50
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

fn jump_to_face2(
    pos: Point,
    dir: u32,
    map: &[[Tile; GRID_WIDTH as usize]; GRID_HEIGHT as usize],
    faces: &HashMap<Point, [Option<(Point, u32)>; 4]>,
) -> (Point, u32) {
    let face_coord = face_coord_from_map_coord(pos);
    let d = dir_num_to_point(dir);

    let curface = faces.get(&face_coord).unwrap();
    let (to_face, rotations) = curface[dir as usize].unwrap();
    println!("pos{:?}", pos);
    let mut face_local = map_coord_to_wrapped_face_local_coord(pos);
    println!("Facelocal{:?}", face_local);

    // Rotate CW
    for i in 0..rotations {
        face_local = rotate_cw_in_face_local(face_local);
    }

    let dest_dir = (dir + rotations + 4) % 4;

    println!(
        "[Jump] From face: {:?} D: {:?} to face: {:?} D: {:?}",
        face_coord, dir, to_face, dest_dir
    );

    // add dir with wrapping
    let dest_local_pos =
        wrap_face_local_coord(face_local + dir_num_to_point(dest_dir));
    let dest_map_coord = face_local_coord_to_map_coord(dest_local_pos, to_face);
    return (dest_map_coord, dest_dir);
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
    let dir = dir % 4;
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
fn wrap_face_coord(coord: Point) -> Point {
    Point::new((coord.x + 4) % 4 as i64, (coord.y + 4) % 4 as i64)
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
    Point::new(p.y, face_width() - 1 - p.x)
}

fn leaving_tile(pos: Point, dir: u32) -> bool {
    let pos = map_coord_to_wrapped_face_local_coord(pos);
    let new_pos = pos + dir_num_to_point(dir);

    new_pos.x < 0
        || new_pos.y < 0
        || new_pos.x >= face_width()
        || new_pos.y >= face_width()
}
