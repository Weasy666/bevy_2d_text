use bevy::{input::mouse::{MouseMotion, MouseWheel}, prelude::*};


pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .init_resource::<State>()
            .add_startup_system(setup.system())
            .add_system(keyboard_motion_system.system())
            .add_system(mouse_motion_system.system())
            .add_system(mouse_zoom_system.system());
    }
}

fn setup(commands: &mut Commands) {
    commands
        .spawn(Camera2dBundle::default())
        .with(MainCamera::default());
}

#[derive(Default)]
pub struct MainCamera;

fn keyboard_motion_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut cam_query: Query<(&mut MainCamera, &mut Transform)>,
) {
    let motion_sense = 10.0;
    for (_camera, mut transform) in cam_query.iter_mut() {
    let mut motion = Vec3::zero();
        if keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up) {
            motion += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
            motion += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down) {
            motion += Vec3::new(0.0, -1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
            motion += Vec3::new(1.0, 0.0, 0.0);
        }
        transform.translation = transform.translation + motion * motion_sense * transform.scale.x;
    }
}

#[derive(Default)]
struct State {
    mouse_motion_event_reader: EventReader<MouseMotion>,
    mouse_scroll_event_reader: EventReader<MouseWheel>,
}

fn mouse_motion_system(
    mut state: ResMut<State>,
    mouse_button_input: Res<Input<MouseButton>>,
    mouse_motion_events: Res<Events<MouseMotion>>,
    mut cam_query: Query<(&mut MainCamera, &mut Transform)>,
) {
    let motion_sense = 2.0;
    for (_camera, mut transform) in cam_query.iter_mut() {
        if mouse_button_input.pressed(MouseButton::Middle) {
            let mut delta: Vec2 = Vec2::zero();
            for event in state.mouse_motion_event_reader.iter(&mouse_motion_events) {
                delta += event.delta;
            }
            if delta == Vec2::zero() {
                return;
            }
            let motion = Vec3::new(-delta.x, delta.y, 0.0) * motion_sense;
            transform.translation += motion;
        }
    }
}

fn mouse_zoom_system(
    mut state: ResMut<State>,
    mouse_scroll_events: Res<Events<MouseWheel>>,
    mut cam_query: Query<(&mut MainCamera, &mut Transform)>,
) {
    let zoom_sense = 10.0;
    for (_camera, mut transform) in cam_query.iter_mut() {
        let mut zoom_delta: f32 = 0.0;
        for event in state.mouse_scroll_event_reader.iter(&mouse_scroll_events) {
            zoom_delta = event.y;
        }

        if zoom_delta != 0.0 {
            let zoom = zoom_delta / zoom_sense;
            transform.scale = Vec3::new((transform.scale.x - zoom).min(4.5).max(0.5), (transform.scale.y - zoom).min(4.5).max(0.5), 1.0);
        }
    }
}
