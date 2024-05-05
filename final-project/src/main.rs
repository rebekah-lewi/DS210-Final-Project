mod readfile;
mod analysis;
mod stats;

use crate::analysis::graph_analysis;
use crate::analysis::Graph;

fn main() -> std::io::Result<()> {
    //creating a graph from the dataset
    let path = "euroroad.csv";
    let edges = readfile::read_file(path)?;
    let graph = Graph::new(&edges);

    //iterating through each node and making it as the starting point
    for i in &graph.Nodes {
        let analysis_result = graph_analysis(&graph, *i); 
        //println!("{:?}", analysis_result); -- comment out to test out what analysis_result looks like

        //statistical analysis on the distances
        let avg = stats::average(analysis_result.clone());
        let max_dist = stats::max_distance(analysis_result.clone());
        let (less_seven, less_fifteen, greater_fifteen) = stats::find_threshold(analysis_result.clone());
        
        println!("Node: {}, Average distance: {}, Maximum distance: {}", i, avg, max_dist);
        println!("Less than 7: {}, Less than 15: {}, Greater than 15: {}, Total Connections: {}\n", less_seven, less_fifteen, greater_fifteen, less_seven + less_fifteen + greater_fifteen);

    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use readfile::read_file;
    use stats::{average, max_distance, find_threshold};
    use analysis::{Graph, graph_analysis};

    #[test]
    fn test_graph_analysis_workflow() {
        // creating a small dataset to mimic the euroroad.csv format
        let test_data = vec![
            (0, 1),
            (1, 2),
            (2, 3),
            (3, 4),
            (0, 4)
        ];

        // simulating reading file, creating graph, and analyzing it
        let graph = Graph::new(&test_data);
        let start_node = 0;
        let analysis_results = graph_analysis(&graph, start_node);

        // checking if the analysis results meet expected outcomes
        assert_eq!(analysis_results.get(&0), Some(&0));
        assert_eq!(analysis_results.get(&4), Some(&1));
        
        // testing statistical functions
        let avg = average(analysis_results.clone());
        let max_dist = max_distance(analysis_results.clone());
        let (less_seven, less_fifteen, greater_fifteen) = find_threshold(analysis_results.clone());

        // checking averages and distances
        assert!(avg > 0.0);
        assert_eq!(max_dist, 2); // Max distance from node 0 to furthest node

        // checking threshold categorizations
        assert_eq!(less_seven, 5); 
        assert_eq!(less_fifteen, 0);
        assert_eq!(greater_fifteen, 0);
    }
}
