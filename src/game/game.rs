


use crate::engine::assets::{AssetManager, u32_palette};
use crate::engine::context::Context;
use crate::game::mainmenu::MainMenu;
use crate::game::board::Board;

pub const XRES: u32 = 160;
pub const YRES: u32 = 144;
pub const XCENTRE: i32 = XRES as i32/2;
pub const YCENTRE: i32 = YRES as i32/2;

pub struct Game {
    mainmenu: Option<MainMenu>,
    board: Option<Board>,
    in_menu: bool
}

impl Game {
    pub fn new() -> Result<Self, String> {

        Ok(Self {
            mainmenu: None,
            board: None,
            in_menu: true})
    }

    pub fn init(&mut self, manager: &mut AssetManager) -> Result<(), String> {
        manager.add_palette(u32_palette(0x210b1b, 0x3d223c, 0x8d655c, 0xbfab61));
        manager.add_palette(u32_palette(0x0d0405, 0x5e1210, 0xd35600, 0xfed018));
        manager.add_palette(u32_palette(0x000000, 0x552222, 0xa44444, 0xff7777));
        manager.add_palette(u32_palette(0x2c2137, 0x446176, 0x3fac95, 0xa1ef8c));
        manager.add_palette(u32_palette(0x141444, 0x1f1488, 0x4a24cc, 0x8f66ff));

        manager.load_texture("assets/jewel1.pcx", "gem1")?;
        manager.load_texture("assets/jewel2.pcx", "gem2")?;
        manager.load_texture("assets/jewel3.pcx", "gem3")?;
        manager.load_texture("assets/jewel4.pcx", "gem4")?;
        manager.load_texture("assets/jewel5.pcx", "gem5")?;
        manager.load_texture("assets/jewel6.pcx", "gem6")?;
        manager.load_texture("assets/arrows.pcx", "arrows")?;
        manager.load_texture("assets/logo.pcx", "logo")?;
        manager.load_texture("assets/barholder.pcx", "barholder")?;
        manager.load_texture("assets/bar.pcx", "bar")?;
        manager.load_texture("assets/bardesc.pcx", "bardesc")?;

        manager.update_textures()?;

        for f in 0..=7 {
            manager.load_sound(format!("assets/combo{:?}.wav", f).as_str(), format!("combo{:?}", f).as_str())?;
        }
        manager.load_sound("assets/intro.wav", "intro")?;
        manager.load_sound("assets/levelup.wav", "levelup")?;
        manager.load_sound("assets/gameover.wav", "gameover")?;

        manager.load_music("assets/music.wav");

        self.mainmenu = Some(MainMenu::new(manager));
        self.board = Some(Board::new());

        Ok(())
    }

    pub fn update(&mut self, ctx: &mut Context, manager: &mut AssetManager) -> bool{
        let mainmenu = self.mainmenu.as_mut().unwrap();
        let board = self.board.as_mut().unwrap();

        ctx.update_events();

        manager.begin_draw();

        manager.draw_rectangle(0, 0, XRES, YRES, 2, true);

        if self.in_menu {
            if mainmenu.update(&ctx.input, manager) {
                self.in_menu = false;
                board.load_game(manager);
                manager.play_music();
            }
        } else if board.update(&ctx.input, manager) {
            return false;
        }

        true
    }
}
