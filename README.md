bevy_raylib
===========

This is a very simple raylib plugin for bevy.


Usage
-----

```rust
use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_raylib::prelude::*;
use bevy_utils::prelude::*;

fn main() {
    App::new()
        .add_plugins(RaylibPlugin)
        .insert_resource(WindowConfig {
            title: "Bevy + Raylib".to_owned(),
            ..default()
        })
        .add_systems(PostUpdate, render)
        .run();
}

fn render(mut raylib: NonSendMut<RaylibContext>, cursor: Res<Cursor>) {
    let mut d = raylib.begin_drawing();
    d.clear_background(Color::WHITE);
    d.draw_text(
        &format!("{}, {}", cursor.x, cursor.y),
        12,
        12,
        20,
        Color::BLACK,
    );
}
```
