use annealing::{Item, Point};
use anyhow::Result;
use fs_err::File;
use kd_tree::KdTree;
use std::collections::HashMap;
use std::time::Instant;

pub fn run() -> Result<KdTree<Item>> {
    let walk_nodes = read_walk_nodes().unwrap();
    let coords: Vec<Item> = walk_nodes
        .iter()
        .map(|(node_id, coord)| Item {
            point: [coord.x, coord.y],
            node_id: *node_id,
        })
        .collect();

    let start_time = Instant::now();
    let kdtree: KdTree<Item> = KdTree::par_build_by_ordered_float(coords);
    println!("Creating tree took: {:?}", start_time.elapsed());
    Ok(kdtree)
}

fn read_walk_nodes() -> Result<HashMap<usize, Point>> {
    let file = File::open("../data/walk_nodes.json")?;
    let reader = std::io::BufReader::new(file);
    let walk_nodes: HashMap<usize, Point> = serde_json::from_reader(reader)?;
    Ok(walk_nodes)
}
