use crate::go_core::*;

pub struct GoBoard {
    cells: Vec<Vec<CellState>>,
    pub turn: CellState,
    pub size: usize,
}

impl GoBoard {
    pub fn new(size: usize) -> GoBoard {
        let mut cells: Vec<Vec<CellState>> = Vec::new();
        for _ in 0..size {
            let mut row: Vec<CellState> = Vec::new();
            for _ in 0..size {
                row.push(CellState::None);
            }
            cells.push(row);
        }

        // let mut captures = Map

        GoBoard {
            cells,
            size,
            turn: CellState::Black,
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

        GoBoard { turn, size, cells }
    }

    pub fn reset(&mut self) {
        self.turn = CellState::Black;
        for j in 0..self.size {
            for i in 0..self.size {
                self.set(Point::new(i as i32, j as i32), CellState::None);
            }
        }
    }

    pub fn place(&mut self, p: Point) {
        if p.x as usize >= self.size || p.y as usize >= self.size {
            return;
        }

        if self.can_place(p) {
            //println!("Placing {} stone at {}", self.turn, p);
            self.set(p, self.turn);
            for q in self.find_captures(p) {
                //println!("removing {} stone at {}", self.get(q), q);
                self.set(q, CellState::None);
            }
            self.turn = match self.turn {
                CellState::Black => CellState::White,
                _ => CellState::Black,
            };
            //self.print();
            //println!();
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

    pub fn get_adjacent(&self, p: Point) -> Vec<Point> {
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

    pub fn can_place(&self, p: Point) -> bool {
        match self.get(p) {
            CellState::None => true,
            _ => false,
        }
    }

    pub fn print(&self) {
        for row in &self.cells {
            for col in row {
                let letter = match col {
                    CellState::White => 'W',
                    CellState::Black => 'B',
                    _ => '.',
                };
                print!("{}", letter);
            }
            println!();
        }
    }

    pub fn get(&self, p: Point) -> CellState {
        self.cells[p.y as usize][p.x as usize]
    }

    fn set(&mut self, p: Point, state: CellState) {
        self.cells[p.y as usize][p.x as usize] = state;
    }

    fn get_liberties(&self, p: Point) -> Vec<Point> {
        let mut liberties = Vec::new();
        let player = self.get(p);
        //println!("getting {}s liberties at {}", player, p);
        if player != CellState::None {
            let mut group: Vec<Point> = Vec::new();
            self.get_group(player, p, &mut group);
            //println!("the group has {} points", group.len());
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
        //println!("{} has {} liberties", player, liberties.len());
        liberties
    }

    pub fn get_group(&self, start: CellState, p: Point, group: &mut Vec<Point>) {
        //println!("getting group of {} at {}", start, p);
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

    pub fn count_liberties(&self, p: Point) -> usize {
        self.get_liberties(p).len()
    }
}
