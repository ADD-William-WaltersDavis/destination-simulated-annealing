use anyhow::Result;
use fs_err::File;
use kd_tree::KdPoint;
use serde::{Deserialize, Serialize};
use std::io::{BufReader, BufWriter, Write};
use typenum;

#[derive(Serialize, Deserialize)]
pub struct NodeWalkWeighted {
    pub has_pt: bool,
    pub edges: Vec<EdgeWalk>,
    pub has_residence: bool,
    pub weight: usize,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct EdgeWalk {
    pub cost: usize,
    pub to: usize,
}

#[derive(Deserialize, Serialize)]
pub struct ReverseNodeRoute {
    pub prev_stop_node: usize,
    pub timetable: Vec<ReverseEdgeRoute>,
}

#[derive(Deserialize, Serialize)]
pub struct ReverseEdgeRoute {
    pub leave_time_previous_stop: usize,
    pub cost: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseTimes(pub Vec<usize>);

#[derive(Debug)]
pub struct Item {
    pub point: [f64; 2],
    pub node_id: usize,
}

impl KdPoint for Item {
    type Scalar = f64;
    type Dim = typenum::U2;
    fn at(&self, k: usize) -> f64 {
        self.point[k]
    }
}

pub fn read_weighted_graph_walk() -> Result<Vec<NodeWalkWeighted>> {
    let file = File::open("../data/pt_graph_walk_weighted.json")?;
    let reader = BufReader::new(file);
    let pt_graph_walk_weighted: Vec<NodeWalkWeighted> = serde_json::from_reader(reader)?;
    Ok(pt_graph_walk_weighted)
}

pub fn read_pt_graph_routes_reverse() -> Result<Vec<ReverseNodeRoute>> {
    let file = File::open("../data/pt_graph_routes_reverse.json")?;
    let reader = BufReader::new(file);
    let pt_graph_routes_reverse: Vec<ReverseNodeRoute> = serde_json::from_reader(reader)?;
    Ok(pt_graph_routes_reverse)
}

pub fn read_base_times() -> Result<Vec<BaseTimes>> {
    let file = File::open("../data/base_times.json")?;
    let reader = BufReader::new(file);
    let base_times: Vec<BaseTimes> = serde_json::from_reader(reader)?;
    Ok(base_times)
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
