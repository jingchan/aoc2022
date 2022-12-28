use std::{
    fmt::Debug,
    ops::{Add, Div, Mul, Sub},
};

#[derive(Clone)]
pub struct Grid<T = u32, U = u32> {
    inner: Vec<T>,
    pub width: U,
    pub height: U,
}

impl<T, U> Grid<T, U>
where
    T: Default + Clone + Copy + std::cmp::PartialEq,
{
    pub fn new(width: U, height: U) -> Self
    where
        U: Mul<Output = U> + Copy,
        usize: TryFrom<U>,
        <usize as TryFrom<U>>::Error: Debug,
    {
        Self {
            inner: vec![T::default(); usize::try_from(width * height).unwrap()],
            width,
            height,
        }
    }
    pub fn new_with_value(width: U, height: U, value: T) -> Self
    where
        U: Mul<Output = U> + Copy,
        usize: TryFrom<U>,
        <usize as TryFrom<U>>::Error: Debug,
    {
        Self {
            inner: vec![value; usize::try_from(width * height).unwrap()],
            width,
            height,
        }
    }

    pub fn get(&self, x: U, y: U) -> T
    where
        U: Mul<Output = U> + TryInto<usize> + Add<Output = U> + Copy,
        usize: TryFrom<U>,
        <usize as TryFrom<U>>::Error: Debug,
    {
        self.inner[usize::try_from(y * self.width + x).unwrap()]
    }

    pub fn get_at_point(&self, point: Point<U>) -> T
    where
        U: Mul<Output = U> + Add<Output = U> + Copy,
        usize: TryFrom<U>,
        <usize as TryFrom<U>>::Error: Debug,
    {
        self.get(point.x, point.y)
    }

    pub fn set(&mut self, x: U, y: U, val: T)
    where
        U: Mul<Output = U> + Add<Output = U> + Copy,
        usize: TryFrom<U>,
        <usize as TryFrom<U>>::Error: Debug,
    {
        self.inner[usize::try_from(y * self.width + x).unwrap()] = val;
    }

    pub fn set_at_point(&mut self, point: Point<U>, val: T)
    where
        U: Mul<Output = U> + Add<Output = U> + Copy,
        usize: TryFrom<U>,
        <usize as TryFrom<U>>::Error: Debug,
    {
        self.set(
            U::try_from(point.x).unwrap(),
            U::try_from(point.y).unwrap(),
            val,
        )
    }

    pub fn check_in_bounds(&self, x: U, y: U) -> bool
    where
        U: PartialOrd,
        usize: TryFrom<U>,
        <usize as TryFrom<U>>::Error: Debug,
    {
        x < self.width
            && y < self.height
            && usize::try_from(x).is_ok()
            && usize::try_from(y).is_ok()
    }

    pub fn check_point_in_bounds(&self, p: Point<U>) -> bool
    where
        U: PartialOrd + Copy + Clone,
        usize: TryFrom<U>,
        <usize as TryFrom<U>>::Error: Debug,
    {
        self.check_in_bounds(p.x, p.y)
    }
}

impl<T> std::fmt::Debug for Grid<T, u32>
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

#[derive(Hash, PartialEq, PartialOrd, Ord, Eq, Clone, Copy, Debug)]
pub struct Point<T>
where
    T: Copy + Clone,
{
    pub x: T,
    pub y: T,
}

// pub struct Point<T = u32> {
//     pub x: T,
//     pub y: T,
// }

impl<T> Point<T>
where
    T: Copy + Clone,
{
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> std::ops::Add for Point<T>
where
    T: Add<Output = T> + std::marker::Copy,
{
    type Output = Point<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> std::ops::Sub for Point<T>
where
    T: Sub<Output = T> + std::marker::Copy,
{
    type Output = Point<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
