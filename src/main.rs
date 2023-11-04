mod game;
mod engine;

use engine::{context::Context, assets::{AssetManager, u32_palette}};
use game::game::{Game, XRES, YRES};
use sdl2::{pixels::Color, rect::Rect};

fn main() -> Result<(), String> {
    let mut ctx = Context::create_ctx("Jewels!", XRES, YRES, true)?;
    let texture_creator = ctx.renderer.borrow().texture_creator();
    let font_manager = sdl2::ttf::init().unwrap();
    let font = font_manager.load_font("assets/font.ttf", 8).unwrap();
    let mut asset_manager = AssetManager::new(
        &texture_creator, &ctx.renderer, &font,
        u32_palette(0x050500, 0x033822, 0x3e9533, 0xd4f044)
    );

    let mut game = Game::new()?;
    game.init(&mut asset_manager)?;

    loop {
        asset_manager.update_textures()?;

        if !game.update(&mut ctx, &mut asset_manager) {
            break;
        }

        ctx.renderer.borrow_mut().present();
    }
    Ok(())
}
