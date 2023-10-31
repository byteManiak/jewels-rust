mod game;
mod engine;

use engine::{context::Context, assets::AssetManager};
use game::game::Game;

fn main() -> Result<(), String> {
    let mut ctx = Context::create_ctx("Jewels!", 1024, 768, true)?;
    let texture_creator = ctx.renderer.texture_creator();
    let mut asset_manager = AssetManager::new(&texture_creator);

    let mut game = Game::new()?;
    game.init(&mut asset_manager)?;

    loop {
        if !game.update(&mut ctx, &asset_manager) {
            break;
        }
    }
    Ok(())
}
