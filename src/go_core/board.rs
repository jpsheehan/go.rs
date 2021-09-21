use crate::go_core::*;

pub struct Board {
    cells: Vec<Vec<CellState>>,
    turn: CellState,
    size: usize,
    captured_stones: Vec<i32>,
    ko: Option<Point>,
    pub allow_suicide: bool,
    placement_cache: Vec<Option<bool>>,
}

impl Board {
    pub fn new(size: usize) -> Board {
        let mut cells: Vec<Vec<CellState>> = Vec::new();
        for _ in 0..size {
            let mut row: Vec<CellState> = Vec::new();
            for _ in 0..size {
                row.push(CellState::None);
            }
            cells.push(row);
        }

        let captured_stones = vec![0, 0, 0];
        let placement_cache = vec![Some(true); size * size];

        Board {
            cells,
            size,
            captured_stones,
            turn: CellState::Black,
            ko: None,
            allow_suicide: false,
            placement_cache,
        }
    }

    pub fn from_str(s: &str, turn: CellState) -> Option<Board> {
        let mut cells = Vec::new();
        let mut size: usize = 0;

        for line in s.trim().lines() {
            let mut row: Vec<CellState> = Vec::new();
            for c in line.trim().chars() {
                row.push(match c {
                    'W' | 'w' => CellState::White,
                    'B' | 'b' => CellState::Black,
                    _ => CellState::None,
                });
            }
            if size == 0 {
                size = row.len()
            } else {
                if size != row.len() {
                    return None;
                }
            }
            cells.push(row);
        }

        if cells.len() != size {
            return None;
        }

        let captured_stones = vec![0, 0, 0];
        let placement_cache = vec![None; size * size];

        Some(Board {
            turn,
            size,
            cells,
            captured_stones,
            ko: None,
            allow_suicide: false,
            placement_cache,
        })
    }

    pub fn get_size(&self) -> usize {
        return self.size;
    }

    pub fn get_turn(&self) -> CellState {
        return self.turn;
    }

    pub fn reset(&mut self) {
        self.turn = CellState::Black;
        self.captured_stones = vec![0, 0, 0];
        self.ko = None;
        self.placement_cache = vec![Some(true); self.size * self.size];
        for j in 0..self.size {
            for i in 0..self.size {
                self.set(Point::new(i as i32, j as i32), CellState::None);
            }
        }
    }

    pub fn place(&mut self, p: Point) {
        if self.can_place(p) {
            //println!("Placing {} stone at {}", self.turn, p);
            self.set(p, self.turn);
            let captured_stones = self.find_captured_stones(p);
            let num_captured_stones = captured_stones.len();
            for q in &captured_stones {
                //println!("removing {} stone at {}", self.get(q), q);
                let owner = self.get(*q);
                self.captured_stones[owner.get_other_player() as usize] += 1;
                self.set(*q, CellState::None);
            }
            if num_captured_stones == 1 && self.is_in_atari(p) {
                self.ko = Some(captured_stones[0]);
            } else {
                self.ko = None;
            }
            self.placement_cache = vec![None; self.size * self.size];
            self.turn = self.turn.get_other_player();
            //self.print();
            //println!();
        }
    }

    fn find_captured_stones(&self, p: Point) -> Vec<Point> {
        let target = self.turn.get_other_player();

        let mut captures = Vec::new();
        for q in self.get_adjacent(p) {
            if self.get(q) == target {
                if captures.iter().any(|&x| x == q) == false {
                    if self.count_liberties(q) == 0 {
                        captures.push(q);
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
        if p.x >= 1 {
            adjacent.push(p + Point::new(-1, 0));
        }
        if p.x as usize + 1 < self.size {
            adjacent.push(p + Point::new(1, 0));
        }
        if p.y >= 1 {
            adjacent.push(p + Point::new(0, -1));
        }
        if p.y as usize + 1 < self.size {
            adjacent.push(p + Point::new(0, 1));
        }
        adjacent
    }

    pub fn can_place(&mut self, p: Point) -> bool {
        if p.x as usize >= self.size || p.y as usize >= self.size {
            return false;
        }
        let cache_idx: usize = p.y as usize * self.size + p.x as usize;
        if let Some(cached) = self.placement_cache[cache_idx] {
            return cached;
        }

        if let Some(ko) = self.ko {
            if ko == p {
                self.placement_cache[cache_idx] = Some(false);
                return false;
            }
        }

        let can_place = match self.get(p) {
            CellState::None => {
                if self.allow_suicide {
                    true
                } else {
                    !self.is_move_suicidal(p)
                }
            }
            _ => false,
        };

        self.placement_cache[cache_idx] = Some(can_place);
        can_place
    }

    pub fn get_captured_stones(&self, p: CellState) -> i32 {
        return self.captured_stones[p as usize];
    }

    pub fn get(&self, p: Point) -> CellState {
        self.cells[p.y as usize][p.x as usize]
    }

    fn set(&mut self, p: Point, state: CellState) {
        self.cells[p.y as usize][p.x as usize] = state;
    }

    pub fn get_liberties(&self, p: Point) -> Vec<Point> {
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
        if self.get(p) == CellState::None {
            return;
        }
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

    pub fn is_in_atari(&self, p: Point) -> bool {
        self.count_liberties(p) == 1
    }

    fn is_move_suicidal(&self, p: Point) -> bool {
        let adjacents = self.get_adjacent(p);
        let other_player = self.turn.get_other_player();
        // attempt to find empty points or opponent's stones in atari
        // within the adjacent points
        for q in adjacents {
            let stone = self.get(q);
            if stone == CellState::None {
                return false;
            } else if stone == other_player && self.is_in_atari(q) {
                return false;
            } else if stone == self.turn && !self.is_in_atari(q) {
                return false;
            }
        }
        return true;
    }

    pub fn get_territory(&self, p: Point) -> Vec<Point> {
        let mut boundaries = Vec::new();
        if self.get(p) == CellState::None {
            boundaries.push(p);
            self._get_territory(p, &mut boundaries);
        }
        boundaries
    }

    fn _get_territory(&self, p: Point, boundaries: &mut Vec<Point>) {
        for q in self.get_adjacent(p) {
            if self.get(q) == CellState::None {
                if boundaries.iter().any(|&x| x == q) == false {
                    boundaries.push(q);
                    self._get_territory(q, boundaries);
                }
            }
        }
    }

    pub fn get_boundary(&self, p: Point) -> Vec<Point> {
        let territory = self.get_territory(p);
        let mut boundary = Vec::new();
        for t in &territory {
            if self
                .get_adjacent(*t)
                .iter()
                .all(|&x| territory.iter().any(|&y| y == x))
                == false
            {
                boundary.push(*t);
            }
        }
        boundary
    }
}
