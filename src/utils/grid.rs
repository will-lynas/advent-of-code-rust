use std::{
    fmt::{
        self,
        Display,
        Formatter,
    },
    ops::{
        Index,
        IndexMut,
    },
    str::FromStr,
};

use super::point::Point;

#[derive(Clone)]
pub struct Grid<T> {
    pub width: i32,
    pub height: i32,
    pub body: Vec<T>,
}

impl FromStr for Grid<u8> {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes: Vec<_> = s.lines().map(str::as_bytes).collect();
        let height = bytes.len() as i32;
        let width = bytes[0].len() as i32;
        let mut body = Vec::with_capacity((width * height) as usize);
        for slice in bytes {
            body.extend_from_slice(slice);
        }
        Ok(Grid {
            width,
            height,
            body,
        })
    }
}

impl Grid<u8> {
    #[must_use]
    pub fn with_points(&self, points: impl IntoIterator<Item = Point>) -> Self {
        let mut grid = self.clone();
        points.into_iter().for_each(|point| grid[point] = b'#');
        grid
    }
}

impl Display for Grid<u8> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", self[Point::new(x, y)] as char)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T> Grid<T> {
    pub fn iter(&self) -> GridIter<T> {
        <&Self as IntoIterator>::into_iter(self)
    }

    pub fn contains(&self, point: Point) -> bool {
        point.y >= 0 && point.y < self.height && point.x >= 0 && point.x < self.width
    }

    pub fn point(&self, index: usize) -> Point {
        let x = (index as i32) % self.width;
        let y = (index as i32) / self.width;
        Point::new(x, y)
    }

    pub fn points(&self) -> impl Iterator<Item = Point> + '_ {
        self.inner_points(0)
    }

    pub fn inner_points(&self, n: i32) -> impl Iterator<Item = Point> + '_ {
        (n..self.height - n).flat_map(move |y| (n..self.width - n).map(move |x| Point::new(x, y)))
    }

    pub fn orthogonals(&self, point: Point) -> Vec<Point> {
        point
            .orthogonals()
            .into_iter()
            .filter(|point| self.contains(*point))
            .collect()
    }

    pub fn neighbors(&self, point: Point) -> Vec<Point> {
        point
            .neighbors()
            .into_iter()
            .filter(|point| self.contains(*point))
            .collect()
    }

    pub fn zero_grid(&self) -> Grid<usize> {
        Grid {
            width: self.width,
            height: self.height,
            body: vec![0; (self.width * self.height) as usize],
        }
    }
}

impl<T: Default + Clone> Grid<T> {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            width,
            height,
            body: vec![T::default(); (width * height) as usize],
        }
    }
}

impl<T: PartialEq> Grid<T> {
    pub fn find(&self, goal: &T) -> Option<Point> {
        self.body
            .iter()
            .position(|b| b == goal)
            .map(|index| self.point(index))
    }
}

impl<T> Index<Point> for Grid<T> {
    type Output = T;

    fn index(&self, point: Point) -> &Self::Output {
        &self.body[(self.width * point.y + point.x) as usize]
    }
}

impl<T> IndexMut<Point> for Grid<T> {
    fn index_mut(&mut self, point: Point) -> &mut Self::Output {
        &mut self.body[(self.width * point.y + point.x) as usize]
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.body[self.width as usize * y + x]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.body[self.width as usize * y + x]
    }
}

impl<'a, T> IntoIterator for &'a Grid<T> {
    type Item = (Point, &'a T);
    type IntoIter = GridIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self)
    }
}

pub struct GridIter<'a, T> {
    grid: &'a Grid<T>,
    index: usize,
}

impl<'a, T> GridIter<'a, T> {
    pub fn new(grid: &'a Grid<T>) -> Self {
        Self { grid, index: 0 }
    }
}

impl<'a, T> Iterator for GridIter<'a, T> {
    type Item = (Point, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        (self.index < self.grid.body.len()).then(|| {
            let point = self.grid.point(self.index);
            let value = &self.grid.body[self.index];
            self.index += 1;
            (point, value)
        })
    }
}
