use std::io;

#[derive(Clone)]
struct Grid<T> {
    inner: Vec<T>,
    pub width: u32,
    pub height: u32,
}

impl<T> Grid<T>
where
    T: Default + Clone + Copy,
{
    fn new(width: u32, height: u32) -> Self {
        Self {
            inner: vec![T::default(); (width * height) as usize],
            width,
            height,
        }
    }
    fn new_with_value(width: u32, height: u32, value: T) -> Self {
        Self {
            inner: vec![value; (width * height) as usize],
            width,
            height,
        }
    }

    fn get(&self, x: u32, y: u32) -> T {
        self.inner[usize::try_from(y * self.width + x).unwrap()]
    }

    fn get_at_point(&self, point: Point) -> T {
        self.get(
            u32::try_from(point.x).unwrap(),
            u32::try_from(point.y).unwrap(),
        )
    }

    fn set(&mut self, x: u32, y: u32, val: T) {
        self.inner[usize::try_from(y * self.width + x).unwrap()] = val;
    }

    fn set_at_point(&mut self, point: Point, val: T) {
        self.set(
            u32::try_from(point.x).unwrap(),
            u32::try_from(point.y).unwrap(),
            val,
        )
    }

    fn check_in_bounds(&self, x: i64, y: i64) -> bool {
        x >= 0
            && y >= 0
            && x < i64::try_from(self.width).unwrap()
            && y < i64::try_from(self.height).unwrap()
    }

    fn check_point_in_bounds(&self, p: Point) -> bool {
        self.check_in_bounds(
            i64::try_from(p.x).unwrap(),
            i64::try_from(p.y).unwrap(),
        )
    }
}

impl<T> std::fmt::Debug for Grid<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            f.write_fmt(format_args!("{}: ", y))?;
            let mut debug = f.debug_list();
            let row = &self.inner[(usize::try_from(y * self.width).unwrap())
                ..usize::try_from((y + 1) * (self.width)).unwrap()];
            debug.entries(row);
            // for x in 0..self.width {
            //     let z = &self.inner[(usize::try_from(y * self.width).unwrap())
            //         ..usize::try_from(y * self.width + x).unwrap()];
            //     debug.entries(z);
            // }
            debug.finish()?;
            f.write_str("\n")?;
        }
        Ok(())
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn dirs() -> [Self; 4] {
        [
            Point::new(-1, 0),
            Point::new(1, 0),
            Point::new(0, 1),
            Point::new(0, -1),
        ]
    }
}

impl std::ops::Add for Point {
    type Output = Point;
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub for Point {
    type Output = Point;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

fn main() -> io::Result<()> {
    let mut starts = Vec::new();
    let mut end = Point::new(0, 0);

    let mut input = Vec::new();
    for line in io::stdin().lines() {
        let line = line.unwrap();
        let mut inchars = Vec::new();
        for char in line.chars() {
            inchars.push(char);
        }
        input.push(inchars);
    }

    let height: u32 = input.len().try_into().unwrap();
    let width: u32 = input[0].len().try_into().unwrap();

    let mut heightmap = Grid::<u32>::new(width as u32, height as u32);

    for (y, row) in input.iter().enumerate() {
        for (x, char) in row.iter().enumerate() {
            let height = match char {
                'S' => {
                    starts.push(Point::new(x as i32, y as i32));
                    0
                }
                'E' => {
                    end = Point::new(x as i32, y as i32);
                    25
                }
                'a' => {
                    starts.push(Point::new(x as i32, y as i32));
                    0
                }
                c => u32::try_from(*c).unwrap() - u32::try_from('a').unwrap(),
            };

            heightmap.set(x as _, y as _, height.try_into().unwrap());
        }
    }

    println!("{:?}, {:?}", width, height);
    let mut path = Grid::<u32>::new_with_value(width, height, 99999);

    for start in starts {
        find_path(&heightmap, &mut path, start, 0);
    }
    let steps = path.get_at_point(end);

    println!("{}", steps);

    Ok(())
}

fn find_path(
    heights: &Grid<u32>,
    path: &mut Grid<u32>,
    cur: Point,
    steps: u32,
) {
    let last_visit_steps = path.get_at_point(cur);
    if last_visit_steps > steps {
        // println!("visiting {:?}: {:?} < {:?} ", cur, steps, last_visit_steps);
        path.set_at_point(cur, steps);

        for dir in Point::dirs() {
            let next = cur + dir;

            if heights.check_point_in_bounds(next) {
                // println!("{:?} in rect", next);
                if heights.get_at_point(next) <= heights.get_at_point(cur) + 1 {
                    // println!(
                    //     "{:?} <= {:?}",
                    //     heights.get_at_point(next),
                    //     heights.get_at_point(cur)
                    // );
                    find_path(heights, path, next, steps + 1)
                } else {
                    // println!(
                    //     "NOT {:?} ({:?}) <= {:?} ({:?})",
                    //     heights.get_at_point(next),
                    //     next,
                    //     heights.get_at_point(cur),
                    //     cur
                    // );
                }
            } else {
                // println!("{:?} not in rect", next);
            }
        }
    } else {
        // println!(
        //     "not visiting {:?}: {:?} < {:?} ",
        //     cur,
        //     steps, last_visit_steps
        // );
    }
}
