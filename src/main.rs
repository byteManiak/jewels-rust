mod game;
mod engine;

use engine::{context::Context, texture::TexManager};
use game::game::{Game};
use sdl2::keyboard::Keycode;

fn main() -> Result<(), String> {
    let mut ctx = Context::create_ctx("bruh", 1024, 768, true)?;
    let tc = ctx.renderer.texture_creator();
    let mut tm = TexManager::new(&tc);

    let game = Game::new()?;

    game.load_assets(&mut tm)?;

    loop {
        ctx.update_events();

        if ctx.is_released(Keycode::Escape) {
            break;
        }
    }
    Ok(())
}
