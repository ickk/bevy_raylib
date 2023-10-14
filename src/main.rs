use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use raylib::prelude::*;

struct RaylibContext {
    rl: RaylibHandle,
    thread: RaylibThread,
}

#[derive(Resource, Debug)]
struct Cursor {
    x: f32,
    y: f32,
}

fn main() {
    let raylib_context = {
        let (rl, thread) = raylib::init().size(640, 480).title("Hello, World").build();
        RaylibContext { rl, thread }
    };

    App::new()
        .insert_non_send_resource(raylib_context)
        .add_systems(Update, update_system)
        .set_runner(runner)
        .run();
}

fn runner(mut app: App) {
    let raylib_context = &mut app
        .world
        .remove_non_send_resource::<RaylibContext>()
        .expect("RaylibContext doesn't exist");

    while !raylib_context.rl.window_should_close() {
        input_mapping(raylib_context, &mut app.world);

        app.update();

        draw(raylib_context, &mut app.world);
    }
}

fn update_system() {
    // simulation
}

fn input_mapping(context: &mut RaylibContext, world: &mut World) {
    let cursor = {
        let Vector2 { x, y } = context.rl.get_mouse_position();
        Cursor { x, y }
    };
    world.insert_resource(cursor);
}

fn draw(context: &mut RaylibContext, world: &mut World) {
    let mut d = context.rl.begin_drawing(&context.thread);
    d.clear_background(Color::WHITE);
    let cursor = world.resource::<Cursor>();
    d.draw_text(
        &format!("{}, {}", cursor.x, cursor.y),
        12,
        12,
        20,
        Color::BLACK,
    );
}
