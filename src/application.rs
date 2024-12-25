use crate::folder::FolderView;
use crate::Config;
use anyhow::{Context, Result};
use sdl2::{
    event::Event,
    image::LoadTexture,
    rect::Rect,
    render::{Canvas, Texture, TextureCreator},
    video::{Window, WindowContext},
    VideoSubsystem,
};
use std::{cell::RefCell, fs::DirEntry};

macro_rules! match_sdl2_keydown {
    ($key:ident) => {
        Event::KeyDown {
            keycode: Some(sdl2::keyboard::Keycode::$key),
            ..
        }
    };
}
pub struct Application {
    folder_view: RefCell<FolderView>,
    sdl_ctx: sdl2::Sdl,
    video_subsystem: VideoSubsystem,
    canvas: RefCell<Canvas<Window>>,
    texture_creator: TextureCreator<WindowContext>,
}

impl Application {
    pub fn from(config: Config) -> Result<Application> {
        let folder_view = FolderView::from(&config)?;

        let (sdl_ctx, vid_sub_sys) = Self::sdl2_init()?;

        let canvas = vid_sub_sys
            .window("image-viewer", 1920, 1080)
            .position_centered()
            .build()?
            .into_canvas()
            .accelerated()
            .build()?;

        let texture_creator = canvas.texture_creator();

        Ok(Application {
            folder_view: RefCell::new(folder_view),
            sdl_ctx,
            video_subsystem: vid_sub_sys,
            canvas: RefCell::new(canvas),
            texture_creator,
        })
    }

    // initialize SDL2, as well as its video subsystem
    // has to be successfully called before any other SDL2 functions
    fn sdl2_init() -> Result<(sdl2::Sdl, VideoSubsystem)> {
        let sdl_ctx = sdl2::init()
            .map_err(|e| anyhow::anyhow!(e))
            .context("Could not init SDL2.")?;

        let video_subsystem = sdl_ctx
            .video()
            .map_err(|e| anyhow::anyhow!(e))
            .context("Could not SDL2 video subystem")?;

        Ok((sdl_ctx, video_subsystem))
    }

    // get the current usable bounds of the screen
    fn get_current_usable_bounds(&self) -> Result<Rect> {
        let display_index = self
            .canvas
            .borrow()
            .window()
            .display_index()
            .map_err(|e| anyhow::anyhow!(e))?;
        self.video_subsystem
            .display_usable_bounds(display_index)
            .map_err(|e| anyhow::anyhow!(e))
    }

    // loads texture from file (represented by a DirEntry)
    fn load_texture(&self, direntry: &DirEntry) -> Result<Texture> {
        self.texture_creator
            .load_texture(direntry.path())
            .map_err(|e| anyhow::anyhow!(e))
            .with_context(|| format!("Texture loading failed: {:?}", direntry.path()))
    }

    fn display_texture(&self, t: &Texture) -> Result<()> {
        // texture dimensions
        let (t_height, t_width) = (t.query().height, t.query().width);
        
        // window dimensions
        let (max_w, max_h) = self
            .get_current_usable_bounds()
            .context("Could not determine usable bounds of screen.")?
            .size();

        // scale the texture to fit the window
        let (h, w) = if t_height > max_h || t_width > max_w {
            let h_ratio = max_h as f32 / t_height as f32;
            let w_ratio = max_w as f32 / t_width as f32;
            if h_ratio < w_ratio {
                (max_h, (t_width as f32 * h_ratio) as u32)
            } else {
                ((t_height as f32 * w_ratio) as u32, max_w)
            }
        } else {
            (t_height, t_width)
        };

        

        // resize the window to fit the texture
        self.canvas
            .borrow_mut()
            .window_mut()
            .set_size(w, h)
            .map_err(|e| anyhow::anyhow!(e))
            .context("Unable to resize window.")?;

        // draw the texture
        self.canvas
            .borrow_mut()
            .copy(t, None, None)
            .map_err(|e| anyhow::anyhow!(e))
            .context("Failed to update canvas.")?;

        // update the canvas
        self.canvas.borrow_mut().present();
        Ok(())
    }

    fn display_direntry(&self, direntry: &DirEntry) -> Result<()> {
        // load texture from file
        let texture = self.load_texture(direntry)?;
        // display texture on canvas
        self.display_texture(&texture)?;

        // try to determine filename
        let title = direntry
            .file_name()
            .into_string()
            .map_err(|_| anyhow::anyhow!("Could not convert filename to string"))?;

        // set window title to filename
        self.canvas
            .borrow_mut()
            .window_mut()
            .set_title(&title)
            .map_err(|e| anyhow::anyhow!(e))
    }

    pub fn run(&self) -> Result<()> {
        let mut event_pump = self.sdl_ctx.event_pump().map_err(|e| anyhow::anyhow!(e))?;
        
        // display initial image
        self.display_direntry(self.folder_view.borrow().cur())?;
        
        // main event loop
        'event_loop: loop {
            for event in event_pump.poll_iter() {
                match event {
                    match_sdl2_keydown!(Q) | Event::Quit { .. } => break 'event_loop,
                    match_sdl2_keydown!(LEFT) => {
                        self.display_direntry(self.folder_view.borrow_mut().next())?;
                    }
                    match_sdl2_keydown!(RIGHT) => {
                        self.display_direntry(self.folder_view.borrow_mut().prev())?;
                    }
                    _ => {}
                }
            }
        }

        Ok(())
    }
}
