mod game;
mod engine;

use engine::{context::Context, assets::{AssetManager, u32_palette}};
use game::game::Game;
use sdl2::pixels::Color;

fn main() -> Result<(), String> {
    let mut ctx = Context::create_ctx("Jewels!", 1024, 768, true)?;
    let texture_creator = ctx.renderer.texture_creator();
    let mut asset_manager = AssetManager::new(
        &texture_creator,
        u32_palette(0x050500, 0x033822, 0x3e9533, 0xd4f044)?
    );

    let mut game = Game::new()?;
    game.init(&mut asset_manager)?;

    loop {
        ctx.renderer.set_draw_color(Color::RGBA(0xd4, 0xf0, 0x44, 0xFF));
        ctx.renderer.clear();
        if !game.update(&mut ctx, &asset_manager) {
            break;
        }
        ctx.renderer.present();
    }
    Ok(())
}
