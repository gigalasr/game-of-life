use std::{env::args, fs, process, sync::Arc};
use image::{ImageBuffer, Rgba};
use log::{error, info};
use pixels::{Pixels, SurfaceTexture};
use winit::{application::ApplicationHandler, dpi::{LogicalSize}, event::{WindowEvent}, event_loop::EventLoop, window::{Window, WindowAttributes}};

use conway::{config::Config, World};

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
    let mut app = App::new(config);
    event_loop.run_app(&mut app).unwrap();
}

struct App {
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'static>>,
    world: World,
    config: Config,
    frame: u64
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        info!("Event: resumed");

        let attributes = WindowAttributes::default()
            .with_title("Conways Game Of Life")
            .with_inner_size(LogicalSize::new(self.config.width * self.config.scale, self.config.height * self.config.scale));

        let window = Arc::new(event_loop.create_window(attributes).unwrap());
        let surface = SurfaceTexture::new(self.config.width, self.config.height, window.clone());

        self.window = Some(window.clone());
        self.pixels = Some(Pixels::new(self.config.width, self.config.height, surface).unwrap());
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
                self.world.update();                
                self.world.render(self.pixels.as_mut().unwrap().frame_mut());

                if let Err(err) = self.pixels.as_mut().unwrap().render() {
                    error!("pixels.render() {}", err);
                } else {
                   self.window.as_mut().unwrap().request_redraw();
                }

                if self.config.save_frames {
                    self.save_current_frame();
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

impl App {
    fn new(config: Config) -> App {
        App {
            window: None,
            pixels: None,
            world: World::new(config.width as usize, config.height as usize),
            config: config,
            frame: 0
        }
    }

    fn save_current_frame(&mut self) {
        let buffer: ImageBuffer<Rgba<u8>, _> = ImageBuffer::from_raw(self.config.width, self.config.height, self.pixels.as_mut().unwrap().frame().to_vec()).expect("Faild to create buffer");
        buffer.save(format!("out/frame-{}.png", self.frame)).expect("Failed to save image");
        self.frame += 1;
    }
}