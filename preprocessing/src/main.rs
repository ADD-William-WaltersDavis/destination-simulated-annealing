mod floodfill;

use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use preproc::{
    read_graph_routes, read_graph_walk, read_node_weightings, read_start_nodes, read_values,
    write_json_file, Times,
};
use rayon::prelude::*;

fn main() {
    let graph_routes = read_graph_routes().unwrap();
    let graph_walk = read_graph_walk().unwrap();
    let values = read_values().unwrap();
    let node_weightings = read_node_weightings().unwrap();
    let start_nodes = read_start_nodes().unwrap();
    let time_limit = 3600;
    let start_time = 28800;
    println!("start node length {:?}", start_nodes);

    assert_eq!(graph_routes.len(), graph_walk.len());
    assert_eq!(graph_routes.len(), values.len());

    let progress = ProgressBar::new(start_nodes.len() as u64).with_style(ProgressStyle::with_template(
        "[{elapsed_precise}] [{wide_bar:.cyan/blue}] {human_pos}/{human_len} ({per_sec}, {eta})").unwrap());

    let results: Vec<Times> = start_nodes
        .into_par_iter()
        .progress_with(progress)
        .map(|start_node| {
            floodfill::run(
                &graph_walk,
                &graph_routes,
                &node_weightings,
                &values,
                start_node,
                &time_limit,
                &start_time,
            )
        })
        .collect();
    write_json_file(format!("results.json"), "output", &results).unwrap();
}
