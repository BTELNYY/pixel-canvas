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
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    // lets create a test image
    let init = vec![0u8; width*height*4];
    
    let game = Game {
        pixels: init,
        context,
        tick: 0
    };

    game_loop(game, 60, 0.5, |mut g| {
        g.game.tick += 1;
        for (offset, pixel) in g.game.pixels.chunks_exact_mut(4).enumerate() {
            let id = ((offset + g.game.tick) % u32::MAX as usize) as u32;
            let px = id.to_le_bytes();
            pixel.copy_from_slice(&[px[0], px[1], px[2], 255]);

        }       
    }, |g| {
        let image_data = ImageData::new_with_u8_clamped_array(Clamped(g.game.pixels.as_slice()), width as u32).unwrap();
        g.game.context.put_image_data(&image_data, 0.0, 0.0).unwrap();
    });
}


const width: usize = 1024;
const height: usize = 1024;

struct Game {
    pixels: Vec<u8>,
    context: CanvasRenderingContext2d,
    tick: usize
}