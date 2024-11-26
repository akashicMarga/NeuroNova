use crate::CartPole;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct Cart;

#[derive(Component)]
pub struct Pole;

impl Resource for CartPole {}

pub struct CartPoleRenderPlugin;

impl Plugin for CartPoleRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_objects)
            .add_systems(Update, sync_physics);
    }
}

fn spawn_objects(mut commands: Commands) {
    // Ground line
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.2, 0.8, 0.2),
            custom_size: Some(Vec2::new(800.0, 2.0)),
            ..default()
        },
        transform: Transform::from_xyz(0.0, -50.0, 0.0),
        ..default()
    });

    // Cart
    let cart_entity = commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.2, 0.4, 0.8),
                    custom_size: Some(Vec2::new(50.0, 30.0)),
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                visibility: Visibility::Visible,
                ..default()
            },
            RigidBody::Dynamic,
            Collider::cuboid(25.0, 15.0),
            Friction::coefficient(0.1),
            Restitution::coefficient(0.0),
            Cart,
        ))
        .id();

    // Pole
    let pole_entity = commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.8, 0.2, 0.2),
                    custom_size: Some(Vec2::new(5.0, 100.0)),
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 50.0, 1.1),
                visibility: Visibility::Visible,
                ..default()
            },
            RigidBody::Dynamic,
            Collider::capsule_y(50.0, 2.5),
            Friction::coefficient(0.0),
            Restitution::coefficient(0.0),
            Pole,
        ))
        .id();

    // Create joint
    let revolute_joint = RevoluteJointBuilder::new()
        .local_anchor1(Vec2::new(0.0, 15.0))
        .local_anchor2(Vec2::new(0.0, -50.0))
        .build();

    commands.spawn(ImpulseJoint::new(cart_entity, revolute_joint));

    // Grid lines
    for i in -5..=5 {
        let x = i as f32 * 100.0;
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0.5, 0.5, 0.5, 0.2),
                custom_size: Some(Vec2::new(2.0, 600.0)),
                ..default()
            },
            transform: Transform::from_xyz(x, 0.0, 0.0),
            visibility: Visibility::Visible,
            ..default()
        });
    }
}

fn sync_physics(
    cart_pole: Res<CartPole>,
    mut cart_query: Query<&mut Transform, With<Cart>>,
    mut pole_query: Query<&mut Transform, (With<Pole>, Without<Cart>)>,
) {
    let state = cart_pole.get_physics_state();

    if let Ok(mut cart_transform) = cart_query.get_single_mut() {
        cart_transform.translation.x = state[0] * 100.0;
    }

    if let Ok(mut pole_transform) = pole_query.get_single_mut() {
        pole_transform.translation.x = state[0] * 100.0;
        pole_transform.rotation = Quat::from_rotation_z(state[2]);
    }
}
