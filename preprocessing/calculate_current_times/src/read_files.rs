use anyhow::Result;
use fs_err::File;
use std::collections::HashSet;
use std::io::BufReader;

use common::common::{NodeRoute, NodeWalk};

pub fn read_graph_routes() -> Result<Vec<NodeRoute>> {
    let file = File::open("../data/pt_graph_routes.json")?;
    let reader = BufReader::new(file);
    let graph_routes: Vec<NodeRoute> = serde_json::from_reader(reader)?;
    Ok(graph_routes)
}

pub fn read_graph_walk() -> Result<Vec<NodeWalk>> {
    let file = File::open("../data/pt_graph_walk.json")?;
    let reader = BufReader::new(file);
    let graph_walk: Vec<NodeWalk> = serde_json::from_reader(reader)?;
    Ok(graph_walk)
}

pub fn read_values() -> Result<Vec<bool>> {
    let file = File::open("../data/doctors_bool_values.json")?;
    let reader = BufReader::new(file);
    let values: Vec<bool> = serde_json::from_reader(reader)?;
    Ok(values)
}

pub fn read_node_weightings() -> Result<Vec<u16>> {
    let file = File::open("../data/residence_node_weightings.json")?;
    let reader = BufReader::new(file);
    let ode_weightings: Vec<u16> = serde_json::from_reader(reader)?;
    Ok(ode_weightings)
}

pub fn read_start_nodes() -> Result<Vec<usize>> {
    let file = File::open("../data/start_nodes.json")?;
    let reader = BufReader::new(file);
    let start_nodes_hashset: HashSet<usize> = serde_json::from_reader(reader)?;
    // convert to a vec
    let mut start_nodes: Vec<usize> = start_nodes_hashset.into_iter().collect();
    start_nodes.sort();
    Ok(start_nodes)
}
