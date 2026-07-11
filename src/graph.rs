use clap::ValueEnum;
use rand::seq::index::sample_weighted;

#[derive(Debug, Clone, ValueEnum)]
pub enum Topology {
    Chain,
    Graph   // m = 3
}

#[derive(Debug)]
pub struct Graph {
    adj: Vec<Vec<usize>>,
}

impl Graph {
    pub fn new(size: usize, topology: Topology) -> Self {
        let mut adj = vec![vec![0; size]; size];

        match topology {
            Topology::Chain => {
                for i in 0..size {
                    adj[i][i - 1] = 1;
                    adj[i][i + 1] = 1;
                }
            },
            Topology::Graph => {
                assert!(size >= 3, "Size must be >= m for Barabasi-Albert");

                for i in 0..3 {
                    for j in (i + 1)..3 {
                        adj[i][j] = 1;
                        adj[j][i] = 1;
                    }
                }

                let mut rng = rand::rng();

                for i in 3..size {
                    let degrees: Vec<f32> = (0..i)
                        .map(|n|
                            adj[n][..i]
                                .iter()
                                .map(|&x| x as f32)
                                .sum::<f32>()
                        )
                        .collect();

                    let targets = sample_weighted(&mut rng, i, |k| degrees[k], 3)
                        .expect("Weighted sampling failed");

                    for j in targets {
                        adj[i][j] = 1;
                        adj[j][i] = 1;
                    }
                }
            },
        };

        Graph { adj }
    }

    pub fn size(&self) -> usize {
        self.adj.len()
    }

    pub fn adj(&self) -> &[Vec<usize>] {
        &self.adj
    }
}
