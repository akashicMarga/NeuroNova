use crate::Environment;
use rapier2d::prelude::*;

#[derive(Default)]
pub struct CartPole {
    // Physics
    rigid_body_set: RigidBodySet,
    collider_set: ColliderSet,
    physics_pipeline: PhysicsPipeline,
    gravity: Vector<Real>,
    integration_parameters: IntegrationParameters,
    island_manager: IslandManager,
    broad_phase: BroadPhase,
    narrow_phase: NarrowPhase,
    // Changed to ImpulseJointSet
    joint_set: ImpulseJointSet,
    ccd_solver: CCDSolver,
    query_pipeline: QueryPipeline,

    // Handles
    cart_handle: RigidBodyHandle,
    pole_handle: RigidBodyHandle,

    // Environment parameters
    max_steps: usize,
    current_step: usize,
}

impl CartPole {
    pub fn get_physics_state(&self) -> [f32; 4] {
        let cart = self.rigid_body_set.get(self.cart_handle).unwrap();
        let pole = self.rigid_body_set.get(self.pole_handle).unwrap();

        let cart_pos = cart.translation().x;
        let cart_vel = cart.linvel().x;
        let pole_angle = pole.rotation().angle();
        let pole_vel = pole.angvel();

        [
            cart_pos as f32,
            cart_vel as f32,
            pole_angle as f32,
            pole_vel as f32,
        ]
    }

    pub fn new() -> Self {
        let mut rigid_body_set = RigidBodySet::new();
        let mut collider_set = ColliderSet::new();

        // Create physics world with gravity
        let gravity = vector![0.0, -9.81];

        // Create cart (rigid body with horizontal movement only)
        let cart_body = RigidBodyBuilder::dynamic()
            .translation(vector![0.0, 0.0])
            .linear_damping(0.5)
            .lock_rotations()
            .build();
        let cart_collider = ColliderBuilder::cuboid(0.5, 0.25).build();
        let cart_handle = rigid_body_set.insert(cart_body);
        collider_set.insert_with_parent(cart_collider, cart_handle, &mut rigid_body_set);

        // Create pole
        let pole_body = RigidBodyBuilder::dynamic()
            .translation(vector![0.0, 0.5])
            .build();
        let pole_collider = ColliderBuilder::capsule_y(0.5, 0.05).build();
        let pole_handle = rigid_body_set.insert(pole_body);
        collider_set.insert_with_parent(pole_collider, pole_handle, &mut rigid_body_set);

        // Create joint between cart and pole
        let joint = RevoluteJointBuilder::new()
            .local_anchor1(point![0.0, 0.25])
            .local_anchor2(point![0.0, -0.5]);

        let mut joint_set = ImpulseJointSet::new();
        joint_set.insert(cart_handle, pole_handle, joint, true);

        Self {
            rigid_body_set,
            collider_set,
            physics_pipeline: PhysicsPipeline::new(),
            gravity,
            integration_parameters: IntegrationParameters::default(),
            island_manager: IslandManager::new(),
            broad_phase: BroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            joint_set,
            ccd_solver: CCDSolver::new(),
            query_pipeline: QueryPipeline::new(),
            cart_handle,
            pole_handle,
            max_steps: 500,
            current_step: 0,
        }
    }

    pub fn get_state(&self) -> [f32; 4] {
        let cart = self.rigid_body_set.get(self.cart_handle).unwrap();
        let pole = self.rigid_body_set.get(self.pole_handle).unwrap();

        let cart_pos = cart.translation().x;
        let cart_vel = cart.linvel().x;
        let pole_angle = pole.rotation().angle();
        let pole_vel = pole.angvel();

        [
            cart_pos as f32,
            cart_vel as f32,
            pole_angle as f32,
            pole_vel as f32,
        ]
    }
}

impl Environment for CartPole {
    type State = [f32; 4]; // [cart_position, cart_velocity, pole_angle, pole_angular_velocity]
    type Action = i32; // 0: left force, 1: right force

    fn reset(&mut self) -> Self::State {
        // Reset physics bodies to initial positions
        let cart = self.rigid_body_set.get_mut(self.cart_handle).unwrap();
        cart.set_translation(vector![0.0, 0.0], true);
        cart.set_linvel(vector![0.0, 0.0], true);

        let pole = self.rigid_body_set.get_mut(self.pole_handle).unwrap();
        pole.set_translation(vector![0.0, 0.5], true);
        pole.set_rotation(nalgebra::UnitComplex::identity(), true);
        pole.set_angvel(0.0, true);

        self.current_step = 0;
        self.get_state()
    }

    fn step(&mut self, action: Self::Action) -> (Self::State, f32, bool) {
        // Apply force based on action
        let force = if action == 0 { -10.0 } else { 10.0 };
        let cart = self.rigid_body_set.get_mut(self.cart_handle).unwrap();
        // Changed apply_force to add_force
        cart.add_force(vector![force, 0.0], true);

        // Step physics
        self.physics_pipeline.step(
            &self.gravity,
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_body_set,
            &mut self.collider_set,
            &mut self.joint_set,
            &mut MultibodyJointSet::new(),
            &mut self.ccd_solver,
            Some(&mut self.query_pipeline),
            &(),
            &(),
        );

        self.current_step += 1;
        let state = self.get_state();

        // Calculate reward and done condition
        let x = state[0];
        let theta = state[2];

        let done = x < -2.4
            || x > 2.4
            || theta < -0.209
            || theta > 0.209
            || self.current_step >= self.max_steps;

        let reward = if !done { 1.0 } else { 0.0 };

        (state, reward, done)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cartpole() {
        let mut env = CartPole::new();
        let initial_state = env.reset();
        assert_eq!(initial_state.len(), 4);

        let (next_state, reward, done) = env.step(1);
        assert_eq!(next_state.len(), 4);
        assert!(!done);
        assert_eq!(reward, 1.0);
    }
}
