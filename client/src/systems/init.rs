use bevy::prelude::*;
use naia_bevy_client::Client;

use shared::{
    protocol::{Auth, Protocol},
    Channels,
};

use crate::{
    app::{AppState, ConnectionAddress},
    resources::Global,
};

pub fn init_network_client(
    mut commands: Commands,
    mut client: Client<Protocol, Channels>,
    mut app_state: ResMut<State<AppState>>,
    conn_addr: Res<ConnectionAddress>,
) {
    if let Some(addr) = &*conn_addr {
        info!("Naia Bevy Client starting");

        client.auth(Auth::new("charlie", "12345"));
        client.connect(format!("http://{}:3478", addr).as_str());

        // Setup Camera
        commands.spawn_bundle(OrthographicCameraBundle::new_2d());

        // Setup Colors
        app_state
            .set(AppState::InGame)
            .expect("Set AppState::InGame failed")
    } else {
        panic!("No connection addr when initting network client");
    }
}
