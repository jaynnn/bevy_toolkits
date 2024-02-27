use bevy::prelude::*;

use text_mgr::TextMgrPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins((DefaultPlugins, TextMgrPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();
}

fn setup(
    mut cmds: Commands
) {
    cmds.spawn(Camera2dBundle::default());
}

fn update(

) {

}