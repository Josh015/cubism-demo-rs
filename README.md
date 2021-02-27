# Cubism Demo
A Bevy Engine port of an old C++ OpenGL demo I made back in 2010.

## Run the sample

To compile and run, use [cargo](https://www.rust-lang.org/learn/get-started):

```
cargo run --release
```

## TODO
- [x] Get basic layout of all objects and camera.
- [x] Add material caching using xpm palettes?
- [x] Ensure that cube groups are parented? How to animate?
- [x] Add different animations via systems and enum component.
- [x] Add spinning light animation.
- [x] Add different camera angles.
- [x] Switch to custom material for emissive objects.
- [x] Add onscreen instructions UI that can be minimized.
- [x] Consolidate scene code into `demo` module with a single `DemoPlugin`.
- [x] Add custom material with support for 256 lights.
- [x] Add simple tonemapping.
- [ ] Add simple clustered forward renderer.
- [ ] Nicer UI?

## Old Video
[![Watch the video](https://i.vimeocdn.com/video/93015207_472x266.jpg)](https://vimeo.com/15442169)
<p><a href="https://vimeo.com/15442169">Cubism Demo (ver. 2)</a> from <a href="https://vimeo.com/user2176585">Josh 015</a> on <a href="https://vimeo.com">Vimeo</a>.</p>
