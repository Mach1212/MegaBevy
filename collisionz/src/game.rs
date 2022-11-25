use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use kayak_ui::prelude::FontMapping;
use kayak_ui::{KayakUICameraPlugin, UICameraBundle};

mod ui;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum State {
    MainMenu,
    InGame,
    Paused,
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(KayakUICameraPlugin)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_state(State::MainMenu)
        .run()
}

fn setup_ui(
    mut commands: Commands,
    mut font_mapping: ResMut<FontMapping>,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn(UICameraBundle::new())
        .insert(Name::new("UI Camera"));
    font_mapping.set_default(asset_server.load("roboto.kayak_font"));
    kayak_ui::prelude::BevyContext::new(|context| {})
}
