mod go_core;
use go_core::*;
use go_core::Point as GPoint;
use bracket_lib::prelude::*;

struct State {
    game: GoBoard
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm)
    {
        self.render(ctx);
    }
}

impl State {
    pub fn render(&self, ctx: &mut BTerm) {
        ctx.cls();
        for j in 0..self.game.get_size() {
            for i in 0..self.game.get_size() {
                ctx.print(i, j, get_player_character(self.game.get(GPoint::new(i as i32, j as i32))));
            }
        }
    }
}

fn get_player_character(cell: CellState) -> String {
    match cell {
        CellState::Black => String::from("B"),
        CellState::White => String::from("W"),
        _ => String::from(" "),
    }
}

fn main() -> BError {
    let mut board = GoBoard::new(9);
    board.place(GPoint::new(2, 2));
    board.place(GPoint::new(2, 3));
    board.place(GPoint::new(2, 3));
    board.place(GPoint::new(3, 3));
    board.place(GPoint::new(4, 4));
    board.place(GPoint::new(1, 3));
    board.place(GPoint::new(5, 5));
    board.place(GPoint::new(2, 4));
    // board.print();
    let context = BTermBuilder::simple80x50()
        .with_title("Go")
        .build()?;
    main_loop(context, State{ game: board })
}
