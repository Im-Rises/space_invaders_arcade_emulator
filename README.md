# space_invaders_arcade_emulator

<p align="center">
      <img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" alt="rustLogo" style="height:60px;"/>
      <img src="https://user-images.githubusercontent.com/59691442/181630275-92b292b4-854c-447c-8da9-f8281c1e89c4.png" alt="sdl2Logo" style="height:60px;"/>
</p>

## Description

Space Invaders arcade game emulator made in Rust with SDL2 and SDL2_mixer.

Complete Emulator of the Intel 8080, the app is implemented to run the Space Invaders Arcade game.

Everything is working including the Space Invaders Easter Egg.

## Features

- Full emulation
- Sound
- Two-players mode
- Window resizing without deformation

## Images

| Title screen                                                                                                           | Game screen                                                                                                            |
|------------------------------------------------------------------------------------------------------------------------|------------------------------------------------------------------------------------------------------------------------|
| ![title_screen](https://user-images.githubusercontent.com/59691442/181736212-8d8cfa4e-4c85-48ce-92ac-1165dcb73891.png) | ![playing_demo](https://user-images.githubusercontent.com/59691442/181736224-da769503-2a2e-45d6-af2c-9204a96e78e1.png) |

| Taito Cop Easter Egg                                                                                                           | Score advance table with Invaders                                                                                             |
|--------------------------------------------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------------------------|
| ![taito_cop_easter_egg](https://user-images.githubusercontent.com/59691442/183047666-97f9711c-e2a4-4659-86df-410db5562450.png) | ![score_advance_table](https://user-images.githubusercontent.com/59691442/183058044-b5d532d6-bad2-4629-a55c-f669c82a5e29.png) |

## Videos

https://user-images.githubusercontent.com/59691442/183045566-0a3df947-06e7-4c46-9fc6-9d2b8f7d9a46.mp4

## Quick start

To download the emulator, you can click one of the icons below depending on your operating system, or you can click the
release section of the GitHub page.

For each version when you unzip the downloaded release, you will get the executable and two folder, one
named `game_roms` where you must put the game's roms:

- invaders.h
- invaders.g
- invaders.e
- invaders.f

Depending on you `operating system` you will need to install some libs, they are installed differently depending on your
system, please follow one of the section below `Windows` or `Linux` or `MacOs`.

### Windows

<a href="https://github.com/Im-Rises/space_invaders_arcade_emulator/releases/latest"><img src="https://img.shields.io/badge/Windows-0078D6?style=for-the-badge&logo=windows&logoColor=white" alt="cmakeLogo" style="height:40px;"/></a>

For Windows users you don't need to install the libs, you just need to be carefully that you have the `SDL2.dll`
and `SDL2_mixer.dll` next to the `space_invaders_arcade_emulator.exe` or the emulator won't start.

Once everything is set up by you can start the emulator by double-clicking the executable of typing the following
command next to it:

```bash
.\space_invaders_arcade_emulator.exe
```

### Linux

<a href="https://github.com/Im-Rises/space_invaders_arcade_emulator/releases/latest"><img src="https://img.shields.io/badge/Linux-FCC624?style=for-the-badge&logo=linux&logoColor=black" alt="cmakeLogo" style="height:40px;"/></a>

For Linux users, you need to install the SDL2 lib, to do so type one of the following commands:

```bash
sudo apt-get install libsdl2-2.0-0 libsdl2-mixer-2.0-0
```

or if you're a developer and want to compile the Emulator, please install this version of SDL2:

```bash
sudo apt-get install libsdl2-dev libsdl2-mixer-dev
```

Then you can start by double-clicking the executable of typing the following command next to it:

```bash
./space_invaders_arcade_emulator
```

### MacOs

<a href="https://github.com/Im-Rises/space_invaders_arcade_emulator/releases/latest"><img src="https://img.shields.io/badge/mac%20os-000000?style=for-the-badge&logo=apple&logoColor=white" alt="cmakeLogo" style="height:40px;"/></a>

For macOS users you will need to install Brew, please follow the instruction in the link below:  
<https://brew.sh>

Once it is installed, you can type the following command to install SDL2.

```bash
brew install sdl2
brew install sdl2_mixer
```

You also need to add `SDL2` to the paths by typing:

```bash
export LIBRARY_PATH="$LIBRARY_PATH:$(brew --prefix)/lib"
```

Then you can start by double-clicking the executable of typing the following command next to it:

```bash
./space_invaders_arcade_emulator
```

## Controls

You can use the keyboard to play the game.

| Arcade buttons | Emulator/Keyboard |
|----------------|-------------------|
| Insert coin    | C                 |
| P1 start       | Space             |
| P1 shoot       | ↑                 |
| P1 ←           | ←                 |
| P1 →           | →                 |
| P2 start       | G                 |
| P2 shoot       | E                 |
| P2 ←           | S                 |
| P2 →           | F                 |

The original game is mapped with some inputs that allow the constructor to choose the difficulty. I Mapped those inputs
to the keyboard. This allows you to increase the numer of lives and change the extra ship necessity points.

Before pressing start with player 1 or 2, you can choose the number of life you want to have for a game party.

| Emulator/Keyboard | Emulator buttons                   |
|-------------------|------------------------------------|
| K                 | 1 more life                        |
| L                 | 2 more lives                       |
| M                 | extra ship at 1000 instead of 1500 |

> **Note**  
> If you don't keep pressed K or L before pressing start and starting a new game you will have 3 lives.  
> In the same way, you can enable the extra ship to came at 1000 points instead of 1500, but you just need to press
> the button one time (a confirmation will be displayed in the console).

## Code architecture

The Emulator is divided into 4 parts:

- main (starter)
- si_arcade (console emulation)
- my_sdl2 (video, audio and inputs si_arcade interpreter)
- binary_lib (a set of binary functions to manipulate data)

The sound is implemented using `.wav` files, they are loaded in the `si_aracade` part of the program and totally
interpreted in the `my_sdl2` part.

## Compilation

First thing you need is to install cargo and rust. You can find them by following the instructions in the link below:  
<https://www.rust-lang.org/tools/install>

You also need to download the following audio files:

- 0.wav
- 1.wav
- 2.wav
- 3.wav
- 4.wav
- 5.wav
- 6.wav
- 7.wav
- 8.wav

The `wav` files wan be downloaded in the links below:  
<https://samples.mameworld.info>  
<https://www.classicgaming.cc/classics/space-invaders/sounds>

They all must be put in the `game_audios` folder.

> **Warning**  
> Be carefully when downloading the .wav files, some files pay have the wrong name.

Depending on your OS, you will need to follow specific steps to compile the app. Please refer to the sections
below `Windows`, `Linux` and `macOs`.

### Windows

~~Windows users need to install SDL2 for your OS. The setup instruction can be followed in the link below:~~  
<https://github.com/Rust-SDL2/rust-sdl2>

I set up a static SDL2 Linking so Windows user just need to download the project and everything will be set up for SDL2.
I used the instructions in the link below to set up the dynamic linking:  
<https://github.com/Rust-SDL2/rust-sdl2#user-content-windows-with-build-script>

### Linux

Linux Users need to install the SDL2 libs (SDL2 and SDL2_mixer). To do so, type in your bash system the following
commands.

If you want to compile the program, install the developer version with the command below:

```bash
sudo apt-get install libsdl2-dev libsdl2-mixer-dev
```

### MacOs

For macOS users you will need to install Brew, please follow the instruction in the link below:  
<https://brew.sh>

Once it is installed, you can type the following command to install SDL2.

```bash
brew install sdl2
brew install sdl2_mixer
```

You also need to add `SDL2` to the paths by typing:

```bash
export LIBRARY_PATH="$LIBRARY_PATH:$(brew --prefix)/lib"
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

It will start a test rom for the Intel 8080 CPU. You can find it in the link below:  
<https://altairclone.com/downloads/cpu_tests/>

```bash
cargo test
```

<!--
If you want some debug infos about the cpu type:

```bash
cargo test -- --show-output
```
-->

Currently, the CPU is passing the following tests:

- [x] cpudiag.bin
- [x] TST8080.COM
- [x] 8080PRE.COM
- [x] CPUTEST.COM
- [x] 8080EXM.COM

The tests are named:

- cpu_test_rom_cpudiag
- cpu_test_rom_tst8080
- cpu_test_rom_8080pre
- cpu_test_rom_cputest
- cpu_test_rom_8080exm

You can start them individuality by typing:

```bash
cargo test <test_name>
```

or

```bash
cargo test --release
```

Example: If you want to start the cpu_test_rom_tst8080 test.

```bash
cargo test cpu_test_rom_tst8080
```

or

```bash
cargo test cpu_test_rom_tst8080 --release
```

You can also debug the processes by uncommenting the two lines following lines in the `cpu.rs` file in the `test`
module.

~~~
// let mut f = File::create("test_roms/my_output.log").expect("Cannot create debug log file");  
~~~

~~~
// write_debug_to_file(&mut cpu_debug, &mut f, cycles_counter);
~~~

> **Note**  
> Depending on the test the output is different. Refer to this project for more explanation about how they work.  
> https://github.com/superzazu/8080  
> http://www.emulator101.com/full-8080-emulation.html
>
> The last test (cpu_test_rom_8080exm) can take a lot of time in debug mode, you should test it in release mode, use the
> command below:
> ```bash
> cargo test cpu_test_rom_8080exm --release
> ```

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
<https://github.com/Rust-SDL2/rust-sdl2>

SDL2 libs download:  
<https://www.libsdl.org/download-2.0.php>  
<https://github.com/libsdl-org/SDL_mixer/releases>

rust-clippy:  
<https://github.com/rust-lang/rust-clippy>

rustfmt:  
<https://github.com/rust-lang/rustfmt>

Intel 8080 documentations:  
<https://archive.org/details/8080Datasheet>  
<https://altairclone.com/downloads/manuals/8080%20Programmers%20Manual.pdf>
<http://bitsavers.org/components/intel/MCS80/9800301D_8080_8085_Assembly_Language_Programming_Manual_May81.pdf>

Intel 8080 opcodes table:  
<https://www.pastraiser.com/cpu/i8080/i8080_opcodes.html>

Wikipedia:  
<https://en.wikipedia.org/wiki/Intel_8080>

Test Roms for the Intel 8080:  
<https://github.com/superzazu/8080/>  
<https://altairclone.com/downloads/cpu_tests/>  
<http://www.emulator101.com/full-8080-emulation.html>

Space Invaders Audio files:  
<https://samples.mameworld.info>  
<https://www.classicgaming.cc/classics/space-invaders/sounds>

## Contributors

Quentin MOREL :

- @Im-Rises
- <https://github.com/Im-Rises>

[![GitHub contributors](https://contrib.rocks/image?repo=Im-Rises/GameBoyEmulator)](https://github.com/Im-Rises/GameBoyEmulator/graphs/contributors)
