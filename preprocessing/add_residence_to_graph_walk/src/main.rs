use common::common::{NodeWalk, NodeWalkWeighted, write_json_file};
use anyhow::Result;
use fs_err::File;
use std::io::BufReader;

fn main() {
    let pt_graph_walk = read_pt_graph_walk().unwrap();
    let residence_node_weightings = read_residence_node_weightings().unwrap();

    let mut pt_graph_walk_weighted: Vec<NodeWalkWeighted> = Vec::new();
    for (i, node) in pt_graph_walk.iter().enumerate() {
        let weight = residence_node_weightings[i];
        let has_residence = weight != 0;
        let node_walk_weighted = NodeWalkWeighted {
            has_pt: node.has_pt,
            edges: node.edges.clone(),
            has_residence,
            weight,
        };
        pt_graph_walk_weighted.push(node_walk_weighted);
    }
    write_json_file(format!("pt_graph_walk_weighted"), "../data", pt_graph_walk_weighted).unwrap();
}

fn read_pt_graph_walk() -> Result<Vec<NodeWalk>> {
    let file = File::open("../data/pt_graph_walk.json")?;
    let reader = BufReader::new(file);
    let graph_walk: Vec<NodeWalk> = serde_json::from_reader(reader)?;
    Ok(graph_walk)
}

fn read_residence_node_weightings() -> Result<Vec<u16>> {
    let file = File::open("../data/residence_node_weightings.json")?;
    let reader = BufReader::new(file);
    let residence_node_weightings: Vec<u16> = serde_json::from_reader(reader)?;
    Ok(residence_node_weightings)
}