use ::ising::linspace;
use ::ising::graph::{Graph, Topology};
use ::ising::ising::{Ising, States, Fields};

#[test]
fn simulation_chain_normal() {
    let graph = Graph::new(50, Topology::Chain);
    let mut ising = Ising::new(graph, States::Normal, Fields(1.0, 0.0));

    for _ in 0..100 {
        ising.step(1.0);
    }

    let energy = ising.energy();
    assert!(energy.is_finite());

    let mag = ising.magnetization();
    assert!(mag >= -50 && mag <= 50);

    let state = ising.state();
    assert_eq!(state.len(), 50);

    for &spin in state {
        assert!(spin == -1 || spin == 1);
    }
}

#[test]
fn simulation_chain_extended() {
    let graph = Graph::new(50, Topology::Chain);
    let mut ising = Ising::new(graph, States::Extended, Fields(1.0, 0.0));

    for _ in 0..100 {
        ising.step(1.0);
    }

    let energy = ising.energy();
    assert!(energy.is_finite());

    let mag = ising.magnetization();
    assert!(mag >= -50 && mag <= 50);

    let state = ising.state();
    assert_eq!(state.len(), 50);

    for &spin in state {
        assert!(spin == -1 || spin == 0 || spin == 1);
    }
}

#[test]
fn simulation_graph_normal() {
    let graph = Graph::new(50, Topology::Graph);
    let mut ising = Ising::new(graph, States::Normal, Fields(1.0, 0.0));

    for _ in 0..100 {
        ising.step(1.0);
    }

    let energy = ising.energy();
    assert!(energy.is_finite());

    let mag = ising.magnetization();
    assert!(mag >= -50 && mag <= 50);

    let state = ising.state();
    assert_eq!(state.len(), 50);

    for &spin in state {
        assert!(spin == -1 || spin == 1);
    }
}

#[test]
fn simulation_graph_extended() {
    let graph = Graph::new(50, Topology::Graph);
    let mut ising = Ising::new(graph, States::Extended, Fields(1.0, 0.0));

    for _ in 0..100 {
        ising.step(1.0);
    }

    let energy = ising.energy();
    assert!(energy.is_finite());

    let mag = ising.magnetization();
    assert!(mag >= -50 && mag <= 50);

    let state = ising.state();
    assert_eq!(state.len(), 50);

    for &spin in state {
        assert!(spin == -1 || spin == 0 || spin == 1);
    }
}

#[test]
fn low_temperature_ordering() {
    let mut count = 0;

    for _ in 0..10 {
        let graph = Graph::new(10, Topology::Chain);
        let mut ising = Ising::new(graph, States::Normal, Fields(1.0, 0.0));

        for _ in 0..100 {
            ising.step(0.01);
        }

        if ising.magnetization().abs() == 10 { count += 1; }
    }

    assert!(count > 8);
}

#[test]
fn thermalization_then_measurement() {
    let graph = Graph::new(40, Topology::Chain);
    let mut ising = Ising::new(graph, States::Normal, Fields(1.0, 0.0));

    for _ in 0..200 {
        ising.step(1.0);
    }

    let mut measurements = Vec::new();

    for _ in 0..10 {
        for _ in 0..10 {
            ising.step(1.0);
        }

        let e = ising.energy();
        assert!(e.is_finite());

        measurements.push(e);
    }

    assert_eq!(measurements.len(), 10);
}

#[test]
fn ascending_temperature_sweep() {
    let temps = linspace(0.1, 3.0, 5);

    for i in 0..temps.len() - 1 {
        assert!(temps[i] < temps[i + 1]);
    }

    let runs = 50;
    let mut avg_energies = Vec::new();

    for &t in &temps {
        let mut energy_sum = 0.0;

        for _ in 0..runs {
            let graph = Graph::new(20, Topology::Chain);
            let mut ising = Ising::new(graph, States::Normal, Fields(1.0, 0.0));

            for _ in 0..200 {
                ising.step(t);
            }

            energy_sum += ising.energy();
        }

        avg_energies.push(energy_sum / runs as f64);
    }

    for i in 0..avg_energies.len() - 1 {
        assert!(avg_energies[i] <= avg_energies[i + 1]);
    }
}

#[test]
fn descending_temperature_sweep() {
    let temps = linspace(3.0, 0.1, 5);

    for i in 0..temps.len() - 1 {
        assert!(temps[i] > temps[i+1]);
    }

    let runs = 50;
    let mut avg_energies = Vec::new();

    for &t in &temps {
        let mut energy_sum = 0.0;

        for _ in 0..runs {
            let graph = Graph::new(20, Topology::Chain);
            let mut ising = Ising::new(graph, States::Normal, Fields(1.0, 0.0));

            for _ in 0..200 {
                ising.step(t);
            }

            energy_sum += ising.energy();
        }

        avg_energies.push(energy_sum / runs as f64);
    }

    for i in 0..avg_energies.len() - 1 {
        assert!(avg_energies[i] >= avg_energies[i + 1]);
    }
}
