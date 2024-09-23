mod reverse_floodfill_route_taken;

use annealing::{read_pt_graph_routes_reverse, read_weighted_graph_walk, write_json_file};
fn main() {
    println!("Reading files");
    let pt_graph_routes_reverse = read_pt_graph_routes_reverse().unwrap();
    let pt_graph_walk_weighted = read_weighted_graph_walk().unwrap();

    let nodes_route_taken = reverse_floodfill_route_taken::run(
        0,
        &21600,
        &pt_graph_routes_reverse,
        &pt_graph_walk_weighted,
        &3600,
    );
    write_json_file(format!("nodes_route_taken"), "data", &nodes_route_taken).unwrap();
}
