mod go_core;

use std::path::Path;
use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::surface::Surface;

use go_core::Point as GPoint;
use go_core::*;

const BOARD_SIZE: u32 = 13;

enum SpriteSheet {
    Cross,
    CrossDot,
    South,
    North,

    East,
    West,
    Northwest,
    Southwest,

    Southeast,
    Northeast,
    _RedDot,
    _BlueDot,

    _GreenDot,
    _PinkDot,
    White,
    Black,
}
const SPRITESHEET_ROWS: i32 = 4;
const SPRITESHEET_COLS: i32 = 4;
const W: u32 = 32;
const H: u32 = 32;

fn create_board_texture(surface: &mut Surface, size: u32) -> Result<(), String> {
    let sprite_sheet = Surface::load_bmp(Path::new("resources/board_lines.bmp"))?;
    *surface = Surface::new(size * W, size * H, sprite_sheet.pixel_format_enum())?;

    for y in 0..size {
        for x in 0..size {
            let mut spr: SpriteSheet = SpriteSheet::Cross;
            if x == 0 && y == 0 {
                spr = SpriteSheet::Northwest;
            } else if x == 0 && y == size - 1 {
                spr = SpriteSheet::Southwest;
            } else if x == size - 1 && y == 0 {
                spr = SpriteSheet::Northeast;
            } else if x == size - 1 && y == size - 1 {
                spr = SpriteSheet::Southeast;
            } else if x == 0 {
                spr = SpriteSheet::West;
            } else if x == size - 1 {
                spr = SpriteSheet::East;
            } else if y == 0 {
                spr = SpriteSheet::North;
            } else if y == size - 1 {
                spr = SpriteSheet::South;
            } else if is_dotted(size, x, y) {
                spr = SpriteSheet::CrossDot;
            }
            let dst_rect = Rect::new((x * W) as i32, (y * H) as i32, W, H);
            blit_from_spritesheet(&sprite_sheet, surface, dst_rect, spr)?;
        }
    }

    Ok(())
}

fn is_dotted(size: u32, x: u32, y: u32) -> bool {
    if size == 9 {
        (x == 2 || x == 6) && (y == 2 || y == 6)
    } else {
        false
    }
}

fn blit_from_spritesheet(
    src: &Surface,
    dst: &mut Surface,
    dst_rect: Rect,
    spr: SpriteSheet,
) -> Result<(), String> {
    let idx = spr as i32;
    let src_x = (idx as i32 % SPRITESHEET_COLS) * W as i32;
    let src_y = (idx as i32 / SPRITESHEET_ROWS) * H as i32;
    let src_rect = Rect::new(src_x, src_y, W, H);
    src.blit(src_rect, dst, dst_rect)?;

    Ok(())
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window(
            &format!("Go {}x{}", BOARD_SIZE, BOARD_SIZE),
            (BOARD_SIZE + 2) * W,
            (BOARD_SIZE + 2) * H,
        )
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .accelerated()
        .build()
        .map_err(|e| e.to_string())?;

    let texture_creator = canvas.texture_creator();

    canvas.set_draw_color(Color::WHITE);

    let mut event_pump = sdl_context.event_pump()?;

    let mut board_surface: Surface = Surface::new(0, 0, sdl2::pixels::PixelFormatEnum::RGB565)?;
    create_board_texture(&mut board_surface, BOARD_SIZE)?;
    let board_texture = board_surface
        .as_texture(&texture_creator)
        .expect("Couldn't convert to texture");

    let sprite_sheet = Surface::load_bmp(Path::new("resources/board_lines.bmp"))?;
    let mut spr_white = Surface::new(32, 32, sprite_sheet.pixel_format_enum())?;
    blit_from_spritesheet(
        &sprite_sheet,
        &mut spr_white,
        Rect::new(0, 0, 32, 32),
        SpriteSheet::White,
    )?;
    let tex_white = spr_white
        .as_texture(&texture_creator)
        .expect("convert to texture");
    let mut tex_white_ghost = sdl2::render::Texture::from_surface(&spr_white, &texture_creator)
        .expect("convert to texture");
    tex_white_ghost.set_alpha_mod(160);

    let mut spr_black = Surface::new(32, 32, sprite_sheet.pixel_format_enum())?;
    blit_from_spritesheet(
        &sprite_sheet,
        &mut spr_black,
        Rect::new(0, 0, 32, 32),
        SpriteSheet::Black,
    )?;
    let tex_black = spr_black
        .as_texture(&texture_creator)
        .expect("convert to texture");
    let mut tex_black_ghost = sdl2::render::Texture::from_surface(&spr_black, &texture_creator)
        .expect("convert to texture");
    tex_black_ghost.set_alpha_mod(160);

    let mut game = Board::new(BOARD_SIZE as usize);

    let mut running = true;
    let mut mouse_pos = Point::new(0, 0);
    while running {
        let mut place_stone = false;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    running = false;
                }
                Event::MouseMotion { x, y, .. } => {
                    mouse_pos = Point::new(x, y);
                }
                Event::MouseButtonDown { .. } => {
                    place_stone = true;
                }
                _ => {}
            }
        }

        if place_stone {
            let x = mouse_pos.x() / W as i32 - 1;
            let y = mouse_pos.y() / H as i32 - 1;
            let p = GPoint::new(x, y);
            if game.can_place(p) {
                game.place(p);
            }
        }

        canvas.clear();

        // render the board
        canvas.copy_ex(
            &board_texture,
            Some(Rect::new(0, 0, BOARD_SIZE * 32, BOARD_SIZE * 32)),
            Some(Rect::new(32, 32, BOARD_SIZE * 32, BOARD_SIZE * 32)),
            0.0,
            None,
            false,
            false,
        )?;

        // render the stones
        for y in 0..BOARD_SIZE {
            for x in 0..BOARD_SIZE {
                let stone = match game.get(GPoint::new(x as i32, y as i32)) {
                    CellState::White => Some(&tex_white),
                    CellState::Black => Some(&tex_black),
                    _ => None,
                };
                if let Some(sprite) = stone {
                    canvas.copy_ex(
                        sprite,
                        Some(Rect::new(0, 0, W, H)),
                        Some(Rect::new((x as i32 + 1) * 32, (y as i32 + 1) * 32, W, H)),
                        0.0,
                        None,
                        false,
                        false,
                    )?;
                }
            }
        }

        // render the ghost stone
        let ghost_x = mouse_pos.x() / W as i32 - 1;
        let ghost_y = mouse_pos.y() / H as i32 - 1;
        if game.can_place(GPoint::new(ghost_x, ghost_y)) {
            if ghost_x >= 0
                && ghost_y >= 0
                && ghost_x < BOARD_SIZE as i32
                && ghost_y < BOARD_SIZE as i32
            {
                canvas.copy_ex(
                    if game.get_turn() == CellState::White {
                        &tex_white_ghost
                    } else {
                        &tex_black_ghost
                    },
                    Some(Rect::new(0, 0, W, H)),
                    Some(Rect::new(
                        (ghost_x + 1) * W as i32,
                        (ghost_y + 1) * H as i32,
                        W,
                        H,
                    )),
                    0.0,
                    None,
                    false,
                    false,
                )?;
            }
        }

        canvas.present();

        std::thread::sleep(Duration::from_millis(1000 / 30));
    }

    Ok(())
}
