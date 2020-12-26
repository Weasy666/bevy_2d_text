use bevy::{diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin}, prelude::*};
use camera::CameraPlugin;

mod camera;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(CameraPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_startup_system(setup.system())
        .add_system(display_framerate.system())
        .run();
}

struct FpsState;

fn display_framerate(
    diagnostics: Res<Diagnostics>,
    mut query: Query<&mut Text, With<FpsState>>
) {

    let mut average_fps = None;
    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        average_fps = fps.average();
    }

    for mut text in &mut query.iter_mut() {
        if let Some(average_fps) = average_fps {
            text.value = format!("FPS: {:.2}", average_fps);
        }
    }
}

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(CameraUiBundle::default())
        .spawn(NodeBundle {
            material: materials.add(Color::NONE.into()),
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::FlexStart,
                ..Default::default()
            },
            ..Default::default()
        }).with_children(|parent| {
            parent
                .spawn(TextBundle {
                    style: Style {
                        align_self: AlignSelf::FlexEnd,
                        margin: Rect {
                            left: Val::Undefined,
                            right: Val::Px(10.0),
                            top: Val::Undefined,
                            bottom: Val::Undefined,
                        },
                        ..Default::default()
                    },
                    text: Text {
                        value: "FPS:".to_string(),
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        style: TextStyle {
                            font_size: 30.0,
                            color: Color::WHITE,
                            ..Default::default()
                        },
                    },
                    ..Default::default()
                })
                .with(FpsState);
        });

    commands
        .spawn(Text2dBundle {
            text: Text {
                value: ".".to_string(),
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                style: TextStyle {
                    font_size: 10.0,
                    color: Color::WHITE,
                    alignment: TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Center,
                    },
                },
            },
            ..Default::default()
        })
        .with_children(|parent| {
            // Create 200 Text2dBundles in a grid layout
            for x in (-10000..=10000).step_by(100) {
                for y in (-10000..=10000).step_by(100) {
                    parent
                        .spawn(Text2dBundle {
                            text: Text {
                                value: format!("({}, {})", x, y),
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                style: TextStyle {
                                    font_size: 10.0,
                                    color: Color::WHITE,
                                    alignment: TextAlignment {
                                        vertical: VerticalAlign::Center,
                                        horizontal: HorizontalAlign::Center,
                                    },
                                },
                            },
                            transform: Transform::from_translation(Vec3::new(x as f32, y as f32, 0.0)),
                            ..Default::default()
                        });
                }
            }
        });
}
