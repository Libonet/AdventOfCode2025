#![allow(unused_imports)]
use std::{cmp::Reverse, collections::{BinaryHeap, HashSet, VecDeque}, error::Error as ErrorTrait, fmt::Display, ops::{Add, AddAssign, Index, IndexMut, Sub, SubAssign}, slice::{Iter, IterMut}};


#[derive(Debug)]
pub enum Error {
    InvalidSize,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidSize => write!(f, "Given size doesn't match array size"),
        }
    }
}

impl ErrorTrait for Error {}

#[derive(Debug, Clone)]
pub struct Matrix<T> {
    vals: Vec<T>,
    rows: usize,
    cols: usize,
}

impl<T> Matrix<T> {
    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn iter(&self) -> Iter<'_, T> {
        self.vals.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        self.vals.iter_mut()
    }

    pub fn iter_pos(&self) -> std::iter::Zip<PosIter, Iter<'_, T>> {
        Pos::iter(self.rows, self.cols).zip(self.vals.iter())
    }

    pub fn iter_mut_pos(&mut self) -> std::iter::Zip<PosIter, IterMut<'_, T>> {
        Pos::iter(self.rows, self.cols).zip(self.vals.iter_mut())
    }

    pub fn get(&self, pos: Pos) -> Option<&T> {
        if pos.0 < 0 || pos.1 < 0 {
            return None;
        }

        let row = pos.0 as usize;
        let col = pos.1 as usize;
        if row >= self.rows || col >= self.cols {
            return None;
        }

        Some(&self.vals[row * self.cols + col])
    }

    pub fn get_mut(&mut self, pos: Pos) -> Option<&mut T> {
        if pos.0 < 0 || pos.1 < 0 {
            return None;
        }

        let row = pos.0 as usize;
        let col = pos.1 as usize;
        if row >= self.rows || col >= self.cols {
            return None;
        }

        Some(&mut self.vals[row * self.cols + col])
    }
}

impl TryFrom<String> for Matrix<char> {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let cols = match value.find('\n') {
            Some(pos) => pos,
            None => return Err("Couldn't split string into rows"),
        };
        let values: Vec<char> = value.replace("\n", "").chars().collect();
        
        match (values, cols).try_into() {
            Ok(matrix) => Ok(matrix),
            Err(err) => match err {
                Error::InvalidSize => Err("Couldn't create a matrix from a vector with this size"),
            },
        }
    }
}

impl<T> Matrix<T> {
    pub fn astar(
        &self,
        start: Pos,
        target: Pos,
        mut move_options: impl FnMut(&Self, &Pos) -> Vec<(Reverse<i64>, Pos)>,
        heuristic: impl Fn(&Pos, &Pos) -> i64,
    ) -> (Vec<Pos>, i64) {
        let mut visited = HashSet::new();
        visited.insert(start);

        let mut next_moves = BinaryHeap::with_capacity(heuristic(&start, &target) as usize);
        next_moves.extend(
            move_options(self, &start).into_iter()
                .map(|(cost, pos)| (Reverse(cost.0 + heuristic(&pos, &target)), pos, vec![start]))
        );

        while let Some((cost, pos, mut curr_path)) = next_moves.pop() {
            if pos == target { return (curr_path, cost.0); }
            if !visited.insert(pos) { continue; }

            curr_path.push(pos);
            next_moves.extend(
                move_options(self, &pos).into_iter()
                    .map(|(move_cost, pos)| (Reverse(move_cost.0 + cost.0 + heuristic(&pos, &target)), pos, curr_path.clone()))
            );
        }

        (Vec::new(), -1)
    }
}

impl<T: Default> Matrix<T> {
    pub fn new(rows: usize, cols: usize) -> Self {
        let mut vals = Vec::with_capacity(rows * cols);
        for _idx in 0..rows*cols {
            vals.push(T::default());
        }
        Self { vals, rows, cols }
    }
}

impl <T: Clone + PartialEq> Matrix<T> {
    pub fn with_default(rows: usize, cols: usize, def: T) -> Self {
        let mut vals = Vec::with_capacity(rows * cols);
        for _idx in 0..rows*cols {
            vals.push(def.clone());
        }
        Self { vals, rows, cols }
    }

    pub fn sq_floodfill(&mut self, start: Pos, remove: T, fill: T) {
        let mov_opts = [
            Pos(0,-1),
            Pos(0, 1),
            Pos(-1,0),
            Pos( 1,0),
        ];

        let mut visited = Matrix::with_default(self.rows, self.cols, false);

        let mut queue = Vec::new();
        queue.extend(mov_opts.map(|offset| start + offset));

        while let Some(pos) = queue.pop() {
            if let Some(visit) = visited.get(pos) {
                if *visit { continue; } else { visited[pos] = true }

                if self[pos] == remove {
                    self[pos] = fill.clone();
                    queue.extend(mov_opts.map(|offset| pos + offset));
                }
            }
        }
    }
}

impl<T> Index<Pos> for Matrix<T> {
    type Output = T;
    fn index(&self, index: Pos) -> &Self::Output {
        let (row, col) = index.try_into().unwrap();
        &self.vals[row * self.cols + col]
    }
}

impl<T> IndexMut<Pos> for Matrix<T> {
    fn index_mut(&mut self, index: Pos) -> &mut Self::Output {
        let (row, col) = index.try_into().unwrap();
        &mut self.vals[row * self.cols + col]
    }
}

impl<T> TryFrom<(Vec<T>, usize)> for Matrix<T> {
    type Error = Error;
    fn try_from(value: (Vec<T>, usize)) -> Result<Self, Self::Error> {
        let rows = value.0.len() / value.1;
        if value.0.len() == value.1 * rows {
            Ok(Self { vals: value.0, rows, cols: value.1 })
        } else {
            Err(Error::InvalidSize)
        }
    }
}

impl<T> IntoIterator for Matrix<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.vals.into_iter()
    }
}

impl<T: Clone + Display> Display for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.rows {
            let start = i*self.cols;
            let row: Vec<T> = self.vals[start..start+self.cols].to_vec();
            for item in row {
                write!(f, "{item} ")?;
            }
            writeln!(f)?
        }

        Ok(())
    }
}

pub fn manhattan_distance(from: &Pos, to: &Pos) -> i64 {
    to.0.abs_diff(from.0) as i64 + to.1.abs_diff(from.1) as i64
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pos(pub i32, pub i32);

impl Pos {
    pub fn iter(rows: usize, cols: usize) -> PosIter {
        PosIter::new(rows, cols)
    }
}

impl Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.0, self.1)
    }
}

impl Add<Pos> for Pos {
    type Output = Pos;

    fn add(self, rhs: Pos) -> Self::Output {
        Pos(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign<Pos> for Pos {
    fn add_assign(&mut self, rhs: Pos) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl Sub<Pos> for Pos {
    type Output = Pos;

    fn sub(self, rhs: Pos) -> Self::Output {
        Pos(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl SubAssign<Pos> for Pos {
    fn sub_assign(&mut self, rhs: Pos) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
    }
}

impl TryFrom<Pos> for (usize, usize) {
    type Error = &'static str;

    fn try_from(value: Pos) -> Result<Self, Self::Error> {
        if value.0 < 0 || value.1 < 0 {
            Err("Value is not positive")
        } else {
            Ok((value.0 as usize, value.1 as usize))
        }
    }
}

pub struct PosIter {
    rows: usize,
    cols: usize,
    curr: Option<Pos>,
}

impl PosIter {
    pub fn new(rows: usize, cols: usize) -> Self {
        let curr = if rows != 0 && cols != 0 {
            Some(Pos(0,0))
        } else {
            None
        };

        Self { rows, cols, curr }
    }
}

impl Iterator for PosIter {
    type Item = Pos;

    fn next(&mut self) -> Option<Self::Item> {
        let curr = self.curr?;

        let (row, col) = curr.try_into().unwrap();
        if col+1 < self.cols {
            self.curr = Some(curr + Pos(0,1));
        } else if row+1 < self.rows {
            self.curr = Some(curr + Pos(1, -((self.cols as i32) - 1)));
        } else {
            self.curr = None;
        }

        Some(curr)
    }
}

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
// pub struct UPos(pub u32, pub u32);


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_astar_2024_18_part1_example() {
        let coordinates = "\
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

        let mut matrix = Matrix::with_default(7, 7, '.');
        let corrupted = 12;
        for coord in coordinates.lines().take(corrupted) {
            let mut pair = coord.split(',');
            let col = pair.next().unwrap().parse().unwrap();
            let row = pair.next().unwrap().parse().unwrap();

            matrix[Pos(row, col)] = '#';
        }

        let (path, _) = matrix.astar(
            Pos(0,0), 
            Pos(6,6), 
            |mat, pos| {
                let pos = *pos;
                vec![
                    (Reverse(1), pos + Pos(0, 1)),
                    (Reverse(1), pos + Pos(0,-1)),
                    (Reverse(1), pos + Pos( 1,0)),
                    (Reverse(1), pos + Pos(-1,0)),
                ].into_iter().filter(|pair| {
                        mat.get(pair.1).is_some_and(|chr| *chr!='#')
                    })
                    .collect()
            },
            manhattan_distance,
        );

        assert_eq!(path.len(), 22);
    }
}
