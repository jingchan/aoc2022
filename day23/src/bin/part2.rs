use std::{
    collections::{BTreeSet, HashMap, HashSet},
    io,
};

type Point = utils::Point<i64>;

#[derive(Debug, Clone, Copy)]
struct Rect {
    top_left: Point,
    bottom_right: Point,
}

impl Rect {
    fn size(&self) -> i64 {
        return (self.bottom_right.x - self.top_left.x)
            * (self.bottom_right.y - self.top_left.y);
    }
    fn pointin(&self, p: Point) -> bool {
        (p.x >= self.top_left.x && p.x < self.bottom_right.x)
            && (p.y >= self.top_left.y && p.y < self.bottom_right.y)
    }
    fn center(&self) -> Point {
        Point::new(
            (self.bottom_right.x - self.top_left.x) / 2 + self.top_left.x,
            (self.bottom_right.y - self.top_left.y) / 2 + self.top_left.y,
        )
    }
}

#[derive(Clone)]
enum QuadTreeNode {
    Empty,
    Elves(Point),
    Next(QuadTree),
}

#[derive(Clone)]
pub struct QuadTree {
    rect: Rect,
    inner: [Box<QuadTreeNode>; 4],
}

impl QuadTree {
    pub fn len(&self) -> u32 {
        let mut cnt = 0;
        for i in 0..4 {
            match *self.inner[i] {
                QuadTreeNode::Empty => {}
                QuadTreeNode::Elves(e) => {
                    cnt += 1;
                }
                QuadTreeNode::Next(ref t) => {
                    cnt += t.len();
                }
            }
        }
        cnt
    }
    pub fn all_as_v(&self) -> Vec<Point> {
        let mut vec = Vec::new();
        for i in 0..4 {
            match *self.inner[i] {
                QuadTreeNode::Empty => {}
                QuadTreeNode::Elves(e) => {
                    vec.push(e);
                }
                QuadTreeNode::Next(ref t) => {
                    vec.append(&mut t.all_as_v());
                }
            }
        }
        return vec;
    }

    pub fn inspect_point(&mut self, p: Point) -> Option<Point> {
        let i = self.get_quadrant_for_point(p);
        // println!("Get quadtrant {:?},,  {:?} in rect: {:?}", i, p, self.rect);
        match *self.inner[i] {
            QuadTreeNode::Empty => None,
            QuadTreeNode::Elves(e) => {
                let x = e;
                Some(e)
            }
            QuadTreeNode::Next(ref mut t) => t.inspect_point(p),
        }
    }

    pub fn take_point(&mut self, p: Point) -> Option<Point> {
        let i = self.get_quadrant_for_point(p);
        // println!("Get quadtrant {:?},,  {:?} in rect: {:?}", i, p, self.rect);
        match *self.inner[i] {
            QuadTreeNode::Empty => None,
            QuadTreeNode::Elves(e) => {
                let x = e;
                self.inner[i] = Box::new(QuadTreeNode::Empty);
                Some(e)
            }
            QuadTreeNode::Next(ref mut t) => t.take_point(p),
        }
    }

    pub fn set_point(&mut self, p: Point) {
        let i = self.get_quadrant_for_point(p);
        match *self.inner[i] {
            QuadTreeNode::Empty => {
                if self.get_quadrant_size(i) == 1 {
                    self.inner[i] = Box::new(QuadTreeNode::Elves(p));
                } else {
                    let mut new = QuadTree {
                        rect: self.get_quadrant_rect(i),
                        inner: [
                            Box::new(QuadTreeNode::Empty),
                            Box::new(QuadTreeNode::Empty),
                            Box::new(QuadTreeNode::Empty),
                            Box::new(QuadTreeNode::Empty),
                        ],
                    };
                    new.set_point(p);
                    self.inner[i] = Box::new(QuadTreeNode::Next(new));
                }
            }
            QuadTreeNode::Elves(e) => {
                dbg!(p);
                panic!()
            }
            QuadTreeNode::Next(ref mut t) => {
                t.set_point(p);
            }
        }
    }

    fn get_quadrant_rect(&self, q: usize) -> Rect {
        let mid = self.rect.center();
        let r = match q {
            0 => Rect {
                top_left: self.rect.top_left,
                bottom_right: mid,
            },
            1 => Rect {
                top_left: Point::new(mid.x, self.rect.top_left.y),
                bottom_right: Point::new(self.rect.bottom_right.x, mid.y),
            },
            2 => Rect {
                top_left: mid,
                bottom_right: self.rect.bottom_right,
            },
            3 => Rect {
                top_left: Point::new(self.rect.top_left.x, mid.y),
                bottom_right: Point::new(mid.x, self.rect.bottom_right.y),
            },
            _ => unreachable!(),
        };
        return r;
    }

    fn get_quadrant_size(&self, q: usize) -> i64 {
        let mid = self.rect.center();

        let r = match q {
            0 => mid - self.rect.top_left,
            1 => Point::new(
                self.rect.bottom_right.x - mid.x,
                mid.y - self.rect.top_left.y,
            ),
            2 => self.rect.bottom_right - mid,
            3 => Point::new(
                mid.x - self.rect.top_left.x,
                self.rect.bottom_right.y - mid.y,
            ),
            _ => unreachable!(),
        };
        r.x * r.y
    }

    fn get_quadrant_for_point(&self, p: Point) -> usize {
        if !self.rect.pointin(p) {
            panic!();
        }
        let mid = self.rect.center();
        for i in 0..4 {
            if self.get_quadrant_rect(i).pointin(p) {
                return i;
            }
        }
        unreachable!()
    }

    // fn get_node_for_point(&mut self, p: Point) -> Box<QuadTreeNode> {
    //     let i = self.get_quadrant_for_point(p);
    //     self.inner[i]
    // }
}

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
                    // dbg!(x, y);
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
    let mut dormant_elves = HashSet::<Point>::new();

    let mut dormant = QuadTree {
        rect: Rect {
            top_left: Point::new(-1000, -1000),
            bottom_right: Point::new(1000, 1000),
        },
        inner: [
            Box::new(QuadTreeNode::Empty),
            Box::new(QuadTreeNode::Empty),
            Box::new(QuadTreeNode::Empty),
            Box::new(QuadTreeNode::Empty),
        ],
    };

    // Basiclaly a collision detection problem.
    // output to part1 looks wrong <- NM

    // Or we can remove from list until activated by anothe elf
    // this one seems easiest and best

    // Either we can use a AABB quad approach to find dormant quadrants
    // [day23/src/bin/part1.rs:148] minx = -6
    // [day23/src/bin/part1.rs:148] maxx = 76
    // [day23/src/bin/part1.rs:148] miny = -4
    // [day23/src/bin/part1.rs:148] maxy = 77

    // Or we can try to hack some active middle rectangle thing.
    // Try the hack first
    // Guessing this wont work.

    let dirs = [
        [Point::new(0, -1), Point::new(1, -1), Point::new(-1, -1)],
        [Point::new(0, 1), Point::new(1, 1), Point::new(-1, 1)],
        [Point::new(-1, 0), Point::new(-1, 1), Point::new(-1, -1)],
        [Point::new(1, 0), Point::new(1, -1), Point::new(1, 1)],
    ];
    // let mut r = 0;
    // loop {
    for r in 0..100000 {
        // println!("moving {} dormantj {}", elves.len(), dormant.len());
        // print_summ(&elves, &mut dormant);
        // println!("dormant {:?}", dormant.all_as_v());
        // println!(
        //     "dormant {:?}",
        //     elves.iter().cloned().collect::<Vec<Point>>()
        // );
        // r += 1;
        if r % 10 == 0 {
            println!("Round: {}", r);
        }
        proposals.clear();
        eproposals.clear();

        for e in &elves {
            let mut found = false;

            for d in 0..4 {
                let dir = dirs[(d + r) % 4];
                for dirp in dir {
                    // println!("checking: {:?}", &(*e + dirp));
                    if elves.get(&(*e + dirp)).is_some() {
                        // println!("Found elf at :{:?}", *e + dirp);

                        found = true;
                    }
                }
            }

            if !found {
                // println!("Going dormant at {:?}", e);
                dormant.set_point(*e);
                dormant_elves.insert(*e);
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
            if let Some(p) = eproposals.get(e) {
                if *proposals.get(p).unwrap() == 1 {
                    newpos.insert(*p);

                    let woken = wake_dormant(&mut dormant, *p);
                    for w in woken {
                        newpos.insert(w);
                        dormant_elves.remove(&w);
                    }

                    continue;
                }
            }

            if !dormant_elves.contains(e) {
                newpos.insert(*e);
            }
        }

        if newpos.is_empty() {
            println!("no moving elves on Round: {}", r + 1);
            break;
        }

        // print_summ(elves, &mut dormant);

        elves = newpos;
    }

    print_summ(&elves, &mut dormant);

    // N S W E

    Ok(())
}

fn wake_dormant(dormant: &mut QuadTree, p: Point) -> Vec<Point> {
    let mut v = Vec::new();
    // wake up any elves
    for i in -1..=1 {
        for j in -1..=1 {
            if let Some(e) = dormant.take_point(Point::new(p.x + i, p.y + j)) {
                // println!("Waking dormant at {:?}", e);
                v.push(e);
                v.append(&mut wake_dormant(dormant, e));
            }
        }
    }
    v
}

fn print_summ(elves: &BTreeSet<Point>, dormant: &mut QuadTree) {
    let mut minx = 999999;
    let mut maxx = -999999;
    let mut miny = 999999;
    let mut maxy = -999999;
    let mut count = 0;
    // dbg!(&elves);
    for e in elves {
        minx = minx.min(e.x);
        maxx = maxx.max(e.x);

        miny = miny.min(e.y);
        maxy = maxy.max(e.y);

        count += 1;
    }

    for e in dormant.all_as_v() {
        minx = minx.min(e.x);
        maxx = maxx.max(e.x);

        miny = miny.min(e.y);
        maxy = maxy.max(e.y);

        count += 1;
    }

    for y in miny..=maxy {
        for x in minx..=maxx {
            let p = Point::new(x, y);
            if elves.contains(&p) {
                print!("#");
            } else if dormant.inspect_point(p).is_some() {
                print!("D");
            } else {
                print!(".");
            }
        }
        println!();
    }
    dbg!(minx, maxx, miny, maxy, count);

    println!(
        "Score: {} from {}*{}-{}",
        (1 + maxx - minx) * (1 + maxy - miny) - count,
        (1 + maxx - minx),
        (1 + maxy - miny),
        count
    );
}
