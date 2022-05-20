use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use shared::*;

fn main() {
    // General
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);

    // Server-Only
    app.add_plugin(WorldInspectorPlugin::new());

    // Shared
    app.add_startup_system(setup);

    app.run();
}
