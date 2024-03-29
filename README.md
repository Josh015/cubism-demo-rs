# Cubism Demo

A Bevy Engine port and refresh of an old C++ OpenGL lighting and shading demo I made back in 2010.

## To Run

To compile and run, use [cargo](https://www.rust-lang.org/learn/get-started):

```shell
cargo run --release
```

## Screenshot

![Cubism Demo](screenshots/screenshot.png)

## TODO

- [x] Get basic layout of all objects and camera.
- [x] Add grid material caching using xpm palettes?
- [x] Ensure that cube groups are parented? How to animate?
- [x] Add different animations via systems and enum component.
- [x] Add spinning light animation.
- [x] Add different camera angles.
- [x] Add onscreen instructions UI.
- [x] Add PBR spherical area lights.
- [x] Fix the UI instructions.
- [x] Add ron support and move configs out of code.
- [x] Restore 256 light setup once Bevy supports it.
- [x] Restore UI.
- [ ] Add hot reloading for the config file?
- [ ] Modularize XPM code to support loading different types and multi-character pixels.

## Original Video

[![Watch the video](https://i.vimeocdn.com/video/93015207_472x266.jpg)](https://vimeo.com/15442169)

[Cubism Demo (ver. 2)](https://vimeo.com/15442169) from [Josh 015](https://vimeo.com/user2176585) on [Vimeo](https://vimeo.com)
