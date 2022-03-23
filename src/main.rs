use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::rapier::na::Vector2;

mod ball;
use ball::*;

mod walls;
use walls::*;

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
        .add_plugin(BallPlugin)
        .add_plugin(WallsPlugin)
        .add_plugin(ShapePlugin)
        .add_startup_system(setup.label("main_setup"))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .run();
}

fn setup(mut commands: Commands, mut rapier_config: ResMut<RapierConfiguration>) {
    // Set gravity to x and spawn camera.
    //rapier_config.gravity = Vector2::zeros();
    rapier_config.gravity = Vector2::new(0.0, 0.0);

    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d());

    rapier_config.scale = 640.0 / 1.0;
}

