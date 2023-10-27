# bevy_qoi

![bevy_qoi Logo](https://github.com/digitaljokerman/bevy_qoi/blob/main/assets/logo.png)

[![Version](https://img.shields.io/github/release/digitaljokerman/bevy_qoi.svg)](https://github.com/digitaljokerman/bevy_qoi/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![CodeFactor](https://www.codefactor.io/repository/github/digitaljokerman/bevy_qoi/badge)](https://www.codefactor.io/repository/github/digitaljokerman/bevy_qoi)
[![Documentation](https://docs.rs/bevy_qoi/badge.svg)](https://docs.rs/bevy_qoi)

## Description

`bevy_qoi` is a Rust library that introduces support for QOI (Quite OK Image) format to Bevy, an open-source game development framework.

The Easy-to-use plugin adds the capability for Bevy applications to load and render QOI images. `bevy_qoi` is incredibly efficient, fast, and keeps the bevy applications fast, light, and productive.

The version 0.1.0 of the `bevy_qoi` package is released under the MIT license. This robust plugin extends Bevy's functionality while maintaining its performance and versatility.

The package includes the following files:
1. `Cargo.toml`: The package manifest file for Rust's package manager, Cargo.
2. `src/lib.rs`: The primary library file where the plugin's features/functions are implemented.
3. `examples/display.rs`: An example Rust file that demonstrates the plugin's usage in an application.


## Installation

To install `bevy_qoi`, add it as a dependency in your Cargo.toml:

```toml
[dependencies]
bevy_qoi = "0.1.0"
```

Then run `cargo build` to download and compile the `bevy_qoi` crate along with all its dependencies.


## Usage

To integrate the `QOIPlugin` into your Bevy application, add the plugin using Bevy's `.add_plugin()` method. 

Here's a simple example on how to use the QOIPlugin in your app:

```rs
use bevy::prelude::*;
use bevy_qoi::QOIPlugin;

fn main() {
    App::new()
        // ...
        .add_plugin(QOIPlugin)
        // ...
        .run();
}
```

Replace `// ...` with initialization code and other plugins as required by your application.


## Contributing

Code contributions are welcome! Please feel free to open a PR or file an issue if you encounter any problems.


## License

`bevy_qoi` is distributed under the terms of the MIT license. See [LICENSE](https://github.com/digitaljokerman/bevy_qoi/blob/main/LICENSE.md) for details.