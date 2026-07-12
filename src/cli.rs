use std::path::PathBuf;

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
#[command(name = "ising")]
#[command(version = "0.0.1")]
#[command(about = "
\u{001B}[1m\u{001B}[4mZero-Ising\u{001B}[22m\u{001B}[24m

Ising simulation model with zero state.
Project done as engineering thesis at WUT, PL.\
", long_about = None)]
pub struct Args {
    #[arg(
        value_enum,
        help = "Spin states of the model"
    )]
    pub spins: States,

    #[arg(
        value_enum,
        help = "Topology of graph underlying the model"
    )]
    pub topology: Topology,

    #[arg(
        value_name = "FILE",
        default_value = "output.csv",
        help = "Output data filename"
    )]
    pub output: PathBuf,

    #[arg(
        short,
        long,
        default_value = "100",
        help = "Simulation steps at given temperature"
    )]
    pub steps: usize,

    #[arg(
        long,
        value_name = "STEPS",
        default_value = "10",
        help = "Modulus-step for data acquisition"
    )]
    pub probe_step: usize,

    #[arg(
        short = 'S',
        long,
        default_value = "1000",
        help = "Nodes in chain/graph"
    )]
    pub size: usize,

    #[arg(
        short,
        long,
        value_name = "FIELD",
        default_value = "1.0",
        help = "Internal field (J) strength"
    )]
    pub internal: f64,

    #[arg(
        short,
        long,
        value_name = "FIELD",
        default_value = "1.0",
        help = "External field (h) strength"
    )]
    pub external: f64,

    #[arg(
        long,
        value_name = "TEMPERATURE",
        default_value = "0.0",
        help = "Temperature lower bound"
    )]
    pub temp_low: f64,


    #[arg(
        long,
        value_name = "TEMPERATURE",
        default_value = "10.0",
        help = "Temperature upper bound"
    )]
    pub temp_high: f64,

    #[arg(
        long,
        value_name = "STEPS",
        default_value = "1000",
        help = "Temperature points"
    )]
    pub temp_steps: usize,

    #[arg(
        long,
        value_enum,
        value_name = "DIRECTION",
        default_value = "asc",
        help = "Temperature sweep direction"
    )]
    pub temp_dir: TemperatureDirection,

    #[arg(
        long,
        value_name = "STEPS",
        default_value = "100",
        help = "Thermalization steps"
    )]
    pub therm_steps: usize
}
