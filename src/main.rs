mod go_core;
use bracket_lib::prelude::*;
use go_core::Point as GPoint;
use go_core::*;

struct State {
    game: GoBoard,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        self.render(ctx);
    }
}

impl State {
    pub fn render(&mut self, ctx: &mut BTerm) {
        if ctx.left_click {
            let p = self.get_mouse_point(ctx);
            if self.game.can_place(p) {
                self.game.place(p);
            }
        }

        ctx.set_active_console(0);
        ctx.cls();
        self.render_board(ctx);
        self.render_stones(ctx);
        self.render_ghost(ctx);
    }

    fn render_stones(&self, ctx: &mut BTerm) {
        ctx.set_active_console(2);
        ctx.cls();
        for y in 0..self.game.get_size() {
            for x in 0..self.game.get_size() {
                let state = self.game.get(GPoint::new(x as i32, y as i32));
                if state != CellState::None {
                    let idx = match state {
                        CellState::Black => 15,
                        CellState::White => 14,
                        _ => 0,
                    };
                    ctx.add_sprite(
                        Rect::with_size((x + 1) * 32, (y + 1) * 32, 32, 32),
                        0,
                        RGBA::from_f32(1.0, 1.0, 1.0, 1.0),
                        idx,
                    );
                }
            }
        }
    }

    fn render_board(&self, ctx: &mut BTerm) {
        ctx.set_active_console(1);
        ctx.cls();
        for y in 0..self.game.get_size() {
            for x in 0..self.game.get_size() {
                let mut idx: usize = 0;
                if x == 0 && y == 0 {
                    idx = 6;
                } else if x == 0 && y == self.game.get_size() - 1 {
                    idx = 7;
                } else if x == self.game.get_size() - 1 && y == 0 {
                    idx = 9;
                } else if x == self.game.get_size() - 1 && y == self.game.get_size() - 1 {
                    idx = 8;
                } else if x == 0 {
                    idx = 5;
                } else if x == self.game.get_size() - 1 {
                    idx = 4;
                } else if y == 0 {
                    idx = 3;
                } else if y == self.game.get_size() - 1 {
                    idx = 2;
                } else {
                    if self.game.get_size() == 19 {
                        if ((x == 3 || x == 9 || x == 15) && y == 3)
                            || ((x == 3 || x == 9 || x == 15) && y == 9)
                            || ((x == 3 || x == 9 || x == 15) && y == 15)
                        {
                            idx = 1;
                        } else {
                            idx = 0;
                        }
                    } else {
                        idx = 0;
                    }
                }
                ctx.add_sprite(
                    Rect::with_size((x + 1) * 32, (y + 1) * 32, 32, 32),
                    0,
                    RGBA::from_f32(1.0, 1.0, 1.0, 1.0),
                    idx,
                );
            }
        }

        //for x in 0..self.game.get_size() + 2 {
        //    ctx.add_sprite(Rect::with_size(x * 32, 0, 32, 32), 0, RGBA::from_f32(1.0, 1.0, 1.0, 1.0), 13);
        //    ctx.add_sprite(Rect::with_size(x * 32, (self.game.get_size() + 1) * 32, 32, 32), 0, RGBA::from_f32(1.0, 1.0, 1.0, 1.0), 11);
        //}
        //for y in 0..self.game.get_size() + 2 {
        //    ctx.add_sprite(Rect::with_size(0, y * 32, 32, 32), 0, RGBA::from_f32(1.0, 1.0, 1.0, 1.0), 10);
        //    ctx.add_sprite(Rect::with_size((self.game.get_size() + 1)*32, y * 32, 32, 32), 0, RGBA::from_f32(1.0, 1.0, 1.0, 1.0), 12);
        //}
        //ctx.add_sprite(Rect::with_size(0, 0, 32, 32), 0, RGBA::from_f32(1.0, 1.0, 1.0, 1.0), 10);
    }

    fn get_mouse_point(&self, ctx: &BTerm) -> GPoint {
        let mx = ctx.mouse_pos.0 / 32 - 1;
        let my = ctx.mouse_pos.1 / 32 - 1;
        GPoint::new(mx, my)
    }

    fn render_ghost(&self, ctx: &mut BTerm) {
        let p = self.get_mouse_point(ctx);
        if self.game.can_place(p) {
            let idx = match self.game.get_turn() {
                CellState::Black => 15,
                CellState::White => 14,
                _ => 0,
            };
            ctx.add_sprite(
                Rect::with_size((p.x + 1) * 32, (p.y + 1) * 32, 32, 32),
                0,
                RGBA::from_f32(1.0, 1.0, 1.0, 0.7),
                idx,
            );
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

const DISPLAY_WIDTH: usize = 672;
const DISPLAY_HEIGHT: usize = 672;

fn main() -> BError {
    let mut board = GoBoard::new(19);
    board.place(GPoint::new(2, 2));
    board.place(GPoint::new(2, 3));
    board.place(GPoint::new(2, 3));
    board.place(GPoint::new(3, 3));
    board.place(GPoint::new(4, 4));
    board.place(GPoint::new(1, 3));
    board.place(GPoint::new(5, 5));
    board.place(GPoint::new(2, 4));
    // board.print();
    let context = BTermBuilder::simple(21, 21)?
        .with_title("Go")
        .with_tile_dimensions(32, 32)
        //.with_resource_path("resources/")
        //.with_font("terminal8x8.png", 8, 8)
        .with_sprite_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, 0)
        .with_sprite_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, 0)
        //.with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "terminal8x8.png")
        .with_sprite_sheet(
            SpriteSheet::new("resources/board_lines.png")
                .add_sprite(Rect::with_size(0, 96, 32, 32))
                .add_sprite(Rect::with_size(32, 96, 32, 32))
                .add_sprite(Rect::with_size(64, 96, 32, 32))
                .add_sprite(Rect::with_size(96, 96, 32, 32))
                .add_sprite(Rect::with_size(0, 64, 32, 32))
                .add_sprite(Rect::with_size(32, 64, 32, 32))
                .add_sprite(Rect::with_size(64, 64, 32, 32))
                .add_sprite(Rect::with_size(96, 64, 32, 32))
                .add_sprite(Rect::with_size(0, 32, 32, 32))
                .add_sprite(Rect::with_size(32, 32, 32, 32))
                .add_sprite(Rect::with_size(64, 32, 32, 32))
                .add_sprite(Rect::with_size(96, 32, 32, 32))
                .add_sprite(Rect::with_size(0, 0, 32, 32))
                .add_sprite(Rect::with_size(32, 0, 32, 32))
                .add_sprite(Rect::with_size(64, 0, 32, 32))
                .add_sprite(Rect::with_size(96, 0, 32, 32)),
        )
        .build()?;
    main_loop(context, State { game: board })
}
