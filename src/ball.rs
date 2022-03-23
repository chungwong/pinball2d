use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;
use nalgebra::Point2;

pub struct BallPlugin;

const BALL_RADIUS: f32 = 30.;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_ball.label("ball"))
        // .add_system(display_events)
        ;
    }
}

#[derive(Component, Debug)]
struct Ball;

fn spawn_ball(mut commands: Commands, rapier_config: ResMut<RapierConfiguration>) {
    let ball_pos: Point2<f32> = Point2::new(0.0, -160.0 / rapier_config.scale);

    let shape_ball = shapes::Circle {
        // radius: BALL_RADIUS * rapier_config.scale,
        radius: BALL_RADIUS,
        center: Vec2::ZERO,
    };

    commands
        .spawn()
        .insert_bundle(GeometryBuilder::build_as(
            &shape_ball,
            DrawMode::Fill(FillMode::color(Color::TEAL)),
            Transform::default(),
        ))
        .insert_bundle(RigidBodyBundle {
            ccd: RigidBodyCcd {
                ccd_enabled: true,
                ..Default::default()
            }
            .into(),
            velocity: RigidBodyVelocity {
                linvel: Vec2::new(1.0, 1.0).into(),
                angvel: 0.0,
            }
            .into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::ball(shape_ball.radius / rapier_config.scale).into(),
            collider_type: ColliderType::Solid.into(),
            flags: (ActiveEvents::CONTACT_EVENTS).into(),
            position: ball_pos.into(),
            material: ColliderMaterial {
                restitution: 1.0,
                friction: 0.0,
                ..Default::default()
            }
            .into(),
            ..ColliderBundle::default()
        })
        .insert(ColliderPositionSync::Discrete)
        .insert(Ball);
}

/* A system that displays the events. */
fn display_events(
    mut contact_events: EventReader<ContactEvent>,
    ball: Query<&Transform, With<Ball>>
) {
    for contact_event in contact_events.iter() {
        if matches!(contact_event, ContactEvent::Started(..)) {
            let b = ball.get_single().unwrap();
            println!("Received contact event: {:?} {:?}", contact_event, b.translation);
        }
    }
}
