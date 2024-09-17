use anyhow::Result;
use fs_err::File;
use serde::{Deserialize, Serialize};
use std::io::{BufWriter, Write, BufReader};

#[derive(Deserialize)]
pub struct NodeWalk {
    pub has_pt: bool,
    pub edges: Vec<EdgeWalk>,
}

#[derive(Serialize, Deserialize)]
pub struct NodeWalkWeighted {
    pub has_pt: bool,
    pub edges: Vec<EdgeWalk>,
    pub has_residence: bool,
    pub weight: u16,
}

#[derive(Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Times {
    pub time: usize,
    pub node: usize,
    pub weight: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseTimes (
    pub Vec<u16>
);

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

pub fn read_pt_graph_walk() -> Result<Vec<NodeWalk>> {
    let file = File::open("../data/pt_graph_walk.json")?;
    let reader = BufReader::new(file);
    let graph_walk: Vec<NodeWalk> = serde_json::from_reader(reader)?;
    Ok(graph_walk)
}
