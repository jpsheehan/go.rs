use std::ops::Add;
use core::fmt::Display;
use core::fmt;

#[derive(PartialEq, Copy, Clone, Debug)]
enum CellState
{
    None,
    White,
    Black
}

impl Display for CellState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            CellState::Black => "Black",
            CellState::White => "White",
            _ => "None",
        })
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Point
{
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point {
            x,
            y,
        }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

struct GoBoard
{
    cells: Vec<Vec<CellState>>,
    turn: CellState,
    size: usize
}

impl GoBoard
{
    pub fn new(size: usize) -> GoBoard {
        let mut cells: Vec<Vec<CellState>> = Vec::new();
        for _ in 0..size
        {
            let mut row: Vec<CellState> = Vec::new();
            for _ in 0..size
            {
                row.push(CellState::None);
            }
            cells.push(row);
        }

        // let mut captures = Map

        GoBoard {
            cells,
            size,
            turn: CellState::Black
        }
    }

    pub fn from_str(s: &str, turn: CellState) -> GoBoard {
        let mut cells = Vec::new();

        for line in s.trim().lines() {
            let mut row: Vec<CellState> = Vec::new();
            for c in line.trim().chars() {
                row.push(match c {
                    'W' | 'w' => CellState::White,
                    'B' | 'b' => CellState::Black,
                    _ => CellState::None,
                });
            }
            cells.push(row);
        }
        let size = cells.len() as usize;

        GoBoard {
            turn,
            size,
            cells,
        }
    }

    pub fn reset(&mut self)
    {
        self.turn = CellState::Black;
        for j in 0..self.size {
            for i in 0..self.size {
                self.set(Point::new(i as i32, j as i32), CellState::None);
            }
        }
    }

    pub fn place(self: &mut Self, p: Point)
    {
        if p.x as usize >= self.size || p.y as usize >= self.size {
            return;
        }

        if self.can_place(p) {
            println!("Placing {} stone at {}", self.turn, p);
            self.set(p, self.turn);
            for q in self.find_captures(p) {
                println!("removing {} stone at {}", self.get(q), q);
                self.set(q, CellState::None);
            }
            self.turn = match self.turn {
                CellState::Black => CellState::White,
                _ => CellState::Black
            };
            self.print();
            println!();
        }

    }

    fn find_captures(&self, p: Point) -> Vec<Point> {
        let target = match self.turn {
            CellState::Black => CellState::White,
            _ => CellState::Black,
        };

        let mut captures = Vec::new();
        for q in self.get_adjacent(p) {
            if self.get(q) == target {
                if captures.iter().any(|&x| x == q) == false {
                    if self.count_liberties(q) == 0 {
                        self.get_group(target, q, &mut captures);
                    }
                }
            }
        }

        if captures.len() == 0 {
            // suicide?
            if self.count_liberties(p) == 0 {
                self.get_group(self.turn, p, &mut captures);
            }
        }

        captures
    }

    fn get_adjacent(&self, p: Point) -> Vec<Point> {
        let mut adjacent = Vec::new();
        if p.x > 1 {
            adjacent.push(p + Point::new(-1, 0));
        }
        if p.x as usize + 1 < self.size {
            adjacent.push(p + Point::new(1, 0));
        }
        if p.y > 1 {
            adjacent.push(p + Point::new(0, -1));
        }
        if p.y as usize + 1 < self.size {
            adjacent.push(p + Point::new(0, 1));
        }
        adjacent
    }

    fn can_place(self: &Self, p: Point) -> bool
    {
        match self.get(p) {
            CellState::None => {
                true
            },
            _ => false
        }
    }

    pub fn print(self: &Self)
    {
        for row in &self.cells {
            for col in row {
                let letter = match col {
                    CellState::White => 'W',
                    CellState::Black => 'B',
                    _ => '.'
                };
                print!("{}", letter);
            }
            println!();
        }
    }

    fn get(self: &Self, p: Point) -> CellState
    {
        self.cells[p.y as usize][p.x as usize]
    }

    fn set(&mut self, p: Point, state: CellState) {
        self.cells[p.y as usize][p.x as usize] = state;
    }

    fn get_liberties(self: &Self, p: Point) -> Vec<Point>
    {
        let mut liberties = Vec::new();
        let player = self.get(p);
        println!("getting {}s liberties at {}", player, p);
        if player != CellState::None {
            let mut group: Vec<Point> = Vec::new();
            self.get_group(player, p, &mut group);
            println!("the group has {} points", group.len());
            for g in group {
                for q in self.get_adjacent(g) {
                    if self.get(q) == CellState::None {
                        if liberties.iter().any(|&x| x == q) == false {
                            liberties.push(q);
                        }
                    }
                }
            }
        }
        println!("{} has {} liberties", player, liberties.len());
        liberties
    }

    fn get_group(self: &Self, start: CellState, p: Point, group: &mut Vec<Point>)
    {
        println!("getting group of {} at {}", start, p);
        if group.len() == 0 {
            group.push(p);
        }
        for q in self.get_adjacent(p) {
            if self.get(q) == start {
                if group.iter().any(|&x| x == q) == false {
                    group.push(q);
                    self.get_group(start, q, group);
                }
            }
        }
    }

    fn count_liberties(self: &Self, p: Point) -> usize {
        self.get_liberties(p).len()
    }
}

mod tests {
    use crate::{Point,GoBoard,CellState};

    #[test]
    fn test_get_adjacent() {
        let mut b = GoBoard::new(19);

        b.place(Point::new(0, 0));
        assert_eq!(b.get_adjacent(Point::new(0, 0)).len(), 2);

        b.place(Point::new(0, 5));
        assert_eq!(b.get_adjacent(Point::new(0, 5)).len(), 3);

        b.place(Point::new(10, 10));
        assert_eq!(b.get_adjacent(Point::new(10, 10)).len(), 4);
    }

    #[test]
    fn test_get_group()
    {
        let mut b = GoBoard::new(19);
        let mut group = Vec::new();

        b.place(Point::new(5, 5));
        b.get_group(CellState::Black, Point::new(5, 5), &mut group);
        assert_eq!(group.len(), 1);

        b.turn = CellState::Black;
        group.clear();
        b.place(Point::new(5, 6));
        b.get_group(CellState::Black, Point::new(5, 5), &mut group);
        assert_eq!(group.len(), 2);
    }

    #[test]
    fn test_suicide()
    {
        let mut b = GoBoard::new(19);
        b.place(Point::new(1, 0));
        b.place(Point::new(5, 5));
        b.place(Point::new(0, 1));
        b.place(Point::new(0, 0));

        assert_eq!(b.get(Point::new(0, 0)), CellState::None)
    }

    #[test]
    fn test_capture()
    {
        let mut b = GoBoard::from_str("
        .....
        .BBB.
        BWWW.
        .BBB.
        .....
        ", CellState::Black);
        b.place(Point::new(4, 2));

        assert_eq!(b.get(Point::new(1, 2)), CellState::None);
        assert_eq!(b.get(Point::new(2, 2)), CellState::None);
        assert_eq!(b.get(Point::new(3, 2)), CellState::None);
    }
}

fn main() {
    let mut board = GoBoard::new(9);
    board.place(Point::new(2, 2));
    board.place(Point::new(2, 3));
    board.place(Point::new(2, 3));
    board.place(Point::new(3, 3));
    board.place(Point::new(4, 4));
    board.place(Point::new(1, 3));
    board.place(Point::new(5, 5));
    board.place(Point::new(2, 4));
}
