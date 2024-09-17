use anyhow::Result;
use fs_err::File;
use geo::Coord;
use std::collections::HashMap;


pub fn read_walk_nodes() -> Result<HashMap<usize, Coord>> {
    let file = File::open("../data/walk_nodes.json")?;
    let reader = std::io::BufReader::new(file);
    let walk_nodes: HashMap<usize, Coord> = serde_json::from_reader(reader)?;
    Ok(walk_nodes)
}