use annealing::{BaseTimes, NodeWalkWeighted, ReverseNodeRoute};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

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

// Telling rust to order the heap by cost heighest first
impl<C: Ord, N: Ord> Ord for PriorityQueueItem<C, N> {
    fn cmp(&self, other: &Self) -> Ordering {
        let ord = self.cost.cmp(&other.cost);
        if ord != Ordering::Equal {
            return ord;
        }
        // The tie-breaker is arbitrary, based on the node
        self.node.cmp(&other.node)
    }
}

pub fn run(
    start_node: usize,
    start_time: &usize,
    pt_graph_routes_reverse: &Vec<ReverseNodeRoute>,
    pt_graph_walk_weighted: &Vec<NodeWalkWeighted>,
    time_limit: &usize,
) -> Vec<usize> {
    let mut queue: BinaryHeap<PriorityQueueItem<usize, usize>> = BinaryHeap::new();
    queue.push(PriorityQueueItem {
        cost: *start_time,
        node: start_node,
    });
    let end_time = start_time - time_limit;
    let mut nodes_visted: Vec<bool> = vec![false; pt_graph_walk_weighted.len()].into();
    let mut nodes_route_taken: Vec<usize> = Vec::new();

    while let Some(current) = queue.pop() {
        if nodes_visted[current.node] {
            continue;
        }
        nodes_visted[current.node] = true;

        for edge in &pt_graph_walk_weighted[current.node].edges {
            let new_cost = current.cost - edge.cost;
            if new_cost >= end_time {
                queue.push(PriorityQueueItem {
                    cost: new_cost,
                    node: edge.to,
                });
            }
        }

        if pt_graph_walk_weighted[current.node].has_pt {
            take_next_pt_route_reverse(
                pt_graph_routes_reverse,
                current.cost,
                current.node,
                &mut queue,
                &end_time,
            );
        }
        nodes_route_taken.push(current.node);
    }
    nodes_route_taken
}

fn take_next_pt_route_reverse(
    graph_routes: &Vec<ReverseNodeRoute>,
    time_so_far: usize,
    current_node: usize,
    queue: &mut BinaryHeap<PriorityQueueItem<usize, usize>>,
    end_time: &usize,
) {
    let mut found_next_service = false;
    let mut arrival_time_previous_node = 0;

    for edge in &graph_routes[current_node].timetable {
        if time_so_far >= edge.leave_time_previous_stop + edge.cost {
            found_next_service = true;
            arrival_time_previous_node = edge.leave_time_previous_stop;
            break;
        }
    }
    if found_next_service {
        if arrival_time_previous_node >= *end_time {
            queue.push(PriorityQueueItem {
                cost: arrival_time_previous_node,
                node: graph_routes[current_node].prev_stop_node,
            });
        }
    }
}
