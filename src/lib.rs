#![allow(unused)]
use std::collections::HashMap;
use std::env::join_paths;
use std::fmt::Debug;
use std::hash::Hash;
use std::str::Lines;

#[macro_export]
macro_rules! aoc_input {
    () => {{
        use std::path::PathBuf;
        let file_path = &PathBuf::from(file!());
        let day = file_path.file_stem().unwrap().to_str().unwrap();
        let s = format!("inputs/{}.in", &day[1..2]);
        std::fs::File::open(PathBuf::from(s)).unwrap()
    }};
}

#[macro_export]
macro_rules! aoc_ex {
    () => {{
        use std::path::PathBuf;
        let file_path = &PathBuf::from(file!());
        let day = file_path.file_stem().unwrap().to_str().unwrap();
        let s = format!("inputs/{}.ine", &day[1..2]);
        std::fs::File::open(PathBuf::from(s)).unwrap()
    }};
}

#[macro_export]
macro_rules! aoc_str {
    () => {{
        use std::io::Read;
        let mut contents = String::new();
        let mut file = $crate::aoc_input!();
        file.read_to_string(&mut contents).unwrap();
        contents
    }};
}

#[macro_export]
macro_rules! aoc_str_ex {
    () => {{
        use std::io::Read;
        let mut contents = String::new();
        let mut file = $crate::aoc_ex!();
        file.read_to_string(&mut contents).unwrap();
        contents
    }};
}
#[derive(Debug)]
pub struct CompleteGrid<C> {
    pub width: u32,
    pub height: u32,
    contents: Vec<C>,
}

impl<C: Default + Debug + Copy> CompleteGrid<C> {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            contents: vec![C::default(); (width * height) as usize],
        }
    }

    pub fn row(&self, row: usize) -> impl Iterator<Item = C> {
        if row > self.height as usize {
            panic!()
        }
        let start = row * self.width as usize;
        self.contents[start..start + self.width as usize]
            .iter()
            .copied()
    }
    pub fn col(&self, col: usize) -> impl Iterator<Item = C> {
        if col > self.height as usize {
            panic!()
        }
        self.contents[col..]
            .iter()
            .step_by(self.width as usize)
            .copied()
    }

    pub fn neighbours(&self, row:usize, col:usize) -> Vec<C>{
        let mut neigbours = vec![];
        for nrow in (row.saturating_sub(1)..=(row+1).clamp(0,self.height as usize-1)){
            for ncol in (col.saturating_sub(1)..=(col+1).clamp(0,self.width as usize-1)){
                if ncol == col && nrow == row  {
                    continue
                }
                    neigbours.push(*self.at(nrow, ncol));
            }
        }
        neigbours
    }

    fn row_col(&self, row: usize, col: usize) -> usize {
        row * self.width as usize + col
    }

    pub fn at(&self, row: usize, col: usize) -> &C {
        &self.contents[self.row_col(row, col)]
    }

    pub fn at_mut(&mut self, row: usize, col: usize) -> &mut C {
        let index = self.row_col(row, col);
        &mut self.contents[index]
    }

    pub fn from_grid_string<S: AsRef<str>>(input: S, f: fn(char) -> C) -> Self {
        let s = input.as_ref().trim();
        let width = s.lines().nth(0).unwrap().len();
        let height = s.lines().count();

        Self {
            width: width as u32,
            height: height as u32,
            contents: s.lines().map(str::chars).flatten()
                .filter(|c| c.is_ascii() || *c == ' ')
                .map(f)
                .collect(),
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Point<I> {
    x: I,
    y: I,
}

struct PointGrid<I, C> {
    grid: HashMap<Point<I>, C>,
}

impl<I: Eq + Hash, C> PointGrid<I, C> {
    pub fn new() -> Self {
        Self {
            grid: HashMap::new(),
        }
    }

    pub fn at(&self, point: Point<I>) -> &C {
        self.grid.get(&point).unwrap()
    }

    pub fn at_mut(&mut self, point: Point<I>) -> &mut C {
        self.grid.get_mut(&point).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_grid_col_can_be_collected() {
        let mut g = CompleteGrid::<u8>::new(3, 3);

        g.contents = Vec::from_iter(0..9);

        let col = g.col(0).collect::<Vec<_>>();
        assert_eq!(col, [0, 3, 6]);
        let col = g.col(1).collect::<Vec<_>>();
        assert_eq!(col, [1, 4, 7]);
        let col = g.col(2).collect::<Vec<_>>();
        assert_eq!(col, [2, 5, 8]);
    }

    #[test]
    fn a_grid_row_can_be_collected() {
        let mut g = CompleteGrid::<u8>::new(3, 3);
        g.contents = Vec::from_iter(0..9);

        let row = g.row(0).collect::<Vec<_>>();
        assert_eq!(row, [0, 1, 2]);
        let row = g.row(1).collect::<Vec<_>>();
        assert_eq!(row, [3, 4, 5]);
        let row = g.row(2).collect::<Vec<_>>();
        assert_eq!(row, [6, 7, 8]);
    }

    #[test]
    fn a_grid_can_be_parse_from_a_grid_string() {
        let gs = r##"
X X
 X 
X X
"##;
        let g = CompleteGrid::from_grid_string(gs, |c| match c {
            'X' => true,
            ' ' => false,
            _ => panic!(),
        });
        assert_eq!(g.contents, (0..9).map(|x| x & 1 == 0).collect::<Vec<_>>())
    }

    #[test]
    fn sparse_grid() {
        let x = PointGrid::<usize, u8>::new();
    }
}
