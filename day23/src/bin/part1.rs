use std::{
    collections::{BTreeSet, HashMap, HashSet},
    io,
};

type Point = utils::Point<i64>;

fn main() -> io::Result<()> {
    // ....#..
    // ..###.#
    // #...#.#
    // .#...##
    // #.###..
    // ##.#.##
    // .#..#..

    let mut elves = BTreeSet::new();
    let mut grid = Vec::new();
    for (y, line) in io::stdin().lines().enumerate() {
        let mut linevec = Vec::new();
        let line = line?;
        let chars = line.chars();
        for (x, c) in chars.enumerate() {
            match c {
                '.' => linevec.push(0),
                '#' => {
                    dbg!(x, y);
                    elves.insert(Point::new(x as i64, y as i64));
                    linevec.push(1);
                }
                _ => panic!(),
            };
        }
        grid.push(linevec);
    }

    let mut proposals = HashMap::<Point, u32>::new();
    let mut eproposals = HashMap::<Point, Point>::new();
    let dirs = [
        [Point::new(0, -1), Point::new(1, -1), Point::new(-1, -1)],
        [Point::new(0, 1), Point::new(1, 1), Point::new(-1, 1)],
        [Point::new(-1, 0), Point::new(-1, 1), Point::new(-1, -1)],
        [Point::new(1, 0), Point::new(1, -1), Point::new(1, 1)],
    ];
    for r in 0..10 {
        let mut minx = 999999;
        let mut maxx = -999999;
        let mut miny = 999999;
        let mut maxy = -999999;
        let mut count = 0;
        // dbg!(&elves);
        for e in &elves {
            minx = minx.min(e.x);
            maxx = maxx.max(e.x);

            miny = miny.min(e.y);
            maxy = maxy.max(e.y);

            count += 1;
        }

        for y in miny..=maxy {
            for x in minx..=maxx {
                if elves.contains(&Point::new(x, y)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        proposals.clear();
        eproposals.clear();

        for e in &elves {
            let mut found = false;
            for d in 0..4 {
                let dir = dirs[(d + r) % 4];
                for dirp in dir {
                    if elves.get(&(*e + dirp)).is_some() {
                        found = true;
                    }
                }
            }
            if !found {
                continue;
            }

            for d in 0..4 {
                let dir = dirs[(d + r) % 4];

                let mut found = false;
                for dirp in dir {
                    if elves.get(&(*e + dirp)).is_some() {
                        found = true;
                    }
                }
                let topos = *e + dir[0];
                if !found {
                    let v = proposals.get(&topos).cloned().unwrap_or_default();
                    proposals.insert(topos, v + 1);
                    eproposals.insert(*e, topos);
                    break;
                }
            }
        }

        let mut newpos = BTreeSet::new();
        for e in &elves {
            if let Some(dest) = eproposals.get(e) {
                if *proposals.get(dest).unwrap() == 1 {
                    newpos.insert(*dest);
                    continue;
                }
            }
            newpos.insert(*e);
        }

        elves = newpos;
    }

    let mut minx = 999999;
    let mut maxx = -999999;
    let mut miny = 999999;
    let mut maxy = -999999;
    let mut count = 0;
    // dbg!(&elves);
    for e in &elves {
        minx = minx.min(e.x);
        maxx = maxx.max(e.x);

        miny = miny.min(e.y);
        maxy = maxy.max(e.y);

        count += 1;
    }

    for y in miny..=maxy {
        for x in minx..=maxx {
            if elves.contains(&Point::new(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    dbg!(minx, maxx, miny, maxy, count);

    println!("{}", (1 + maxx - minx) * (1 + maxy - miny) - count);

    // N S W E

    Ok(())
}
