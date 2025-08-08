# Conway's Game of Life 👾
<div align="center">
  <span style="float: left;"><a href="">‹ Previous</a></span>
  <strong>Conway's Game of Life</strong>
  <span style="float: right;"><a href="">Next ›</a></span>
</div>


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

## 🔮 Future Work 
For my next project I plan on implementing Conway's Game of Life on the GPU using wgpu and wgsl.
