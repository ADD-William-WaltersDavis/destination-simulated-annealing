use common::common::{BaseTimes, Times, write_json_file, read_pt_graph_walk};
use anyhow::Result;
use fs_err::File;
use std::io::BufReader;

fn main() {
    let graph_length = get_graph_length();
    let mut base_times: Vec<BaseTimes> = Vec::new();
    for _ in 0..graph_length {
        base_times.push(BaseTimes(Vec::new()));
    }

    // run from 7am - 10am
    for i in 0..37 {
        let start_time = 25200 + (i * 300);
        println!("Adding base times from {}", start_time);

        let calculated_times = read_calculated_times(&start_time.to_string()).unwrap();
        for calculated_time in calculated_times {
            let time: u16 = calculated_time.time as u16;
            base_times[calculated_time.node].0.push(time);
        }
    }
    // check that the base times are either empty or have 37 elements
    for base_time in base_times.iter() {
        assert!(base_time.0.len() == 37 || base_time.0.len() == 0);
    }
    write_json_file(format!("base_times"), "../data", &base_times).unwrap();
}

fn read_calculated_times(time: &str) -> Result<Vec<Times>> {
    let file = File::open(format!("output/{}_results.json", time)).unwrap();
    let reader = BufReader::new(file);
    let calculated_times: Vec<Times> = serde_json::from_reader(reader).unwrap();
    Ok(calculated_times)
}
fn get_graph_length() -> usize {
    let pt_graph_walk = read_pt_graph_walk().unwrap();
    pt_graph_walk.len()
}