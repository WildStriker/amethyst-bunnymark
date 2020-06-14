# amethyst-bunnymark
Implementation of BunnyMark for amethyst, a rust game engine.

Based on the original BunnyMark by Iain Lobb (code) and Amanda Lobb (art) - [Link](http://blog.iainlobb.com/2010/11/display-list-vs-blitting-results.html)

## How to run

To run the game, run the following command, which defaults to the `vulkan` graphics backend:

```bash
cargo run --release
```

Windows and Linux users may explicitly choose `"vulkan"` with the following command:

```bash
cargo run --release --no-default-features --features "vulkan"
```

Mac OS X users may explicitly choose `"metal"` with the following command:

```bash
cargo run --release --no-default-features --features "metal"
```

## Assets

Assets used in this project were taken from the following sources.

1. square.tff
    * Taken from the amethest rust project.
    * Located [here](https://github.com/amethyst/amethyst/tree/1e96d7fe2ad5e7e7d87a8aeddd7814fa72009745/examples/assets/font)

2. wabbit_alpha.png
    * Taken from the original bunny mark source, found on [Iain's Blog](http://blog.iainlobb.com/2010/11/display-list-vs-blitting-results.html)
    * Can be downloaded directly [here](http://www.iainlobb.com/bunnies/BUNNYMARK.zip)
