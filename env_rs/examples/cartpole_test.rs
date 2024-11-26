use env_rs::{CartPole, Environment};

fn main() {
    let mut env = CartPole::new();

    // Run a few episodes
    for episode in 0..3 {
        println!("\nEpisode {}", episode);
        let mut state = env.reset();
        let mut total_reward = 0.0;
        let mut steps = 0;

        // Print initial state
        println!(
            "Initial state: cart_pos={:.3}, cart_vel={:.3}, pole_angle={:.3}, pole_vel={:.3}",
            state[0], state[1], state[2], state[3]
        );

        loop {
            // Take random action
            let action = if rand::random::<f32>() > 0.5 { 1 } else { 0 };

            // Step environment
            let (new_state, reward, done) = env.step(action);

            // Update stats
            total_reward += reward;
            steps += 1;

            // Print state every 10 steps
            if steps % 10 == 0 {
                println!(
                    "Step {}: cart_pos={:.3}, cart_vel={:.3}, pole_angle={:.3}, pole_vel={:.3}",
                    steps, new_state[0], new_state[1], new_state[2], new_state[3]
                );
            }

            if done {
                println!(
                    "Episode finished after {} steps with total reward {}",
                    steps, total_reward
                );
                break;
            }

            state = new_state;
        }
    }
}
