[![Test](https://github.com/Zitronenjoghurt/lemon-gb/actions/workflows/test.yaml/badge.svg)](https://github.com/Zitronenjoghurt/lemon-gb/actions/workflows/test.yaml)
[![codecov](https://codecov.io/gh/Zitronenjoghurt/lemon-gb/graph/badge.svg?token=UM6T22YO17)](https://codecov.io/gh/Zitronenjoghurt/lemon-gb)
![](https://tokei.rs/b1/github/Zitronenjoghurt/lemon-gb?category=code&type=Rust&logo=https://simpleicons.org/icons/rust.svg)

# lemon-gb

A work-in-progress Game Boy emulator written in Rust. The main purpose of this project is to learn how the gameboy works
and how an emulator for it could work like from an architectural point of view.

# Learning Material

The sources I have used to create this emulator.

- https://rylev.github.io/DMG-01/public/book/introduction.html
    - Great entrypoint for building the foundation of the emulator
- https://gbdev.io/pandocs/
    - Really nice technical documentation
- https://gekkio.fi/files/gb-docs/gbctr.pdf
    - Great for going in-depth into more specific things
- http://marc.rawer.de/Gameboy/Docs/GBCPUman.pdf
    - Detailed CPU explanations
- https://hacktix.github.io/GBEDG/
    - Even more detailed information especially for the PPU and timer

# Current State

WORK IN PROGRESS, not launchable yet

- Currently only the title screen of two games render properly
- No input, no sound, no sprite/object and window rendering
- Major visual glitches in the vast majority of games

## Components

- ✅ CPU (501/501 Instructions, except STOP)
    - Passing Blarggs cpu tests
- ✅ Timer
- 🚧 MMU (see memory banking)
- 🚧 PPU
- ❌ APU

## Memory Banking

- ✅ No MBC
- ✅ MBC1
- ❌ MBC2
- ❌ MBC3
- ❌ MBC5
- ❌ MBC6
- ❌ MBC7
- ❌ ... (others)

## Examples

### Donkey Kong Land (Title Screen)

Only background rendering
![Donkey Kong Land (Title Screen)](public/dk_bg_rendering.png)

### Dr. Mario (Title Screen)

Only background rendering
![dr_mario_bg_rendering.png](public/dr_mario_bg_rendering.png)