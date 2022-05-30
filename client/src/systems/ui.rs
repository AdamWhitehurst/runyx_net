use bevy::prelude::*;
use bevy_egui::{
    egui::{self, menu, RichText},
    EguiContext,
};
use lazy_static::lazy_static;
use regex::Regex;

use crate::app::{AppState, ConnectionAddress};

lazy_static! {
        static ref IP_PORT: Regex = Regex::new(r"^(([0-9]|[1-9][0-9]|1[0-9]{2}|2[0-4][0-9]|25[0-5])\.){3}([0-9]|[1-9][0-9]|1[0-9]{2}|2[0-4][0-9]|25[0-5])?:?[0-9]+$").unwrap();
}

#[derive(Default)]
pub struct MenuState {
    ip_str: String,
    error_msg: String,
}

pub fn menu_ui(
    mut egui_context: ResMut<EguiContext>,
    mut menu_state: Local<MenuState>,
    mut conn_addr: ResMut<ConnectionAddress>,
    mut app_state: ResMut<State<AppState>>,
) {
    let margin = egui::style::Margin {
        top: 100.0,
        bottom: 100.0,
        left: 100.0,
        right: 100.0,
    };

    egui::CentralPanel::default()
        .frame(egui::Frame {
            outer_margin: margin,
            fill: egui::Color32::DARK_GRAY,
            ..Default::default()
        })
        .show(egui_context.ctx_mut(), |ui| {
            ui.heading("Menu");
            ui.add_space(100.0);
            ui.vertical_centered_justified(|ui| {
                ui.horizontal_top(|ui| {
                    ui.label("IP:");
                    ui.text_edit_singleline(&mut menu_state.ip_str);

                    if ui.button("Connect").clicked() {
                        if IP_PORT.is_match(&menu_state.ip_str) {
                            *conn_addr = Some(menu_state.ip_str.clone());
                            app_state
                                .set(AppState::PreConnect)
                                .expect("Set AppState::PreConnect Failed");
                        } else {
                            menu_state.error_msg = "Invalid IP".to_string();
                        }
                    };
                });
                ui.horizontal_top(|ui| {
                    ui.label(menu_state.error_msg.clone());
                });
            });
        });
}
