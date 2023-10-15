use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_utils::prelude::*;
use raylib::prelude::*;

fn main() {
    App::new()
        .init_resource::<Cursor>()
        .insert_resource(WindowConfig {
            title: "Bevy + Raylib".to_owned(),
            ..default()
        })
        .add_systems(PreUpdate, update_cursor)
        .add_systems(PostUpdate, render)
        .set_runner(runner)
        .run();
}

fn runner(mut app: App) {
    let window_config = app
        .world
        .remove_resource::<WindowConfig>()
        .unwrap_or_default();
    let raylib_context = {
        let (rl, thread) = raylib::init()
            .size(window_config.width, window_config.height)
            .title(&window_config.title)
            .build();
        RaylibContext { rl, thread }
    };
    app.world.insert_non_send_resource(raylib_context);

    let should_close = |app: &App| {
        app.world
            .get_non_send_resource::<RaylibContext>()
            .map_or(false, |context| context.rl.window_should_close())
    };

    while !should_close(&app) {
        app.update();
    }
}

fn update_cursor(raylib_context: NonSend<RaylibContext>, mut cursor: ResMut<Cursor>) {
    *cursor = {
        let Vector2 { x, y } = raylib_context.rl.get_mouse_position();
        Cursor { x, y }
    }
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

struct RaylibContext {
    rl: RaylibHandle,
    thread: RaylibThread,
}

impl RaylibContext {
    pub fn begin_drawing(&mut self) -> RaylibDrawHandle {
        self.rl.begin_drawing(&self.thread)
    }
}

#[derive(Resource)]
struct WindowConfig {
    pub width: i32,
    pub height: i32,
    pub title: String,
}

impl Default for WindowConfig {
    fn default() -> Self {
        WindowConfig {
            width: 640,
            height: 480,
            title: "App".to_owned(),
        }
    }
}

#[derive(Resource, Debug, Default)]
struct Cursor {
    x: f32,
    y: f32,
}
