use bevy::prelude::*;
use shared::*;

fn main() {
    // General
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);

    // Client-Only

    // Shared
    app.add_startup_system(setup);

    app.run();
}
