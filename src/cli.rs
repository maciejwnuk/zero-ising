use clap::{
    Parser,
    ValueEnum
};

use crate::{
    graph::Topology,
    ising::States
};

#[derive(Clone, ValueEnum)]
pub enum TemperatureDirection {
    Asc,
    Desc
}

#[derive(Parser)]
#[command(name = "Zero-Ising")]
#[command(version = "0.0.1")]
#[command(about = "
\u{001B}[1m\u{001B}[4mZero-Ising\u{001B}[22m\u{001B}[24m

Ising simulation model with zero state.
Project done as engineering thesis at WUT, PL.\
", long_about = None)]
pub struct Args {
    #[arg(value_enum)]
    spins: States,

    #[arg(value_enum)]
    topology: Topology,

    #[arg(
        short,
        long,
        default_value = "1000",
        help = "Simulation steps"
    )]
    steps: usize,

    #[arg(
        long,
        value_name = "STEPS",
        default_value = "10",
        help = "Modulus-step for data acquisition"
    )]
    probe_step: usize,

    #[arg(
        short = 'S',
        long,
        default_value = "1000",
        help = "Nodes in chain/graph"
    )]
    size: usize,

    #[arg(
        short,
        long,
        value_name = "FIELD",
        default_value = "1.0",
        help = "Internal field (J) strength"
    )]
    internal: f64,

    #[arg(
        short,
        long,
        value_name = "FIELD",
        default_value = "1.0",
        help = "External field (h) strength"
    )]
    external: f64,

    #[arg(
        long,
        value_name = "TEMPERATURE",
        default_value = "0.0",
        help = "Temperature lower bound"
    )]
    temp_low: f64,


    #[arg(
        long,
        value_name = "TEMPERATURE",
        default_value = "10.0",
        help = "Temperature upper bound"
    )]
    temp_high: f64,

    #[arg(
        long,
        value_name = "STEPS",
        default_value = "100",
        help = "Temperature points"
    )]
    temp_steps: usize,

    #[arg(
        long,
        value_enum,
        value_name = "DIRECTION",
        default_value = "asc",
        help = "Temperature sweep direction"
    )]
    temp_dir: TemperatureDirection,

    #[arg(
        long,
        value_name = "STEPS",
        default_value = "1000",
        help = "Thermalization (initialization) steps"
    )]
    therm_steps: usize
}
