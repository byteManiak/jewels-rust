mod game;
mod engine;

use engine::{context::Context, assets::{AssetManager, u32_palette}};
use game::game::{Game, XRES, YRES};
use sdl2::{pixels::Color, rect::Rect};

fn main() -> Result<(), String> {
    let mut ctx = Context::create_ctx("Jewels!", XRES, YRES, true)?;
    let texture_creator = ctx.renderer.texture_creator();
    let mut asset_manager = AssetManager::new(
        &texture_creator,
        u32_palette(0x050500, 0x033822, 0x3e9533, 0xd4f044)
    );

    let mut game = Game::new()?;
    game.init(&mut asset_manager)?;

    loop {
        ctx.renderer.set_draw_color(Color::RGBA(0, 0, 0, 0));
        ctx.renderer.clear();
        ctx.renderer.set_draw_color(Color::RGBA(0xd5, 0xf0, 0x44, 0));
        ctx.renderer.fill_rect(Rect::new(0, 0, XRES, YRES))?;

        asset_manager.update_textures()?;

        if !game.update(&mut ctx, &asset_manager) {
            break;
        }
        ctx.renderer.present();
    }
    Ok(())
}
