use std::io::{self, Write};

use clap::Parser;

use ::ising::*;

fn main() -> Result<(), io::Error> {
    let args = cli::Args::parse();

    let graph = graph::Graph::new(args.size, args.topology);

    let fields = ising::Fields(args.internal, args.external);

    let mut model = ising::Ising::new(graph, args.spins, fields);

    print!("Thermalizing... ");
    io::stdout().flush()?;

    for _ in 0..args.therm_steps {
        model.step(args.temp_low);
    }

    print!("done.\n");
    io::stdout().flush()?;

    let temperatures = match args.temp_dir {
        cli::TemperatureDirection::Asc => {
            linspace(
                args.temp_low,
                args.temp_high,
                args.temp_steps
            )
        },
        cli::TemperatureDirection::Desc => {
            linspace(
                args.temp_high,
                args.temp_low,
                args.temp_steps
            )
        },
    };

    println!("Begin simulation");
    for temp in temperatures {
        println!("Temperature: {temp}");

        for step in 0..args.steps {
            model.step(temp);

            if step % args.probe_step == 0 {
                println!(
                    "Energy: {:.2}, magnetization: {:.2}",
                    model.energy(),
                    model.magnetization()
                );
            }
        }
    }

    Ok(())
}
