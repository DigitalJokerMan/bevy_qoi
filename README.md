# bevy_qoi

Bevy support for the QOI (Quite OK Image) format.

## Usage

Add the `QOIPlugin` to your app and you're good to go.

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
