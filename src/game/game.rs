use crate::engine::texture::TexManager;
use crate::game::mainmenu::MainMenu;
use crate::game::board::Board;

pub struct Game {
    mainmenu: MainMenu,
    board: Board,
    inMenu: bool
}

impl<'a> Game {
    pub fn new() -> Result<Self, String> {

        Ok(Self {
            mainmenu: MainMenu {  },
            board: Board { logoMoving: true, logoY: -24  },
            inMenu: true})
    }

    pub fn load_assets(&self, manager: &mut TexManager<'a>) -> Result<(), String> {
        manager.create_texture("assets/jewel1.pcx", "gem1")?;
        manager.create_texture("assets/jewel2.pcx", "gem2")?;
        manager.create_texture("assets/jewel3.pcx", "gem3")?;
        manager.create_texture("assets/jewel4.pcx", "gem4")?;
        manager.create_texture("assets/jewel5.pcx", "gem5")?;
        manager.create_texture("assets/jewel6.pcx", "gem6")?;
        manager.create_texture("assets/arrows.pcx", "arrows")?;
        manager.create_texture("assets/logo.pcx", "logo")?;
        manager.create_texture("assets/barholder.pcx", "barholder")?;
        manager.create_texture("assets/bar.pcx", "bar")?;
        manager.create_texture("assets/bardesc.pcx", "bardesc")?;

        Ok(())
    }

    pub fn update(&self) -> bool{
        false
    }
}