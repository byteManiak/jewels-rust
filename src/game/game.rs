use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

use crate::engine::assets::AssetManager;
use crate::engine::context::Context;
use crate::game::mainmenu::MainMenu;
use crate::game::board::Board;

pub struct Game {
    mainmenu: Option<MainMenu>,
    board: Option<Board>,
    in_menu: bool
}

impl<'a> Game {
    pub fn new() -> Result<Self, String> {

        Ok(Self {
            mainmenu: None,
            board: None,
            in_menu: true})
    }

    pub fn init(&mut self, manager: &mut AssetManager<'a>) -> Result<(), String> {
        manager.add_palette("Gold", &[
            Color::RGBA(0x21, 0xb, 0x1b, 0xFF),
            Color::RGBA(0x3d, 0x22, 0x3c, 0xFF),
            Color::RGBA(0x8d, 0x65, 0x5c, 0xFF),
            Color::RGBA(0xbf, 0xab, 0x61, 0xFF)
        ])?;
        manager.add_palette("Amber", &[
            Color::RGBA(0xd, 4, 5, 0xFF),
            Color::RGBA(0x5e, 0x12, 0x10, 0xFF),
            Color::RGBA(0xd3, 0x56, 0, 0xFF),
            Color::RGBA(0xfe, 0xd0, 0x18, 0xFF)
        ])?;
        manager.add_palette("Vboy", &[
            Color::RGBA(0, 0, 0, 0xFF),
            Color::RGBA(0x55, 0x22, 0x22, 0xFF),
            Color::RGBA(0xa4, 0x44, 0x44, 0xFF),
            Color::RGBA(0xff, 0x77, 0x77, 0xFF)
        ])?;
        manager.add_palette("Nymph", &[
            Color::RGBA(0x2c, 0x21, 0x37, 0xFF),
            Color::RGBA(0x44, 0x61, 0x76, 0xFF),
            Color::RGBA(0x3f, 0xac, 0x95, 0xFF),
            Color::RGBA(0xa1, 0xef, 0x8c, 0xFF)
        ])?;
        manager.add_palette("Blue", &[
            Color::RGBA(0x14, 0x14, 0x44, 0xFF),
            Color::RGBA(0x1f, 0x14, 0x88, 0xFF),
            Color::RGBA(0x4a, 0x24, 0xcc, 0xFF),
            Color::RGBA(0x8f, 0x66, 0xff, 0xFF)
        ])?;

        manager.set_palette("Vboy");
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

        for f in 0..=7 {
            manager.load_sound(format!("assets/combo{:?}.wav", f).as_str(), format!("combo{:?}", f).as_str())?;
        }
        manager.load_sound("assets/intro.wav", "intro")?;

        self.mainmenu = Some(MainMenu::new(manager));
        self.board = Some(Board {  });

        Ok(())
    }

    pub fn update(&mut self, ctx: &mut Context, manager: &AssetManager) -> bool{
        let mainmenu = self.mainmenu.as_mut().unwrap();
        let board = self.board.as_mut().unwrap();

        ctx.update_events();

        if self.in_menu {
            if mainmenu.update(&ctx.input, &manager) {
                self.in_menu = false;
                board.load_game();
            }
        } else {
            if board.update() {
                return false;
            }
        }

        if ctx.input.is_released(Keycode::Escape) {
            return false;
        }

        manager.draw_texture(&mut ctx.renderer, "gem1", 15, 15, 30, 30, 0, 0, 15, 15);

        true
    }
}