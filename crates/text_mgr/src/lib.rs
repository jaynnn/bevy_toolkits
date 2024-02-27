use bevy::{
    app::{Startup, Update}, prelude::{
        App, Plugin
    },
    diagnostic::FrameTimeDiagnosticsPlugin,
};
use fps::{startup_fps, update_fps};

mod fps;

pub struct TextMgrPlugin;

impl Plugin for TextMgrPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(
            FrameTimeDiagnosticsPlugin::default(),
        )
        .add_systems(Startup, startup_fps)
        .add_systems(Update, update_fps);
    }
}
