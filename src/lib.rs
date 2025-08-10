use std::slice::Iter;
use noise::{NoiseFn, Perlin};
use rand::Rng;


const WIDTH: u32 = 200;
const HEIGHT: u32 = 100;
const ALIVE: u8 = 255;
const DEAD: u8 = 0;

#[derive(Debug)]
pub struct Config {
    pub width: usize,
    pub height: usize,
    pub scale: usize,
    pub max_iterations: Option<usize>,
    pub save_frames: bool
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, String> {
        let mut config = Config {
            width: 200,
            height: 100,
            scale: 4,
            max_iterations: None,
            save_frames: false
        };

        let mut args_iter = args.iter();
        args_iter.next(); // skip executable name
        while let Some(arg) = args_iter.next() {
            match arg.as_str() {
                "--width" => {
                    config.width = Config::parse_next(&mut args_iter)?;
                }
                "--height" => {
                    config.height = Config::parse_next(&mut args_iter)?;
                }
                "--max-iterations" => {
                    config.max_iterations = Some(Config::parse_next(&mut args_iter)?);
                }
                "--scale" => {
                    config.scale = Config::parse_next(&mut args_iter)?;
                }
                "--save-frames" => {
                    config.save_frames = true;
                }
                other => {
                    return Err(format!("unknown option {}", other));
                }
            }
        }

        Ok(config)
    }

    fn parse_next(args: &mut Iter<'_, String>) -> Result<usize, &'static str> {
        args.next()
            .ok_or_else(|| "Missing value for flag")?
            .parse::<usize>()
            .map_err(|_| "Could not parse value")
    }
}


pub struct World {
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

    pub fn update(&mut self) {
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

    pub fn render(&mut self, frame: &mut [u8]) {
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