pub mod cartpole;
pub mod render;

pub use cartpole::CartPole;

pub trait Environment {
    type State;
    type Action;

    fn reset(&mut self) -> Self::State;
    fn step(&mut self, action: Self::Action) -> (Self::State, f32, bool);
    fn render(&self) {}
}
