use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::rapier::na::Vector2;

mod ball;
use ball::*;

mod walls;
use walls::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    SettingUpVariables,
    SettingUpWorld,
    SetUp,
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Pinball2d".to_string(),
            width: 640.0,
            height: 640.0,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(ShapePlugin)
        .add_plugin(WallsPlugin)
        .add_plugin(BallPlugin)
        .add_state(AppState::SettingUpVariables)
        .add_system_set(SystemSet::on_update(AppState::SettingUpVariables).with_system(setup))
        .run();
}

fn setup(
    mut commands: Commands,
    mut rapier_config: ResMut<RapierConfiguration>,
    mut app_state: ResMut<State<AppState>>,
) {
    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d());

    rapier_config.gravity = Vector2::zeros();
    rapier_config.scale = 640.0 / 1.0;

    app_state.set(AppState::SettingUpWorld).unwrap();
}
