# space_invaders_arcade_emulator

Space Invaders arcade game emulator in development made in Rust.

## Description

Complete Emulator of the Intel 8080, the app is implemented to run the Space Invaders Arcade game.

## Features

## Images

## Videos

## Quick start

To use the Emulator, you need the roms. The roms the emulator is working with are named:

- invaders.h
- invaders.g
- invaders.e
- invaders.f

Download the executable and create a folder named `roms` next to it. In this folder put the roms, now, you can start the
emulator by just double-clicking it or by typing the command below next to the executable.

```bash
./space_invaders_arcade_emulator.exe
```

## Compilation

First thing you need is to install cargo. You have it if you already have installed Rust, if not, please follow the
instruction at the link below:  
<https://www.rust-lang.org/tools/install>

### Windows

~~Windows users need to install SDL2 for your OS. The setup instruction can be followed in the link below:~~  
<https://github.com/Rust-SDL2/rust-sdl2>

I set up a static SDL2 Linking so Windows user just need to download the project and everything will be set up for SDL2.
I used the instructions in the link below to set up the dynamic linking:  
<https://github.com/Rust-SDL2/rust-sdl2#user-content-windows-with-build-script>

### Linux

Linux Users need to install the SDL2 lib. To do so, type in your bash system the following commands.

If you want to compile the program, install the developer version with the command below:

```bash
sudo apt-get install libsdl2-dev
```

or if you just want to use the emulator type this command:

```bash
sudo apt-get install libsdl2-2.0-0  
```

---

With Rust and SDL2 libs installed, you can now compile the project in two-way, debug or release. To compile go to the
project root folder and type one of the two following commands below (If you want to use the emulator please compile
using the second command).

```bash
cargo build
```

or

```bash
cargo build --release
```

The compiled app will be in the folder `target/debug` or `target/release` depending on the compilation you did.

## Rust tests

<https://doc.rust-lang.org/book/ch11-00-testing.html>

```bash
cargo test
```

If you want some debug infos about the cpu type:

```bash
cargo test -- --nocapture
```

## GitHub Actions

[![Rust](https://github.com/Im-Rises/space_invaders_arcade_emulator/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/Im-Rises/space_invaders_arcade_emulator/actions/workflows/rust.yml)
[![rust-clippy analyze](https://github.com/Im-Rises/space_invaders_arcade_emulator/actions/workflows/rust-clippy.yml/badge.svg?branch=main)](https://github.com/Im-Rises/space_invaders_arcade_emulator/actions/workflows/rust-clippy.yml)

The project is set with a set of different scripts:

- Rust : Check the code compilation.
- Rust clippy : Evaluate the code quality (error, warnings, etc...).
- Rust publisher : Publish the app to releases when pushing to the main branch.

## Documentation

emulator101:  
<http://www.emulator101.com>

Computer Archeology:  
<https://www.computerarcheology.com/Arcade/SpaceInvaders/Hardware.html>

Emudev.de:  
<https://emudev.de/q00-si/a-short-fun-project/>

Rust:  
<https://doc.rust-lang.org/book>

SDL2 Rust:  
<https://www.libsdl.org>  
<https://github.com/Rust-SDL2/rust-sdl2>

Intel 8080 documentations:  
<https://pastraiser.com/cpu/i8080/i8080_opcodes.html>  
<https://archive.org/details/8080Datasheet>  
<https://altairclone.com/downloads/manuals/8080%20Programmers%20Manual.pdf>

Intel 8080 opcodes table:  
<https://www.pastraiser.com/cpu/i8080/i8080_opcodes.html>

Wikipedia:  
<https://en.wikipedia.org/wiki/Intel_8080>
