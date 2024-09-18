mod create_tree;
mod reverse_floodfill;

use annealing::{read_base_times, read_pt_graph_routes_reverse, read_weighted_graph_walk, Point, Bounds};
use rand::prelude::*;
use rayon::iter::IntoParallelIterator;
use rayon::prelude::*;
use std::collections::HashMap;
use std::time::Instant;

fn main() {
    let start_time = Instant::now();

    println!("Reading files");
    let pt_graph_routes_reverse = read_pt_graph_routes_reverse().unwrap();
    let pt_graph_walk_weighted = read_weighted_graph_walk().unwrap();
    let base_times = read_base_times().unwrap();
    let kdtree = create_tree::run().unwrap();
    println!("Reading files took: {:?}", start_time.elapsed());

    let (n_iterations, temp, step_size, bounds) = get_settings();

    let mut best_point = set_start_point(&bounds);
    let mut best_time_reduction = 0;
    let (mut previous_point, mut previous_time_reduction) = (best_point, best_time_reduction);
    let (mut candidate_point, mut candidate_time_reduction) = (best_point, best_time_reduction);

    let time_index_lookup = get_time_index_lookop();
    let start_times = get_start_times();

    for i in 0..n_iterations {
        // TODO add a set of tested start nodes to avoid recalculating
        let start_node = kdtree
            .nearest(&[candidate_point.y, candidate_point.x])
            .unwrap()
            .item
            .node_id;

        let time_reductions: Vec<usize> = start_times
            .clone()
            .into_par_iter()
            .map(|start_time| {
                reverse_floodfill::run(
                    start_node,
                    &start_time,
                    &pt_graph_routes_reverse,
                    &pt_graph_walk_weighted,
                    &base_times,
                    &3600,
                    &time_index_lookup,
                )
            })
            .collect();
        candidate_time_reduction = time_reductions.iter().sum();

        // decrease the temperature
        let temperature = temp - (temp / n_iterations as f64) * i as f64;

        // difference between the candidate and previous time reduction
        let delta_time_reduction: usize = candidate_time_reduction - previous_time_reduction;
        // if increase in time reduction we accept the candidate
        // or if the random number is less than the probability
        if delta_time_reduction > 0
            || rand::thread_rng().gen_range(0.0..1.0)
                < (delta_time_reduction as f64 / temperature).exp()
        {
            previous_time_reduction = candidate_time_reduction;
            previous_point = candidate_point.clone();
        }

        // get new candidate point
        candidate_point = get_new_candidate(previous_point, &step_size, &temperature, &bounds);

        // if the candidate time reduction is better than the best then update the best
        if candidate_time_reduction > best_time_reduction {
            best_time_reduction = candidate_time_reduction;
            best_point = candidate_point;
        }
    }
    println!("Best point: {:?} with time reduction: {}", best_point, best_time_reduction);
}

fn get_settings() -> (usize, f64, [f64; 2], Bounds) {
    let n_iterations = 100;
    let temp = 1.0;
    let bounds: Bounds = Bounds {
        min: Point {
            x: 53.822138,
            y: -0.45730591,
        },
        max: Point {
            x: 53.864068,
            y: -0.41181564,
        },
    };
    let step_size = [(bounds.max.x - bounds.min.x), (bounds.max.y - bounds.min.y)];
    (n_iterations, temp, step_size, bounds)
}

fn set_start_point(bounds: &Bounds) -> Point {
    let mut rng = rand::thread_rng();
    let lat = rng.gen_range(bounds.min.x..bounds.max.x);
    let lon = rng.gen_range(bounds.min.y..bounds.max.y);
    Point { x: lat, y: lon }
}

fn get_time_index_lookop() -> HashMap<usize, usize> {
    let mut time_index_lookup = HashMap::new();
    for i in 0..37 {
        // run from 7am - 10am
        let start_time = 25200 + (i * 300);
        time_index_lookup.insert(start_time, i);
    }
    time_index_lookup
}

fn get_start_times() -> Vec<usize> {
    let mut start_times = Vec::new();
    for i in 12..37 {
        // start times from 8am - 10am
        let start_time = 25200 + (i * 300);
        start_times.push(start_time);
    }
    start_times
}

fn get_new_candidate(
    current_point: Point,
    step_size: &[f64; 2],
    _temperature: &f64, // TODO possible integration
    bounds: &Bounds,
) -> Point {
    let mut new_point: Point = Point { x: 0.0, y: 0.0 };
    let points_outside_bounds = true;
    let mut rng = rand::thread_rng();
    while points_outside_bounds {
        let lat = rng.gen_range(-step_size[0]..step_size[0]);
        let lon = rng.gen_range(-step_size[1]..step_size[1]);
        let new_lat = current_point.x + lat;
        let new_lon = current_point.y + lon;

        if new_lon >= bounds.min.y
            && new_lon <= bounds.max.y
            && new_lat >= bounds.min.x
            && new_lat <= bounds.max.x
        {
            new_point = Point {
                x: new_lat,
                y: new_lon,
            };
            break;
        }
    }
    new_point
}
