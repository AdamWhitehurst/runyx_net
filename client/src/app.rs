use bevy::prelude::*;
use bevy::{app::App, DefaultPlugins};

use bevy_egui::EguiPlugin;
use naia_bevy_client::{ClientConfig, Plugin as ClientPlugin, Stage};

use crate::resources::Global;
use shared::{protocol::Protocol, shared_config, Channels};

use crate::systems::{events, init_network_client, input, menu_ui, sync, tick};
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    MainMenu,
    PreConnect,
    InGame,
    // Paused,
}

pub type ConnectionAddress = Option<String>;

pub fn run() {
    App::default()
        // Plugins
        .add_plugins(DefaultPlugins)
        .add_plugin(ClientPlugin::<Protocol, Channels>::new(
            ClientConfig::default(),
            shared_config(),
        ))
        .init_resource::<Global>()
        .add_plugin(EguiPlugin)
        // States
        .add_state(AppState::MainMenu)
        .insert_resource(ConnectionAddress::default())
        // Startup System
        .add_system_set(SystemSet::on_update(AppState::MainMenu).with_system(menu_ui))
        .add_system_set(SystemSet::on_enter(AppState::PreConnect).with_system(init_network_client))
        // Realtime Gameplay Loop
        .add_system_to_stage(Stage::Connection, events::connect_event)
        .add_system_to_stage(Stage::Disconnection, events::disconnect_event)
        .add_system_to_stage(Stage::ReceiveEvents, events::spawn_entity_event)
        .add_system_to_stage(Stage::ReceiveEvents, events::insert_component_event)
        .add_system_to_stage(Stage::ReceiveEvents, events::update_component_event)
        .add_system_to_stage(Stage::ReceiveEvents, events::receive_message_event)
        .add_system_to_stage(Stage::Frame, input)
        .add_system_to_stage(Stage::PostFrame, sync)
        // Gameplay Loop on Tick
        .add_system_to_stage(Stage::Tick, tick)
        // Run App
        .run();
}
