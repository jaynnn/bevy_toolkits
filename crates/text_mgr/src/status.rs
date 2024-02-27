use bevy::{
    asset::AssetServer, ecs::{component::Component, system::Query}, prelude::*, utils::default,
    
    diagnostic::{FrameTimeDiagnosticsPlugin, DiagnosticsStore},
};

#[derive(Component)]
pub struct StatusText;

pub fn startup_game_status(
    asset_server: Res<AssetServer>,
    mut cmds: Commands,
) {
    let font = asset_server.load::<Font>("fonts/Arimo-Regular.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 12.36,
        color: Color::ORANGE_RED,
    };
    let text_justification = JustifyText::Center;
    cmds.spawn((
        TextBundle::from_sections([
            TextSection::new("FPS:", text_style.clone()),
            TextSection::new("00.00", text_style.clone()),
            TextSection::new(" FPS_AVG:", text_style.clone()),
            TextSection::new("00.00", text_style.clone()),
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
        StatusText
    ));
}

pub fn update_game_status(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<StatusText>>,
) {
    for mut text in query.iter_mut() {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(raw) = fps.value() {
                text.sections[1].value = format!("{raw:.2}");
            }
            if let Some(avg) = fps.average() {
                text.sections[3].value = format!("{avg:.2}");
            }
        }
    }
}