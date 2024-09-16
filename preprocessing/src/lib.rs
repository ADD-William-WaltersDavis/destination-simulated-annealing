use anyhow::Result;
use fs_err::File;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::io::{BufReader, BufWriter, Write};

#[derive(Deserialize)]
pub struct NodeWalk {
    pub has_pt: bool,
    pub edges: Vec<EdgeWalk>,
}

#[derive(Deserialize)]
pub struct EdgeWalk {
    pub cost: usize,
    pub to: usize,
}

#[derive(Deserialize)]
pub struct NodeRoute {
    pub next_stop_node: usize,
    pub timetable: Vec<EdgeRoute>,
}

#[derive(Deserialize)]
pub struct EdgeRoute {
    pub leavetime: usize,
    pub cost: usize,
}

#[derive(Debug, Serialize)]
pub struct Times {
    pub time: usize,
    pub node: usize,
    pub weight: u16,
}

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

pub fn write_json_file<T: Serialize>(
    file_name: String,
    output_directory: &str,
    data: T,
) -> Result<()> {
    let path = format!("{output_directory}/{file_name}.json");
    println!("Writing to {path}");
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, &data)?;
    writer.flush()?;
    Ok(())
}
