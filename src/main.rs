mod game;
mod engine;

use engine::{context::Context, assets::AssetManager};
use game::game::Game;
use sdl2::pixels::{Palette, Color};

fn main() -> Result<(), String> {
    let mut ctx = Context::create_ctx("Jewels!", 1024, 768, true)?;
    let texture_creator = ctx.renderer.texture_creator();
    let mut asset_manager = AssetManager::new(
        &texture_creator,
        Palette::with_colors(&[
            Color::RGBA(5, 5, 0, 0xFF),
            Color::RGBA(3, 0x38, 0x22, 0xFF),
            Color::RGBA(0x3e, 0x95, 0x33, 0xFF),
            Color::RGBA(0xd4, 0xf0, 0x44, 0xFF),
            Color::RGBA(0xFF, 0xFF, 0xFF, 0),

        ])?
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
