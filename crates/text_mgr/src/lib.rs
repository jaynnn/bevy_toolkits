use bevy::{
    app::{Startup, Update}, prelude::{
        App, Plugin
    },
    diagnostic::FrameTimeDiagnosticsPlugin,
};
use status::{startup_game_status, update_game_status};

mod status;

pub struct TextMgrPlugin;

impl Plugin for TextMgrPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(
            FrameTimeDiagnosticsPlugin::default(),
        )
        .add_systems(Startup, startup_game_status)
        .add_systems(Update, update_game_status);
    }
}
