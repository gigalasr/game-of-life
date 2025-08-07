use std::{fmt::format, fs, sync::Arc};
use image::{ImageBuffer, Rgba};
use log::{error, info};
use noise::{NoiseFn, Perlin};
use pixels::{Pixels, SurfaceTexture};
use rand::Rng;
use winit::{application::ApplicationHandler, dpi::{LogicalSize}, event::{WindowEvent}, event_loop::EventLoop, window::{Window, WindowAttributes}};


const WIDTH: u32 = 100;
const HEIGHT: u32 = 100;
const WINDOW_SCALE_FACTOR: u32 = 8;

const ALIVE: u8 = 255;
const DEAD: u8 = 0;


fn main() {    
    env_logger::init();

    let _ = fs::create_dir("out");

    let event_loop = EventLoop::new().unwrap();
    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}


fn save_image(frame: &[u8], n: usize) {
    let buffer: ImageBuffer<Rgba<u8>, _> = ImageBuffer::from_raw(WIDTH, HEIGHT, frame.to_vec()).expect("Faild to create buffer");
    buffer.save(format!("out/frame-{}.png", n)).expect("Failed to save image");
}

struct World {
    cells: Vec<u8>,
    cells_buffer: Vec<u8>,
    iteration: usize
}

impl World {
    fn count_neighbours(&self, x: usize , y: usize) -> u8 {
        let mut count = 0;
        let directions = [
            (-1, -1), (0, -1), (1, -1),
            (-1,  0),          (1,  0),
            (-1,  1), (0,  1), (1,  1)
        ];

        for (dx, dy) in directions.iter() {
            let nx = ((x as isize + WIDTH as isize + dx) % WIDTH as isize) as usize;
            let ny = ((y as isize + HEIGHT as isize + dy) % HEIGHT as isize) as usize;
            if self.cells[nx + ny * WIDTH as usize] == ALIVE {
                count += 1;
            }
        }

        count
    }

    fn cool(&mut self) {
        for cell in self.cells.iter_mut() {
            if *cell != ALIVE {
                *cell = 0;
            }
        }    
    }

    fn to_index(&self, x: usize, y: usize) -> usize {
        return x + y * WIDTH as usize;
    }

    fn update(&mut self) {
      for y in 0..HEIGHT as usize {
        for x in 0..WIDTH as usize {
            let idx = self.to_index(x, y);
            let alive = self.cells[idx] == ALIVE;
            let n = self.count_neighbours(x, y);

            let next = (alive && (n == 2 || n == 3)) || (!alive && n == 3);

            if next {
                self.cells_buffer[idx] = ALIVE;
            } else {
                self.cells_buffer[idx] = self.cells[idx].saturating_sub(1);
            }
        }
      }

      self.iteration += 1;
      std::mem::swap(&mut self.cells_buffer, &mut self.cells);
    }

    fn render(&mut self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let rgba = if self.cells[i] == ALIVE { 
                [0xff,0xff,0xff,0xff] 
            } else { 
                [0x00,self.cells[i],self.cells[i],0xff] 
            };

            pixel.copy_from_slice(&rgba);
        }
    }
}


impl Default for World {
    fn default() -> Self {
        let mut cells = vec![0; (WIDTH * HEIGHT) as usize];
        let mut rng = rand::rng();

        let perlin = Perlin::new(rng.random());

        for (i, cell) in cells.iter_mut().enumerate() {
            let level = (perlin.get([((i % WIDTH as usize) as f64) / 25.0, ((i / WIDTH as usize) as f64) / 25.0]) + 1.0) / 2.0;
            *cell = if level >= 0.5 { ALIVE } else { DEAD };
        }

        let mut world = World {
            cells: cells,
            cells_buffer: vec![0; (WIDTH * HEIGHT) as usize],
            iteration: 0
        };

        for _ in 0..10 {
            world.update();
            world.cool();
        }

        world
    }
}

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
                self.world.update();
               
                save_image(self.pixels.as_mut().unwrap().frame(), self.world.iteration);
               
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

