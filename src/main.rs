mod go_core;
use bracket_lib::prelude::*;
use go_core::Point as GPoint;
use go_core::*;

const CONSOLE_SIMPLE: usize = 0;
const CONSOLE_BOARD: usize = 1;
const CONSOLE_STONES: usize = 2;
const CONSOLE_OVERLAY: usize = 3;

const SPR_CROSS: usize = 0;
const SPR_CROSS_DOT: usize = 1;
const SPR_SOUTH: usize = 2;
const SPR_NORTH: usize = 3;
const SPR_EAST: usize = 4;
const SPR_WEST: usize = 5;
const SPR_NORTHWEST: usize = 6;
const SPR_SOUTHWEST: usize = 7;
const SPR_SOUTHEAST: usize = 8;
const SPR_NORTHEAST: usize = 9;
const SPR_RED: usize = 10;
const SPR_BLUE: usize = 11;
//const SPR_GREEN: usize = 12;
const SPR_PINK: usize = 13;
const SPR_WHITE: usize = 14;
const SPR_BLACK: usize = 15;

struct State {
    game: Board,
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
        if ctx.control {
            self.game.allow_suicide = true;
        } else {
            self.game.allow_suicide = false;
        }

        ctx.set_active_console(CONSOLE_SIMPLE);
        ctx.cls_bg(RGBA::from_f32(1.0, 1.0, 1.0, 1.0));

        // if ctx.shift {
        //     for y in 0..self.game.get_size() {
        //         ctx.print_color(0, self.game.get_size() as i32 - y as i32 - 1, RGBA::from_f32(0.0, 0.0, 0.0, 1.0), RGBA::from_f32(1.0, 1.0, 1.0, 1.0), format!("{}", y + 1));
        //     }
        // }

        self.render_board(ctx);
        self.render_stones(ctx);
        self.render_ghost(ctx);
        self.render_overlay(ctx);
    }

    fn render_overlay(&self, ctx: &mut BTerm) {
        ctx.set_active_console(CONSOLE_OVERLAY);
        ctx.cls();
        let p = self.get_mouse_point(ctx);
        if (p.x as usize) < self.game.get_size() && (p.y as usize) < self.game.get_size() {
            let stone = self.game.get(p);
            if stone != CellState::None {
                let mut group = Vec::new();
                self.game.get_group(stone, p, &mut group);
                for q in group {
                    ctx.add_sprite(
                        Rect::with_size((q.x + 1) * 32, (q.y + 1) * 32, 32, 32),
                        0,
                        RGBA::from_f32(1.0, 1.0, 1.0, 1.0),
                        SPR_BLUE,
                    );
                }

                let liberties = self.game.get_liberties(p);
                for q in liberties {
                    ctx.add_sprite(
                        Rect::with_size((q.x + 1) * 32, (q.y + 1) * 32, 32, 32),
                        0,
                        RGBA::from_f32(1.0, 1.0, 1.0, 1.0),
                        SPR_RED,
                    );
                }
                // ctx.print(0, 0, format!("{} S, {} L", group.len(), liberties));
            } else {
                for b in self.game.get_territory(p) {
                    ctx.add_sprite(
                        Rect::with_size((b.x + 1) * 32, (b.y + 1) * 32, 32, 32),
                        0,
                        RGBA::from_f32(1.0, 1.0, 1.0, 1.0),
                        SPR_PINK,
                    );
                }
            }
        }
    }

    fn render_stones(&self, ctx: &mut BTerm) {
        ctx.set_active_console(CONSOLE_STONES);
        ctx.cls();
        for y in 0..self.game.get_size() {
            for x in 0..self.game.get_size() {
                let state = self.game.get(GPoint::new(x as i32, y as i32));
                if state != CellState::None {
                    let idx = match state {
                        CellState::Black => SPR_BLACK,
                        CellState::White => SPR_WHITE,
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
        ctx.set_active_console(CONSOLE_BOARD);
        ctx.cls();
        for y in 0..self.game.get_size() {
            for x in 0..self.game.get_size() {
                let mut idx: usize = SPR_CROSS;
                if x == 0 && y == 0 {
                    idx = SPR_NORTHWEST;
                } else if x == 0 && y == self.game.get_size() - 1 {
                    idx = SPR_SOUTHWEST;
                } else if x == self.game.get_size() - 1 && y == 0 {
                    idx = SPR_NORTHEAST;
                } else if x == self.game.get_size() - 1 && y == self.game.get_size() - 1 {
                    idx = SPR_SOUTHEAST;
                } else if x == 0 {
                    idx = SPR_WEST;
                } else if x == self.game.get_size() - 1 {
                    idx = SPR_EAST;
                } else if y == 0 {
                    idx = SPR_NORTH;
                } else if y == self.game.get_size() - 1 {
                    idx = SPR_SOUTH;
                } else {
                    if self.game.get_size() == 19 {
                        if (x == 3 || x == 9 || x == 15) && (y == 3 || y == 9 || y == 15) {
                            idx = SPR_CROSS_DOT;
                        }
                    } else if self.game.get_size() == 13 {
                        if ((x == 3 || x == 9) && (y == 3 || y == 9)) || (x == 6 && y == 6) {
                            idx = SPR_CROSS_DOT;
                        }
                    } else if self.game.get_size() == 9 {
                        if (x == 2 || x == 6) && (y == 2 || y == 6) {
                            idx = SPR_CROSS_DOT;
                        }
                    } else if self.game.get_size() == 5 {
                        if x == 2 && y == 2 {
                            idx = SPR_CROSS_DOT;
                        }
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

const BOARD_SIZE: usize = 13;
const DISPLAY_WIDTH: usize = (BOARD_SIZE + 2) * 32;
const DISPLAY_HEIGHT: usize = (BOARD_SIZE + 2) * 32;

fn main() -> BError {
    let board = Board::new(BOARD_SIZE);
    let context = BTermBuilder::simple(BOARD_SIZE + 2, BOARD_SIZE + 2)?
        .with_title(format!("Go {}x{}", BOARD_SIZE, BOARD_SIZE))
        .with_tile_dimensions(32, 32)
        .with_sprite_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, 0) // board
        .with_sprite_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, 0) // stones
        .with_sprite_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, 0) // overlay
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
