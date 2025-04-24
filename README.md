# EMU

EMU is a Gameboy emulator intended to serve as a testbed for improving emulation performance. The original Gameboy was chosen as an emulation target due to its suitable level of complexity - the SNES system would be too simple @Kelley_2013 but the Gameboy Advance would be difficult to emulate within a semester. Still, the Gameboy is not a simple device, with over 500 opcodes in the CPU alone, combined with arcane details like cartridge maps, DMA, and scanline graphics.

The *primary goal* is to implement a working Gameboy emulator from scratch, which is itself quite a complex task. The *secondary goal* is to experiment with techniques for improving performance.

## Workspace

The project is split into four major parts.

### CPU

Contains the emulated CPU implementation used for initial development and as a fallback for the JIT branch. Also includes an automated testing suite.

### Emulator

The debugger used primarily for development. Contains a set of tools used to explore the unified bus and system state.

### Library

The core hardware emulation systems, abstracted out from the backend CPU and frontend display. Contains all GameBoy systems attached to a abstract `Addressable` object for integration.

### SDL

The end-user emulator, with a simple frontend built purely to play games.

## Building

All subprojects are built with Cargo. More information about setting up the SDL library (required for the sdl version) is available on [their repository](https://github.com/Rust-SDL2/rust-sdl2).

## Usage

All executable subprojects (emulator and sdl) take a single argument: the path to the ROM you want to run.
