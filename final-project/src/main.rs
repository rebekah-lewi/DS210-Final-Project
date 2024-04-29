mod readfile;
mod analysis;
mod stats;

use crate::analysis::graph_analysis;
use crate::analysis::Graph;

use std::env;

fn main() -> std::io::Result<()> {
    let path = "euroroad.csv";
    let edges = readfile::read_file(path)?;

    let graph = Graph::new(&edges);
    for node in 
        for ...
            let analysis_result = graph_analysis(&graph, 2); 
            // Assuming you want to start analysis from node 1
        println!("{:?}", analysis_result);
        for data in datasets.iter() {
            let avg = stats::average();
            let max_dist = stats::max_distance();
            let threshold_count = stats::find_threshold();
    for (node, distance) in analysis_result {
        println!("Node {}: Distance from start = {}", node, distance);
    }

    Ok(())
}