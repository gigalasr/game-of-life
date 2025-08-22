use std::fmt::{self, write, Write};

use noise::{NoiseFn, Perlin};
use rand::Rng;

pub mod config;

#[derive(Clone, Debug)]
pub enum CellState {
    Alive,
    Dead(u8)
}

impl CellState {
    fn die(&self) -> Self {
        match self {
            Self::Alive => Self::Dead(254),
            Self::Dead(level) => Self::Dead(level.saturating_sub(1))
        }
    }
}

impl PartialEq for CellState {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

pub struct World {
    cells: Vec<CellState>,
    cells_buffer: Vec<CellState>,
    width: usize,
    height: usize
}

/// For formatting unit test output 
impl fmt::Debug for World {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_char('\n')?;
    
        for row in self.cells.chunks(self.width) {
            for cell in row {
                write!(f, "{}", match cell {
                    CellState::Alive => "⬜",
                    CellState::Dead(_) => "⬛"
                })?;
            }
            f.write_char('\n')?;
        }

        Ok(())
    }
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
            if *self.cell_at(nx, ny) == CellState::Alive {
                count += 1;
            }
        }

        count
    }

    fn to_index(&self, x: usize, y: usize) -> usize {
        return x + y * self.width;
    }

    fn cell_at(&self, x: usize, y: usize) -> &CellState {
        return &self.cells[x + y * self.width];
    }

    pub fn update(&mut self) {
      for y in 0..self.height as usize {
        for x in 0..self.width as usize {
            let idx = self.to_index(x, y);
            let alive = self.cells[idx] == CellState::Alive;
            let n = self.count_neighbours(x, y);

            let next = (alive && (n == 2 || n == 3)) || (!alive && n == 3);

            if next {
                self.cells_buffer[idx] = CellState::Alive;
            } else {
                // Without the enum we could just subtract here directly :D
                self.cells_buffer[idx] = self.cells[idx].die();
            }
        }
      }

      std::mem::swap(&mut self.cells_buffer, &mut self.cells);
    }

    pub fn render(&mut self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            if let CellState::Dead(level) = self.cells[i] {
                pixel.copy_from_slice(&[0x00, level, level, 0xff]);
            } else {
                pixel.copy_from_slice(& [0xff,0xff,0xff,0xff]);
            }
        }
    }
}


impl World {
    pub fn from(world: &[u8], width: usize, height: usize) -> Self {
        let cells: Vec<CellState> = world.iter()
        .map(|x| match x {
            1 => CellState::Alive,
            0 => CellState::Dead(0),
            n => panic!("Illegal Value")
        })
        .collect();

        World {
            cells_buffer: cells.clone(),
            cells: cells,
            width: width,
            height: height
        }
    }

    pub fn new(width: usize, height: usize) -> Self {

        let mut cells = vec![CellState::Dead(0); width * height];
        let mut rng = rand::rng();

        let perlin = Perlin::new(rng.random());

        for (i, cell) in cells.iter_mut().enumerate() {
            let level = (perlin.get([((i % width) as f64) / 25.0, ((i / width) as f64) / 25.0]) + 1.0) / 2.0;
            *cell = if level >= 0.5 { CellState::Alive } else { CellState::Dead(0) };
        }

        World {
            cells: cells,
            cells_buffer: vec![CellState::Dead(0); width * height],
            width: width,
            height: height
        }
    }
}

impl PartialEq for World {
    fn eq(&self, other: &Self) -> bool {
        if self.width != other.width || self.height != other.height {
            panic!("Can't copare worlds of different sizes");
        }
        
        self.cells == other.cells
    }
}