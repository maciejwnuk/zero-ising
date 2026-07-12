# Zero-Ising

Ising model simulation with an optional zero spin state, supporting both periodic chain and Barabási–Albert graph topologies.

Project done as engineering thesis at Warsaw University of Technology (WUT), Poland.

## Description

The simulator implements a modified Glauber dynamics algorithm for the Ising model Hamiltonian:

$$H = -J \sum_{\langle i,j \rangle} s_i s_j - h \sum_i s_i$$

where $J$ is the internal (exchange) field strength and $h$ is the external field strength.

### Features

- **Spin states**: standard $\{-1, +1\}$ or extended $\{-1, 0, +1\}$ (zero state)
- **Topologies**:
  - **Chain** — periodic 1D lattice (ring)
  - **Graph** — Barabási–Albert scale-free network ($m = 3$)
- **Temperature sweep** — ascending or descending through a configurable range
- **Thermalization** — configurable equilibration steps before data collection
- **CSV output** — energy and magnetization measurements at probe intervals

## Building

Requires Rust 2024 edition.

```sh
cargo build --release
```

## Usage

```
ising <SPINS> <TOPOLOGY> [FILE] [OPTIONS]
```

### Positional arguments

| Argument | Values | Description |
|---|---|---|
| `SPINS` | `normal`, `extended` | Spin states of the model |
| `TOPOLOGY` | `chain`, `graph` | Topology of the underlying graph |
| `FILE` | path (default: `output.csv`) | Output data filename |

### Options

| Flag | Default | Description |
|---|---|---|
| `-s, --steps` | `100` | Simulation steps at each temperature |
| `--probe-step` | `10` | Modulus-step for data acquisition |
| `-S, --size` | `1000` | Number of nodes in the chain/graph |
| `-i, --internal` | `1.0` | Internal field ($J$) strength |
| `-e, --external` | `1.0` | External field ($h$) strength |
| `--temp-low` | `0.0` | Temperature lower bound |
| `--temp-high` | `10.0` | Temperature upper bound |
| `--temp-steps` | `1000` | Number of temperature points |
| `--temp-dir` | `asc` | Temperature sweep direction (`asc` / `desc`) |
| `--therm-steps` | `100` | Thermalization steps before collecting data |

### Examples

Standard Ising model on a periodic chain with 500 nodes:

```sh
zero-ising normal chain -S 500 --steps 200
```

Extended model (with zero state) on a Barabási–Albert graph, cooling sweep:

```sh
zero-ising extended graph results.csv -S 2000 --temp-dir desc --temp-steps 500
```

Weak coupling with strong external field:

```sh
zero-ising normal chain -i 0.5 -e 2.0 --temp-low 0.1 --temp-high 5.0
```

## Output

The program writes a CSV file with the following columns:

| Column | Description |
|---|---|
| `n` | System size |
| `T` | Temperature |
| `E` | Total energy |
| `s` | Total magnetization |

## License

[MIT](LICENSE.md) — Copyright (c) 2026 Maciej Wnuk
