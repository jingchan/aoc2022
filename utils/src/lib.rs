#[derive(Clone)]
pub struct Grid<T> {
    inner: Vec<T>,
    pub width: u32,
    pub height: u32,
}

impl<T> Grid<T>
where
    T: Default + Clone + Copy + std::cmp::PartialEq,
{
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            inner: vec![T::default(); (width * height) as usize],
            width,
            height,
        }
    }
    pub fn new_with_value(width: u32, height: u32, value: T) -> Self {
        Self {
            inner: vec![value; (width * height) as usize],
            width,
            height,
        }
    }

    pub fn get(&self, x: u32, y: u32) -> T {
        self.inner[usize::try_from(y * self.width + x).unwrap()]
    }

    pub fn get_at_point(&self, point: Point) -> T {
        self.get(
            u32::try_from(point.x).unwrap(),
            u32::try_from(point.y).unwrap(),
        )
    }

    pub fn set(&mut self, x: u32, y: u32, val: T) {
        self.inner[usize::try_from(y * self.width + x).unwrap()] = val;
    }

    pub fn set_at_point(&mut self, point: Point, val: T) {
        self.set(
            u32::try_from(point.x).unwrap(),
            u32::try_from(point.y).unwrap(),
            val,
        )
    }

    pub fn check_in_bounds(&self, x: i64, y: i64) -> bool {
        x >= 0
            && y >= 0
            && x < i64::try_from(self.width).unwrap()
            && y < i64::try_from(self.height).unwrap()
    }

    pub fn check_point_in_bounds(&self, p: Point) -> bool {
        self.check_in_bounds(
            i64::try_from(p.x).unwrap(),
            i64::try_from(p.y).unwrap(),
        )
    }

    pub fn draw_from(&mut self, from: Point, to: Point, val: T) {
        let diff = to - from;

        for i in 0..=i32::abs(diff.x) {
            let point = from + Point::new(i * i32::signum(diff.x), 0);
            self.set_at_point(point, val)
        }
        for i in 0..=i32::abs(diff.y) {
            let point = from + Point::new(0, i * i32::signum(diff.y));
            self.set_at_point(point, val)
        }
    }

    /// Returns true if sand stabilizes, otherwise false.
    pub fn spawn_sand(&mut self, origin: Point, val: T, floor: i32) -> bool {
        // if origin.y < 30 {
        //     dbg!(origin);
        // }

        let down = Point::new(0, 1);
        let left = Point::new(-1, 1);
        let right = Point::new(1, 1);

        for dir in [origin + down, origin + left, origin + right] {
            if self.get_at_point(dir) == T::default() && dir.y < floor {
                return self.spawn_sand(dir, val, floor);
            }
        }

        if origin.x == 500 && origin.y == 0 {
            return false;
        }
        self.set_at_point(origin, val);
        true
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
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn dirs() -> [Self; 4] {
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
