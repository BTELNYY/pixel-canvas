use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
};
use game_loop::game_loop;
use std::{
    convert::Infallible,
    error::Error,
    ops::{Deref, DerefMut},
    panic,
};
use time::{macros::format_description, OffsetDateTime};
use wasm_bindgen::{prelude::*, Clamped};
use wasm_sockets::{ConnectionStatus, PollingClient};
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
                now.format(format_description!(
                    "[month]-[day]-[year] [hour]:[minute]:[second]"
                ))
                .unwrap_or_else(|_| "now?".to_string()),
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
use web_sys::{CanvasRenderingContext2d, HtmlElement, ImageData};

fn main() {
    //alert("Hello, world!");
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    setup_cool_logging_dot_jpg().unwrap();
    info!("Hello, world!");
    let document = web_sys::window().unwrap().document().unwrap();
    let location = document.location().unwrap();
    let origin = location.origin().unwrap();
    let protocol = location.protocol().unwrap();
    let domain = location.hostname().unwrap();
    let port = location.port().unwrap();
    info!("origin: {}", origin);
    info!("protocol: {}", protocol);
    info!("domain: {}", domain);
    info!("port: {}", port);
    let is_secure = match protocol.as_str() {
        "https:" => true,
        "http:" => false,
        _ => panic!("unknown protocol: {}", protocol),
    };
    let websocket_url = format!(
        "ws{}://{}{}/ws",
        if is_secure { "s" } else { "" },
        domain,
        if port.is_empty() {
            "".to_string()
        } else {
            format!(":{}", port)
        }
    );
    info!("Websocket url (for nerds): {}", websocket_url);
    let status_text = document
        .get_element_by_id("status-text")
        .unwrap()
        .dyn_into::<web_sys::HtmlElement>()
        .unwrap();
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
    let b = Box::new([0u8; WIDTH * HEIGHT * 3]);
    //let render = Box::new([0u8; WIDTH * HEIGHT * 3]);
    // lets create a test image
    //let init = vec![0u8; WIDTH*HEIGHT*3];
    let socket = wasm_sockets::PollingClient::new(&websocket_url).unwrap();
    let game = Game {
        pixels: b,
        context,
        render: RenderWrapper::new(),
        tick: 0,
        socket,
        status_text,
        socket_url: websocket_url,
    };
    let mut frame_buffer = Box::new([0u8; WIDTH * HEIGHT * 4]);
    game_loop(
        game,
        60,
        0.5,
        |g| {
            g.game.tick();
        },
        move |g| {
            g.game.render();
            for (offset, pixel) in g.game.render.chunks_exact(3).enumerate() {
                let px = [pixel[0], pixel[1], pixel[2], 255];
                frame_buffer[offset * 4..offset * 4 + 4].copy_from_slice(&px);
            }
            let image_data = ImageData::new_with_u8_clamped_array(
                Clamped(frame_buffer.as_slice()),
                WIDTH as u32,
            )
            .unwrap();
            g.game
                .context
                .put_image_data(&image_data, 0.0, 0.0)
                .unwrap();
        },
    );
}

const WIDTH: usize = 1000;
const HEIGHT: usize = 1000;

impl Game {
    fn tick(&mut self) {
        let status = self.socket.status();
        match status {
            ConnectionStatus::Connected => {
                self.status_text.set_inner_text("Connected");
                // handle events here
            }
            ConnectionStatus::Connecting => {
                self.status_text.set_inner_text("Connecting");
            }
            ConnectionStatus::Disconnected => {
                self.status_text.set_inner_text("Disconnected");
                self.socket = wasm_sockets::PollingClient::new(&self.socket_url).unwrap();
            }
            ConnectionStatus::Error => {
                self.status_text.set_inner_text("Error");
                // maybe try to reconnect?
                self.socket = wasm_sockets::PollingClient::new(&self.socket_url).unwrap();
            }
        }
        self.tick += 1;
        for (offset, pixel) in self.pixels.chunks_exact_mut(3).enumerate() {
            let id = ((offset + self.tick) % u32::MAX as usize) as u32;
            let px = id.to_le_bytes();
            pixel.copy_from_slice(&[px[0], px[1], px[2]]);
        }
    }
    fn render(&mut self) {
        self.render.copy_from_slice(&self.pixels[..]);
        Rectangle::new(Point::new(256, 256), Size::new(512, 512))
            .into_styled(PrimitiveStyle::with_stroke(Rgb888::BLACK, (self.tick as u32/4) % 16))
            .draw(&mut self.render)
            .unwrap();
    }
}

struct RenderWrapper {
    render: Box<[u8; WIDTH * HEIGHT * 3]>,
}

impl OriginDimensions for RenderWrapper {
    fn size(&self) -> Size {
        Size::new(WIDTH as u32, HEIGHT as u32)
    }
}

impl DrawTarget for RenderWrapper {
    type Color = Rgb888;
    type Error = Infallible;
    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for pixel in pixels {
            let offset = (pixel.0.y * WIDTH as i32 + pixel.0.x) as usize;
            let px = [pixel.1.r(), pixel.1.g(), pixel.1.b()];
            self.render[offset * 3..offset * 3 + 3].copy_from_slice(&px);
        }
        Ok(())
    }
}

impl RenderWrapper {
    fn new() -> Self {
        Self {
            render: Box::new([0u8; WIDTH * HEIGHT * 3]),
        }
    }
}

impl Deref for RenderWrapper {
    type Target = [u8; WIDTH * HEIGHT * 3];
    fn deref(&self) -> &Self::Target {
        &self.render
    }
}

impl DerefMut for RenderWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.render
    }
}

struct Game {
    pixels: Box<[u8; WIDTH * HEIGHT * 3]>,
    render: RenderWrapper,
    context: CanvasRenderingContext2d,
    tick: usize,
    socket: PollingClient,
    status_text: HtmlElement,
    socket_url: String,
}
