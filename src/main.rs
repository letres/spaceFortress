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
}

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut screen = Vec::new();
    for x in 0..80 {
        for y in 0..60 {
            screen.push(Cell {
                posistion: Rect::new(x * 10, y * 10, 10, 10),
                texture_posistion: (x * y % 256) as u8,
                x: 8 as u8,
                y: 8 as u8,
                r: 255 as u8,
                g: 255 as u8,
                b: 255 as u8,
                on: false,
            });
        }
    }

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();

    let mut texture =
        texture_creator.load_texture(std::path::Path::new("./src/reasources/font1.png"))?;
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.copy(&texture, None, Some(Rect::new(100, 100, 256, 256)))?;
    canvas.copy_ex(
        &texture,
        None,
        Some(Rect::new(450, 100, 256, 256)),
        0.0,
        None,
        false,
        false,
    )?;
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
        i = (i + 1) % screen.len();
        rands = rng.gen();
        screen[i].set_texture(rands);
        screen[i].set_color(rng.gen(), rng.gen(), rng.gen());
        //change texture randomly
        canvas.clear();
        for cell in screen.iter() {
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

        let t1 = now.elapsed();
        let now = std::time::Instant::now();

        canvas.present();
        canvas.window_mut().set_title(&format!(
            "{:.4}s logic, {:.4}s present, {:.4}s overall",
            t1.as_secs_f32(),
            now.elapsed().as_secs_f32(),
            start.elapsed().as_secs_f32()
        ));
    }
    Ok(())
}
