# Hexlab

<!-- toc -->

- [Features](#features)
- [Installation](#installation)
- [Getting Started](#getting-started)
- [Usage](#usage)
- [Documentation](#documentation)
- [Contributing](#contributing)
- [Acknowledgements](#acknowledgements)
- [License](#license)

<!-- tocstop -->

Hexlab is a Rust library for generating and manipulating hexagonal mazes.

## Features

- Create hexagonal mazes of configurable size
- Customizable maze properties (radius, start position, seed)
- Efficient bit-flag representation of walls for optimized memory usage
- Multiple maze generation algorithms (WIP)
- Maze builder pattern for easy and flexible maze creation

## Installation

Add `hexlab` as a dependency:

```sh
cargo add hexlab
```

## Getting Started

```rust
use hexlab::prelude::*;

fn main() {
    // Create a new maze with radius 5
    let maze = MazeBuilder::new()
        .with_radius(5)
        .build()
        .expect("Failed to create maze");
    println!("Maze size: {}", maze.count());
}
```

## Usage

```rust
use hexlab::prelude::*;

// Create a new maze
let maze = MazeBuilder::new()
    .with_radius(5)
    .build()
    .expect("Failed to create maze");

// Get a specific tile
let tile = maze.get_tile(&Hex::new(1, -1)).unwrap();

// Check if a wall exists
let has_wall = tile.walls().contains(EdgeDirection::FLAT_NORTH);
```

## Documentation

Full documentation is available at [docs.rs](https://docs.rs/hexlab).

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Acknowledgements

Hexlab relies on the excellent [hexx](https://github.com/ManevilleF/hexx)
library for handling hexagonal grid mathematics, coordinates, and related
operations. We're grateful for the robust foundation it provides for working
with hexagonal grids.

## License

This project is dual-licensed under either:

- MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

at your option.
