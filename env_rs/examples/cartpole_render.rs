use bevy::{prelude::*, window::PresentMode};
use bevy_rapier2d::prelude::*;
use env_rs::{CartPole, Environment};
use rand::random;

#[derive(Resource)]
struct SimulationState {
    steps: usize,
    episode: usize,
    total_reward: f32,
}

#[derive(Component)]
struct StatsText;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "CartPole Environment".into(),
                resolution: (1024., 768.).into(),
                present_mode: PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin {
            mode: DebugRenderMode::all(),
            ..default()
        })
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .insert_resource(CartPole::new())
        .insert_resource(SimulationState {
            steps: 0,
            episode: 0,
            total_reward: 0.0,
        })
        .add_startup_system(setup_camera)
        .add_startup_system(setup_physics)
        .add_startup_system(setup_ui)
        .add_systems(Update, (simulate, update_ui))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 100.0),
        ..default()
    });
}

fn setup_physics(mut commands: Commands) {
    // Cart
    let cart_entity = commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.2, 0.4, 0.8),
                    custom_size: Some(Vec2::new(50.0, 30.0)),
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            },
            RigidBody::Dynamic,
            Collider::cuboid(25.0, 15.0),
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
                transform: Transform::from_xyz(0.0, 50.0, 0.0),
                ..default()
            },
            RigidBody::Dynamic,
            Collider::capsule_y(50.0, 2.5),
        ))
        .id();

    // Joint
    let revolute_joint = RevoluteJointBuilder::new()
        .local_anchor1(Vec2::new(0.0, 15.0))
        .local_anchor2(Vec2::new(0.0, -50.0))
        .build();

    commands.spawn(ImpulseJoint::new(cart_entity, revolute_joint));

    // Ground
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.2, 0.8, 0.2),
            custom_size: Some(Vec2::new(1000.0, 2.0)),
            ..default()
        },
        transform: Transform::from_xyz(0.0, -100.0, 0.0),
        ..default()
    });
}

fn setup_ui(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_sections([TextSection::new(
            "Stats\n",
            TextStyle {
                font_size: 20.0,
                color: Color::WHITE,
                ..default()
            },
        )])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
        StatsText,
    ));
}

fn simulate(
    mut cart_pole: ResMut<CartPole>,
    mut sim_state: ResMut<SimulationState>,
    time: Res<Time>,
) {
    if time.elapsed_seconds() > 0.016 {
        let action = if random::<f32>() > 0.5 { 1 } else { 0 };
        let (_, reward, done) = cart_pole.step(action);

        sim_state.steps += 1;
        sim_state.total_reward += reward;

        if done {
            println!(
                "Episode {} finished after {} steps with total reward {}",
                sim_state.episode, sim_state.steps, sim_state.total_reward
            );
            cart_pole.reset();
            sim_state.steps = 0;
            sim_state.episode += 1;
            sim_state.total_reward = 0.0;
        }
    }
}

fn update_ui(
    sim_state: Res<SimulationState>,
    cart_pole: Res<CartPole>,
    mut query: Query<&mut Text, With<StatsText>>,
) {
    let state = cart_pole.get_physics_state();
    for mut text in query.iter_mut() {
        text.sections[0].value = format!(
            "Episode: {}\nSteps: {}\nTotal Reward: {:.2}\nCart Pos: {:.2}\nPole Angle: {:.2}°",
            sim_state.episode,
            sim_state.steps,
            sim_state.total_reward,
            state[0],
            state[2].to_degrees(),
        );
    }
}
