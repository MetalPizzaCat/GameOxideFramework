# Game Oxide Framework

Pretty basic framework for making 2d games written fully in rust using ideas of ECS, with help of https://github.com/amethyst/specs, and using SDL2 for handling rendering.

Rust is iron oxide, but game is iron so game oxide.

## Features

* Basic asset loading
* Simplification of window generation
* Basic UI system

## Notes

Because this depends on rust-sdl2 crate there is some additional work necessary before using this framework. See [Rust SDl2 requirements](https://github.com/Rust-SDL2/rust-sdl2/blob/master/README.md#sdl20-development-libraries)

## Branch info
This is the master branch which contains latest changes and as such the least stable

## Dependencies

| crate   | version                                                                                        |
----------|------------------------------------------------------------------------------------------------|
| specs   | [![specs](https://img.shields.io/crates/v/specs.svg)](https://crates.io/crates/specs/)         |
| sdl2    | [![rust-sdl2](https://img.shields.io/crates/v/sdl2.svg)](https://crates.io/crates/sdl2)        |
| nalgebra| [![nalgebra](https://img.shields.io/crates/v/nalgebra.svg)](https://crates.io/crates/nalgebra) |
