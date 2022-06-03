# Rusteroids
A rust asteroids game, made mostly for learning purposes but may offer someone else an interesting start.

## How the project was born
I have been curious about Rust for a long time and this project was born out of my wanting to learn Rust. It probably has a lot of issues and "bad practice" so there is no guarantee that it's done "correctly". I am very interested in suggestions or glaringly obvious issues with the code, so please let me know if you see something. 

## Installing SDL2 - Windows
You will need to download the SDL2 development libraries. [https://www.libsdl.org/download-2.0.php](https://www.libsdl.org/download-2.0.php). Export the libraries into the following folders
```
msvc
  -- dll
    -- 32
    -- 64
  -- lib
    -- 32
    -- 64
mingw
  -- dll
    -- 32
    -- 64
  -- lib
    -- 32
    --64
```
`msvc` is for the Microsoft VC++ libraries. `.dll` files go into the `dll` folder and `.lib` files go into the `lib` folder. The `32` and `64` folders are for the 32-bit and 64-bit versions of the libraries, depending on which platform you are hoping to build for. The majority of this code is being tested on Windows x64 - if you are able to easily expand these instructions for Linux and MacOS then please issue a PR and help keep the documents up 
to date and relevant. I will get around to those platforms when I have a chance, but for now the testing is on Windows. 

A more complete set of instructions can be found here [https://rustrepo.com/repo/AngryLawyer-rust-sdl2](https://rustrepo.com/repo/AngryLawyer-rust-sdl2), from which is where this build file and toolchain took inspiration. 

## Tutorials
This will be updated when the tutorials have been made/published. The tags will match the tutorial numbers. I'm hoping to be able to get YouTube videos to demonstrate how to make each stage of this project, however this takes time, so please be patient while these are in production. 

| Tutorial | Tag | Description | Link |
| -------- | --- | ----------- | ---- |
| Tutorial 1 | [tutorial1](https://github.com/filtoid/rusteroids/releases/tag/tutorial1) | Getting set up with an SDL2 Window | [YouTube](https://youtu.be/SzxWkoK4uv4) |
| Tutorial 2 | [tutorial2](https://github.com/filtoid/rusteroids/releases/tag/tutorial2) | Drawing Text to an SDL2 Canvas | [YouTube](https://youtu.be/vVJIYaX3Kjw) |
| Tutorial 3 | [tutorial3](https://github.com/filtoid/rusteroids/releases/tag/tutorial3) | Draw a PNG to the SDL2 Canvas | [YouTube](https://youtu.be/scGSiMF02eo) |
| Tutorial 4 | [tutorial4](https://github.com/filtoid/rusteroids/releases/tag/tutorial4) | Adding a Key Manager | [YouTube](https://www.youtube.com/flU6h4iHdhw) |
| Tutorial 5 | [tutorial5](https://github.com/filtoid/rusteroids/releases/tag/tutorial5) | Adding Specs Entity Component System (ECS) | [YouTube](https://youtu.be/HtESVyc0DNY) |
| Tutorial 6 | [tutorial6](https://github.com/filtoid/rusteroids/releases/tag/tutorial6) | Adding Movement to the Player Character | [YouTube](https://youtu.be/sBx2x0n72AI) |
| Tutorial 7 | [tutorial7](https://github.com/filtoid/rusteroids/releases/tag/tutorial7) | Adding Momentum to the Player Character | [YouTube](https://youtu.be/OltffJldfZg) |
| Tutorial 8 | [tutorial8](https://github.com/filtoid/rusteroids/releases/tag/tutorial8) | Adding Asteroid to the game world | [YouTube](https://youtu.be/8YuS3d51PJo) |
| Tutorial 9 | [tutorial9](https://github.com/filtoid/rusteroids/releases/tag/tutorial9) | Colliding the Asteroid and Player | [YouTube](https://youtu.be/KTDdlWErmYU) |
| Tutorial 10 | [tutorial10](https://github.com/filtoid/rusteroids/releases/tag/tutorial10) | Allowing Player to Fire a Missile | [YouTube](https://youtu.be/Qk_EVsG_IhA) |


## License
This example project is provided with the MIT license. My understanding of this is that this means you can take this code, change it, release it, sell it, do as you want with it. No guarantee is provided that this code won't melt your computer (although I hope it won't). If you make something really cool then please let me know and we'll post a link to it somewhere in this Readme. 
