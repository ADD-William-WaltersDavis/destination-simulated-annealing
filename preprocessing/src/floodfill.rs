use preproc::{NodeRoute, NodeWalk, Times};
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(PartialEq, Eq, Clone)]
pub struct PriorityQueueItem<C, N> {
    pub cost: C,
    pub node: N,
}

impl<C: Ord, N: Ord> PartialOrd for PriorityQueueItem<C, N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Telling rust to order the heap by cost
impl<C: Ord, N: Ord> Ord for PriorityQueueItem<C, N> {
    fn cmp(&self, other: &Self) -> Ordering {
        let ord = other.cost.cmp(&self.cost);
        if ord != Ordering::Equal {
            return ord;
        }
        // The tie-breaker is arbitrary, based on the node
        self.node.cmp(&other.node)
    }
}

pub fn run(
    graph_walk: &Vec<NodeWalk>,
    graph_routes: &Vec<NodeRoute>,
    node_weightings: &Vec<u16>,
    values: &Vec<bool>,
    start_node: usize,
    time_limit: &usize,
    start_time: &usize,
) -> Times {
    let mut queue: BinaryHeap<PriorityQueueItem<usize, usize>> = BinaryHeap::new();
    queue.push(PriorityQueueItem {
        cost: *start_time,
        node: start_node,
    });

    let mut time_taken = time_limit.clone();
    let end_time = start_time + time_limit;
    let mut nodes_visted: Vec<bool> = vec![false; graph_walk.len()].into();

    while let Some(current) = queue.pop() {
        // break if we find a doctors/clinic
        if values[current.node] {
            time_taken = current.cost - start_time;
            break;
        }
        // skip if visited before
        if nodes_visted[current.node] {
            continue;
        }
        // mark node as visited
        nodes_visted[current.node] = true;

        for edge in &graph_walk[current.node].edges {
            let new_cost = current.cost + edge.cost;
            if new_cost <= end_time {
                queue.push(PriorityQueueItem {
                    cost: new_cost,
                    node: edge.to,
                });
            }
        }
        if graph_walk[current.node].has_pt {
            take_next_pt_route(
                graph_routes,
                current.cost,
                current.node,
                &mut queue,
                &end_time,
            );
        }
    }

    Times {
        time: time_taken,
        node: start_node,
        weight: node_weightings[start_node],
    }
}

fn take_next_pt_route(
    graph_routes: &Vec<NodeRoute>,
    time_so_far: usize,
    current_node: usize,
    queue: &mut BinaryHeap<PriorityQueueItem<usize, usize>>,
    end_time: &usize,
) {
    let mut found_next_service = false;
    let mut journey_time_to_next_node = 0;
    let mut next_leaving_time = 0;

    for edge in &graph_routes[current_node].timetable {
        if time_so_far <= edge.leavetime {
            found_next_service = true;
            journey_time_to_next_node = edge.cost;
            next_leaving_time = edge.leavetime;
            break;
        }
    }
    if found_next_service {
        let arrival_time = next_leaving_time + journey_time_to_next_node;
        if arrival_time <= *end_time {
            queue.push(PriorityQueueItem {
                cost: arrival_time,
                node: graph_routes[current_node].next_stop_node,
            });
        }
    }
}
