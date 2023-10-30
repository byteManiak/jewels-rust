mod game;
mod engine;

use engine::{context::Context, texture::TexManager};
use game::game::{Game};

fn main() -> Result<(), String> {
    let ctx = Context::create_ctx("bruh", 1024, 768, true)?;
    let tc = ctx.renderer.texture_creator();
    let mut tm = TexManager::new(&tc);

    let game = Game::new()?;

    game.load_assets(&mut tm)?;

    loop {
        if !game.update() {
            break;
        }
    }
    Ok(())
}
