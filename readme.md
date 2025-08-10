# Conway's Game of Life 👾

&emsp;&emsp;

<div align="center">
  <span>‹ Previous</span> &emsp;&emsp;&emsp;&emsp;
  <strong>Project 01</strong> &emsp;&emsp;&emsp;&emsp;
  <a href="https://github.com/gigalasr/game-of-gpu">Next ›</a>
</div>

&emsp;&emsp;

## 👋 Intro
![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white) ![Visual Studio Code](https://img.shields.io/badge/Visual%20Studio%20Code-0078d7.svg?style=for-the-badge&logo=visual-studio-code&logoColor=white) 

<div align="center">

![Game Of Life GIF](docs/conway.webp)

1000 iterations. Initial configuration generated from perlin noise.

</div>

This is the first in a series of projects I’m developing to learn Rust, with a focus on graphics programming. 
For this one, I implemented Conway's Game of Life using [winit](https://crates.io/crates/winit) and [pixels](https://crates.io/crates/pixels) crate. 
The pixels crate was especially helpful, as it provides a simple frame buffer that's easy to draw to. 
I sometimes took a slightly less performant approach on purpose to explore and experiment with different language features. 
The trail effect was inspired by the Conway's Game of Life example from the pixels crate’s GitHub repository.

## 📖 Resources
The following resources helped me to implement this project:

- https://docs.rs/pixels/latest/pixels/
- https://github.com/parasyte/pixels
- https://docs.rs/winit/latest/winit/
- https://github.com/parasyte/pixels/issues/403
- https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life

## 🛠️ Usage
There are a few flags you can set to change the size of the grid and the window:
```
--width <number>  - Set the width of the grid
--height <number> - Set the height of the grid 
--scale <number>  - Set the window scale (multiplier for width/height)
--save-frames     - Save all frames as pngs to a folder
``` 

For exmaple, to launch the simulation on a 400x200 Grid with a window size of 800x400 simply run:
```
cargo run -- --width 400 --height 200 --scale 2
```

## 🔮 Future Work 
For my next project I plan on implementing Conway's Game of Life on the GPU using wgpu and wgsl.


