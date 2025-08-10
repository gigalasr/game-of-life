use noise::{NoiseFn, Perlin};
use rand::Rng;

pub mod config;

const ALIVE: u8 = 255;
const DEAD: u8 = 0;

pub struct World {
    cells: Vec<u8>,
    cells_buffer: Vec<u8>,
    width: usize,
    height: usize
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
            let nx = ((x as isize + self.width as isize + dx) % self.width as isize) as usize;
            let ny = ((y as isize + self.height as isize + dy) % self.height as isize) as usize;
            if self.cells[nx + ny * self.width] == ALIVE {
                count += 1;
            }
        }

        count
    }

    fn to_index(&self, x: usize, y: usize) -> usize {
        return x + y * self.width;
    }

    fn cell(&self, x: usize, y: usize) -> &u8 {
        return &self.cells[x + y * self.width];
    }

    pub fn update(&mut self) {
      for y in 0..self.height as usize {
        for x in 0..self.width as usize {
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


impl World {
    pub fn new(width: usize, height: usize) -> Self {

        let mut cells = vec![0; width * height];
        let mut rng = rand::rng();

        let perlin = Perlin::new(rng.random());

        for (i, cell) in cells.iter_mut().enumerate() {
            let level = (perlin.get([((i % width) as f64) / 25.0, ((i / width) as f64) / 25.0]) + 1.0) / 2.0;
            *cell = if level >= 0.5 { ALIVE } else { DEAD };
        }

        println!("Width is {width}");
        println!("Height is {height}");

        World {
            cells: cells,
            cells_buffer: vec![0; width * height],
            width: width,
            height: height
        }
    }
}