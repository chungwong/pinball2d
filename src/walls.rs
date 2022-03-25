use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;
use nalgebra::Point2;

use crate::AppState;

pub struct WallsPlugin;

impl Plugin for WallsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::SettingUpWorld).with_system(spawn_walls.label("walls")),
        );
    }
}

fn spawn_walls(
    mut commands: Commands,
    rapier_config: Res<RapierConfiguration>,
    windows: Res<Windows>,
    mut app_state: ResMut<State<AppState>>,
) {
    let window = windows.get_primary().unwrap();

    //Spawn outer wall
    //Spawn top and bottom wall
    let shape_top_and_bottom_wall = shapes::Rectangle {
        extents: Vec2::new(window.width(), 5.),
        origin: shapes::RectangleOrigin::Center,
    };

    //Spawn bottom wall
    let bottom_wall_pos: Point2<f32> = Point2::new(0.0, -320.0 / rapier_config.scale);
    commands
        .spawn()
        .insert_bundle(GeometryBuilder::build_as(
            &shape_top_and_bottom_wall,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::TEAL),
                outline_mode: StrokeMode::color(Color::TEAL),
            },
            Transform::default(),
        ))
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static.into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(
                shape_top_and_bottom_wall.extents.x / rapier_config.scale / 2.0,
                shape_top_and_bottom_wall.extents.y / rapier_config.scale / 2.0,
            )
            .into(),
            position: bottom_wall_pos.into(),
            material: ColliderMaterial {
                restitution: 1.0,
                friction: 0.0,
                ..Default::default()
            }
            .into(),
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete);

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::RED,
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(315.0, 0.0, 0.0),
            scale: Vec3::new(2.0, 2.0, 0.0),
            ..Default::default()
        },
        ..Default::default()
    });

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::RED,
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(-315.0, 0.0, 0.0),
            scale: Vec3::new(2.0, 2.0, 0.0),
            ..Default::default()
        },
        ..Default::default()
    });

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::RED,
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, -315.0, 0.0),
            scale: Vec3::new(2.0, 2.0, 0.0),
            ..Default::default()
        },
        ..Default::default()
    });

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::RED,
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 315.0, 0.0),
            scale: Vec3::new(2.0, 2.0, 0.0),
            ..Default::default()
        },
        ..Default::default()
    });
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::RED,
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, -315.0, 0.0),
            scale: Vec3::new(2.0, 2.0, 0.0),
            ..Default::default()
        },
        ..Default::default()
    });

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::RED,
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 160.0, 2.0),
            scale: Vec3::new(2.0, 2.0, 0.0),
            ..Default::default()
        },
        ..Default::default()
    });

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::RED,
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, -160.0, 2.0),
            scale: Vec3::new(2.0, 2.0, 0.0),
            ..Default::default()
        },
        ..Default::default()
    });

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::RED,
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(160.0, 0.0, 2.0),
            scale: Vec3::new(2.0, 2.0, 0.0),
            ..Default::default()
        },
        ..Default::default()
    });

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::RED,
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(-160.0, 0.0, 2.0),
            scale: Vec3::new(2.0, 2.0, 0.0),
            ..Default::default()
        },
        ..Default::default()
    });

    //Spawn top wall
    let top_wall_pos: Point2<f32> = Point2::new(0.0, 320.0 / rapier_config.scale);
    commands
        .spawn()
        .insert_bundle(GeometryBuilder::build_as(
            &shape_top_and_bottom_wall,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::TEAL),
                outline_mode: StrokeMode::color(Color::TEAL),
            },
            Transform::default(),
        ))
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static.into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(
                shape_top_and_bottom_wall.extents.x / rapier_config.scale / 2.0,
                shape_top_and_bottom_wall.extents.y / rapier_config.scale / 2.0,
            )
            .into(),
            position: top_wall_pos.into(),
            material: ColliderMaterial {
                restitution: 1.0,
                friction: 0.0,
                ..Default::default()
            }
            .into(),
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete);

    //Spawn left and right wall
    let shape_left_and_right_wall = shapes::Rectangle {
        extents: Vec2::new(5., window.height()),
        origin: shapes::RectangleOrigin::Center,
    };

    //Spawn left wall
    let left_wall_pos: Point2<f32> = Point2::new(-320.0 / rapier_config.scale, 0.0);
    commands
        .spawn()
        .insert_bundle(GeometryBuilder::build_as(
            &shape_left_and_right_wall,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::TEAL),
                outline_mode: StrokeMode::color(Color::TEAL),
            },
            Transform::default(),
        ))
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static.into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(
                shape_left_and_right_wall.extents.x / rapier_config.scale / 2.0,
                shape_left_and_right_wall.extents.y / rapier_config.scale / 2.0,
            )
            .into(),
            position: left_wall_pos.into(),
            material: ColliderMaterial {
                restitution: 1.0,
                friction: 0.0,
                ..Default::default()
            }
            .into(),
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete);

    //Spawn right wall
    let right_wall_pos: Point2<f32> = Point2::new(320.0 / rapier_config.scale, 0.0);
    commands
        .spawn()
        .insert_bundle(GeometryBuilder::build_as(
            &shape_left_and_right_wall,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::TEAL),
                outline_mode: StrokeMode::color(Color::TEAL),
            },
            Transform::default(),
        ))
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static.into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(
                shape_left_and_right_wall.extents.x / rapier_config.scale / 2.0,
                shape_left_and_right_wall.extents.y / rapier_config.scale / 2.0,
            )
            .into(),
            position: right_wall_pos.into(),
            material: ColliderMaterial {
                restitution: 1.0,
                friction: 0.0,
                ..Default::default()
            }
            .into(),
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete);

    app_state.set(AppState::SetUp).unwrap();
}
