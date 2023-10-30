use sdl2::{Sdl, render::WindowCanvas, mixer::{AUDIO_S16LSB, InitFlag, Sdl2MixerContext}};

pub struct Context {
    sdl: Sdl,
    pub renderer: WindowCanvas,
    pub audio: Sdl2MixerContext
}

impl Context {
    pub fn create_ctx(title: &str, width: u32, height: u32, int_scaling: bool) -> Result<Context, String> {
        let sdl = sdl2::init()?;

        sdl2::mixer::open_audio(44100, AUDIO_S16LSB, 2, 4096)?;
        let audio = sdl2::mixer::init(InitFlag::MP3)?;
        sdl2::mixer::allocate_channels(8);

        let mut window = sdl.video()?.window(title, width*3, height*3).build().map_err(|e| e.to_string())?;
        window.show();

        let mut renderer = window.into_canvas().accelerated().present_vsync().build().map_err(|e| e.to_string())?;

        if int_scaling {
            renderer.set_integer_scale(true)?;
            renderer.set_logical_size(width, height).map_err(|e| e.to_string())?;
        }

        Ok(Context {sdl, audio, renderer})
    }
}