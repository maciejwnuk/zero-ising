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
                    adj[i][(i + size - 1) % size] = 1;
                    adj[i][(i + 1) % size] = 1;
                }
            },
            Topology::Graph => {
                assert!(size >= 3, "Size must be bigger than m = 3 for Barabasi-Albert");

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chain_topology_size() {
        let graph = Graph::new(10, Topology::Chain);

        assert_eq!(graph.size(), 10);
    }

    #[test]
    fn test_chain_topology_symmetric() {
        let size = 10;

        let graph = Graph::new(size, Topology::Chain);
        let adj = graph.adj();

        for i in 0..size {
            for j in 0..size {
                assert_eq!(adj[i][j], adj[j][i]);
            }
        }
    }

    #[test]
    fn test_chain_topology_neighbors() {
        let size = 10;

        let graph = Graph::new(size, Topology::Chain);
        let adj = graph.adj();

        for i in 0..size {
            let sum: usize = adj[i].iter().sum();
            assert_eq!(sum, 2);
            assert_eq!(adj[i][(i + size - 1) % size], 1);
            assert_eq!(adj[i][(i + 1) % size], 1);
            assert_eq!(adj[i][i], 0);
        }
    }

    #[test]
    fn test_chain_topology_size_two() {
        let graph = Graph::new(2, Topology::Chain);
        let adj = graph.adj();

        assert_eq!(adj[0][1], 1);
        assert_eq!(adj[1][0], 1);
    }

    #[test]
    fn test_graph_topology_size() {
        let graph = Graph::new(10, Topology::Graph);

        assert_eq!(graph.size(), 10);
    }

    #[test]
    fn test_graph_topology_symmetric_and_no_self_loops() {
        let size = 10;

        let graph = Graph::new(size, Topology::Graph);
        let adj = graph.adj();

        for i in 0..size {
            assert_eq!(adj[i][i], 0);
            for j in 0..size {
                assert_eq!(adj[i][j], adj[j][i]);
            }
        }
    }

    #[test]
    fn test_graph_topology_connected() {
        let size = 10;

        let graph = Graph::new(size, Topology::Graph);
        let adj = graph.adj();

        let mut total_edges = 0;

        for i in 0..size {
            let sum: usize = adj[i].iter().sum();
            assert!(sum >= 1);
            total_edges += sum;
        }

        assert!(total_edges / 2 >= size);
    }
}
