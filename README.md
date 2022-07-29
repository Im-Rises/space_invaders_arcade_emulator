# space_invaders_arcade_emulator

<p align="center">
      <img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" alt="rustLogo" style="height:60px;"/>
      <img src="https://user-images.githubusercontent.com/59691442/181630275-92b292b4-854c-447c-8da9-f8281c1e89c4.png" alt="sdl2Logo" style="height:60px;"/>
</p>

## Description

Space Invaders arcade game emulator in development made in Rust.

Complete Emulator of the Intel 8080, the app is implemented to run the Space Invaders Arcade game.

[//]: # (## Features)

## Images

| Title screen | Game demo |
|--------------|-----------|
|![title_screen](https://user-images.githubusercontent.com/59691442/181736212-8d8cfa4e-4c85-48ce-92ac-1165dcb73891.png)|![playing_demo](https://user-images.githubusercontent.com/59691442/181736224-da769503-2a2e-45d6-af2c-9204a96e78e1.png)|

[//]: # (## Videos)

## Quick start

To use the Emulator, you need the roms. The roms the emulator is working with are named:

- invaders.h
- invaders.g
- invaders.e
- invaders.f

You can download the latest version in the release section of the GitHub repository, you can also download the last
version by clicking on the icon of you os below.

### Windows

<a href="https://github.com/Im-Rises/space_invaders_arcade_emulator/releases/latest"><img src="https://img.shields.io/badge/Windows-0078D6?style=for-the-badge&logo=windows&logoColor=white" alt="cmakeLogo" style="height:40px;"/></a>

Download the executable and create a folder named `roms` next to it. In this folder put the roms, now, you can start the
emulator by just double-clicking it or by typing the command below next to the executable.

```bash
.\space_invaders_arcade_emulator.exe
```

### Linux

<a href="https://github.com/Im-Rises/space_invaders_arcade_emulator/releases/latest"><img src="https://img.shields.io/badge/Linux-FCC624?style=for-the-badge&logo=linux&logoColor=black" alt="cmakeLogo" style="height:40px;"/></a>

Download the executable and create a folder named `roms` next to it. In this folder put the roms, now, you can start the
emulator by just double-clicking it or by typing the command below next to the executable.

```bash
./space_invaders_arcade_emulator
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

You can test the good behaviour of the project by typing the commands onf of the following command. It will start the
unit test of the CPU.

It will start a test rom for the Intel 8080 CPU.

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
[![rustfmt check](https://github.com/Im-Rises/space_invaders_arcade_emulator/actions/workflows/rustfmt.yml/badge.svg?branch=main)](https://github.com/Im-Rises/space_invaders_arcade_emulator/actions/workflows/rustfmt.yml)
[![rust-publish](https://github.com/Im-Rises/space_invaders_arcade_emulator/actions/workflows/rust-publish.yaml/badge.svg?branch=main)](https://github.com/Im-Rises/space_invaders_arcade_emulator/actions/workflows/rust-publish.yaml)

The project is set with a set of different scripts:

- rust : Check the code compilation.
- rust-clippy analyze : Evaluate the code quality (error, warnings, etc...).
- rustfmt check :  Check the code good formatting
- rust-publisher : Publish the app to releases when pushing to the main branch.

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

Rustfmt:  
<https://github.com/rust-lang/rustfmt>
