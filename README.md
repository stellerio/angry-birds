# Angry Birds Rust

An original native Rust slingshot-game prototype. This project uses its own placeholder shapes and gameplay code; it does not include Angry Birds' proprietary source code or assets.

## Current prototype

- Native desktop window powered by `winit`
- GPU rendering with `wgpu`
- Drag the bird back from the slingshot, then release to launch
- Simple projectile gravity and collision checks
- Pigs and wooden blocks, with a score counter
- Press **R** to reset the level

## Run

Install a recent stable Rust toolchain from https://rustup.rs, then run:

```sh
cargo run --release
```

The first build downloads Rust dependencies. A graphics backend supported by wgpu is required.

## Controls

- **Left mouse button:** click and drag the bird backward
- **Release mouse:** launch
- **R:** reset the level
- **Esc:** quit

## Roadmap

- Better collision response and destructible structures
- Multiple bird types and levels
- Mesh/texture asset pipeline
- Sound and particles
- Save/load level data

## License

MIT. See the repository license file when added. Third-party crate licenses remain with their respective authors.
