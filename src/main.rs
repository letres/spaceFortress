extern crate sdl2;

use rand::Rng;
use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use std::time::Duration;
//autosize for shit png/ascii
//something something chess maybe
//

#[derive(Copy, Clone)]
struct Cell {
    posistion: Rect,
    texture_posistion: u8,
    x: u8,
    y: u8,
    r: u8,
    g: u8,
    b: u8,
    back: sdl2::pixels::Color,
    on: bool,
}

impl Cell {
    fn texture(self) -> Rect {
        Rect::new(
            (self.texture_posistion as i32 % 16) * self.x as i32,
            (self.texture_posistion as i32 / 16) * self.y as i32,
            self.x as u32,
            self.y as u32,
        )
    }
    fn set_texture(&mut self, x: u8) {
        self.texture_posistion = x
    }
    fn pos(self) -> Rect {
        self.posistion
    }
    fn red(self) -> u8 {
        self.r
    }
    fn green(self) -> u8 {
        self.g
    }
    fn blue(self) -> u8 {
        self.b
    }
    fn set_color(&mut self, red: u8, green: u8, blue: u8) {
        self.r = red;
        self.g = green;
        self.b = blue;
    }
    fn background_color(self) -> sdl2::pixels::Color {
        self.back
    }
    fn set_background_color(&mut self, r: u8, b: u8, g: u8) {
        self.back = sdl2::pixels::Color::RGBA(r, g, b, 255)
    }
}

struct Panel {
    data: Vec<Cell>,
    size: (u16, u16),
    font: (u8, u8),
}

impl Panel {
    fn new(size: (u16, u16), font: (u8, u8)) -> Panel {
        let mut screen = Vec::new();
        for x in 0..size.0 {
            for y in 0..size.1 {
                screen.push(Cell {
                    posistion: Rect::new(
                        (x * font.0 as u16 ).into(),
                        (y * font.1 as u16 ).into(),
                        font.0 as u32,
                        font.1 as u32,
                    ),
                    texture_posistion: 0 as u8,
                    x: font.0 as u8,
                    y: font.1 as u8,
                    r: 255 as u8,
                    g: 255 as u8,
                    b: 255 as u8,
                    back: sdl2::pixels::Color::RGB(0, 0, 0),
                    on: false,
                });
            }
        }
        Panel {
            data: screen,
            size: size,
            font: font,
        }
    }
    fn panel(self) -> Vec<Cell> {
        self.data
    }
}

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("SpaceFortressFactoryTransportTycoon", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut screen = Panel::new((100, 75), (8, 8));

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let mut texture =
        texture_creator.load_texture(std::path::Path::new("./src/reasources/font1.png"))?;
    canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));
    canvas.copy(&texture, None, Some(Rect::new(100, 100, 256, 256)))?;
    canvas.present();
    canvas.clear();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    let mut rands: u8;
    let mut rng = rand::thread_rng();
    let mut prev = std::time::Instant::now();
    let start = std::time::Instant::now();
    'running: loop {
        let prev_t = prev.elapsed();
        let now = std::time::Instant::now();
        prev = now;
        rands = rng.gen();
        screen.data[i].set_texture(rands);
        screen.data[i].set_color(rng.gen(), rng.gen(), rng.gen());
        screen.data[i].set_background_color(rng.gen(), rng.gen(), rng.gen());
        //change texture randomly
        canvas.clear();
        for cell in screen.data.iter() {
            canvas.set_draw_color(cell.background_color());
            canvas.fill_rect(cell.pos());
        }
        //Render loop
        for cell in screen.data.iter() {
            texture.set_color_mod(cell.red(), cell.green(), cell.blue());
            canvas.copy(&texture, cell.texture(), cell.pos())?;
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        i = (i + 1) % screen.data.len();
        let t1 = now.elapsed();
        let now = std::time::Instant::now();

        canvas.present();
        canvas.window_mut().set_title(&format!(
            "{:.4}s logic, {:.4}fps, {:.4}s overall",
            t1.as_secs_f32(),
            (1.0 / now.elapsed().as_secs_f32()) as i32,
            start.elapsed().as_secs_f32()
        ));
    }
    Ok(())
}
