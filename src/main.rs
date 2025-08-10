use std::{env::args, fs, sync::Arc, process};
use log::{error, info};
use pixels::{Pixels, SurfaceTexture};
use winit::{application::ApplicationHandler, dpi::{LogicalSize}, event::{WindowEvent}, event_loop::EventLoop, window::{Window, WindowAttributes}};

use conway::{Config, World};

const WINDOW_SCALE_FACTOR: u32 = 8;

const WIDTH: u32 = 200;
const HEIGHT: u32 = 100;

fn main() {    
    env_logger::init();

    let args: Vec<String> = args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Error while parsing arguments: {err}");
        process::exit(1);
    });

    if config.save_frames {
        fs::create_dir("out").unwrap();
    }

    let event_loop = EventLoop::new().unwrap();
    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}

// fn save_image(frame: &[u8], n: usize) {
//     let buffer: ImageBuffer<Rgba<u8>, _> = ImageBuffer::from_raw(WIDTH, HEIGHT, frame.to_vec()).expect("Faild to create buffer");
//     buffer.save(format!("out/frame-{}.png", n)).expect("Failed to save image");
// }


#[derive(Default)]
struct App {
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'static>>,
    world: World
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        info!("Event: resumed");

        let attributes = WindowAttributes::default()
            .with_title("Conways Game Of Life")
            .with_inner_size(LogicalSize::new(WIDTH * WINDOW_SCALE_FACTOR, HEIGHT * WINDOW_SCALE_FACTOR));

        let window = Arc::new(event_loop.create_window(attributes).unwrap());
        let surface = SurfaceTexture::new(WIDTH, HEIGHT, window.clone());

        self.window = Some(window.clone());
        self.pixels = Some(Pixels::new(WIDTH, HEIGHT, surface).unwrap());
    }

    fn window_event(
            &mut self,
            event_loop: &winit::event_loop::ActiveEventLoop,
            _window_id: winit::window::WindowId,
            event: winit::event::WindowEvent,
        ) {
        
        match event {
            WindowEvent::CloseRequested => {
                info!("The close button was pressed!");
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
               // if self.world.iteration >= 1000 {
               //     event_loop.exit();
               // }
               
                self.world.update();
               
               // save_image(self.pixels.as_mut().unwrap().frame(), self.world.iteration);
               
                self.world.render(self.pixels.as_mut().unwrap().frame_mut());
                if let Err(err) = self.pixels.as_mut().unwrap().render() {
                    error!("pixels.render() {}", err);
                } else {
                   self.window.as_mut().unwrap().request_redraw();
                }
            },
            WindowEvent::Resized(size) => {
                if let Err(err) = self.pixels.as_mut().unwrap().resize_surface(size.width, size.height) {
                    error!("pixels.resize_surface {}", err);
                    event_loop.exit();
                }
            },
            _ => {}
        }
    }
}

