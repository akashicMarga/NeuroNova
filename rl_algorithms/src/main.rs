/*
export PYO3_PYTHON="$CONDA_PREFIX/bin/python"
export PYTHON_SYS_EXECUTABLE="$CONDA_PREFIX/bin/python"
export DYLD_LIBRARY_PATH="$CONDA_PREFIX/lib:$DYLD_LIBRARY_PATH"
*/

use candle::Result;
use clap::{Parser, Subcommand};

mod gym_env;
mod vec_gym_env;

mod ddpg;
mod dqn;
mod policy_gradient;

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Pg,
    Ddpg,
    Dqn,
}

fn main() -> Result<()> {
    let args = Args::parse();
    match args.command {
        Command::Pg => policy_gradient::run()?,
        Command::Ddpg => ddpg::run()?,
        Command::Dqn => dqn::run()?,
    }
    Ok(())
}
