use bevy::{
    asset::AssetServer, ecs::{component::Component, system::Query}, prelude::*, utils::default,
    
    diagnostic::{FrameTimeDiagnosticsPlugin, DiagnosticsStore},
};

#[derive(Component)]
pub struct FpsText;

pub fn startup_fps(
    asset_server: Res<AssetServer>,
    mut cmds: Commands,
) {
    cmds.spawn(Camera2dBundle::default());
    let font = asset_server.load::<Font>("fonts/Arimo-Regular.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 12.36,
        color: Color::ORANGE_RED,
    };
    let text_justification = JustifyText::Center;
    cmds.spawn((
        TextBundle::from_sections([
            TextSection::new("FPS: ", text_style.clone()),
            TextSection::new("00.00", text_style),
        ])
        .with_text_justify(text_justification)
        .with_style(Style {
            position_type: PositionType::Absolute,
            margin: UiRect {
                left: Val::Px(10.0),
                top: Val::Px(10.0),
                ..default()
            },
            ..default()
        }),
        FpsText
    ));
}

pub fn update_fps(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                text.sections[1].value = format!("{value:.2}");
            }
        }
    }
}