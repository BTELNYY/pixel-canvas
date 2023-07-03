use std::{panic, error::Error};
use time::{OffsetDateTime, macros::format_description};
use wasm_bindgen::{prelude::*, Clamped};
use game_loop::game_loop;
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

fn setup_cool_logging_dot_jpg() -> Result<(), Box<dyn Error>> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            let now = OffsetDateTime::now_utc();
            out.finish(format_args!(
                "[{} {} {}] {}",
                now.format(format_description!("[month]-[day]-[year] [hour]:[minute]:[second]")).unwrap_or_else(|_| "now?".to_string()),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(fern::Output::call(console_log::log))
        .apply()?;
    Ok(())
}

use log::info;
use web_sys::{ImageData, CanvasRenderingContext2d};

fn main() {
    //alert("Hello, world!");
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    setup_cool_logging_dot_jpg().unwrap();
    info!("Hello, world!");
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("the-pixel-canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();
    canvas.set_width(WIDTH as u32);
    canvas.set_height(HEIGHT as u32);
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    let b = Box::new([0u8; WIDTH*HEIGHT*3]);
    let render = Box::new([0u8; WIDTH*HEIGHT*3]);
    // lets create a test image
    //let init = vec![0u8; WIDTH*HEIGHT*3];
    
    let game = Game {
        pixels: b,
        context,
        render,
        tick: 0
    };
    let mut frame_buffer = Box::new([0u8; WIDTH*HEIGHT*4]);
    game_loop(game, 60, 0.5, |g| {
        g.game.tick();
    }, move |g| {
        g.game.render();
        for (offset, pixel) in g.game.render.chunks_exact(3).enumerate() {
            let px = [pixel[0], pixel[1], pixel[2], 255];
            frame_buffer[offset*4..offset*4+4].copy_from_slice(&px);
        }
        let image_data = ImageData::new_with_u8_clamped_array(Clamped(frame_buffer.as_slice()), WIDTH as u32).unwrap();
        g.game.context.put_image_data(&image_data, 0.0, 0.0).unwrap();
    });
}


const WIDTH: usize = 1000;
const HEIGHT: usize = 1000;


impl Game {
    fn tick(&mut self) {
        self.tick += 1;
        for (offset, pixel) in self.pixels.chunks_exact_mut(3).enumerate() {
            let id = ((offset + self.tick) % u32::MAX as usize) as u32;
            let px = id.to_le_bytes();
            pixel.copy_from_slice(&[px[0], px[1], px[2]]);

        }       
    }
    fn render(&mut self) {
        self.render.copy_from_slice(&self.pixels[..]);
    }
}

struct Game {
    pixels: Box<[u8; WIDTH*HEIGHT*3]>,
    render: Box<[u8; WIDTH*HEIGHT*3]>,
    context: CanvasRenderingContext2d,
    tick: usize
}