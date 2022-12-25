#![feature(iter_advance_by)]
#![feature(option_get_or_insert_default)]
use std::{
    collections::{hash_map, BTreeSet, HashMap, HashSet},
    hash::Hash,
    io,
};
use utils::*;

const MAX_TIME: u32 = 30;

/// Keeps track of highest scores from this location for a given set of items on
struct Path {
    turned_on: HashSet<String>,
    score: u32,
}

#[derive(Debug)]
struct Valve {
    r: u32,
    conn: Vec<String>,
}

#[derive(Debug)]
struct Valve2 {
    r: u32,
    conn: HashMap<String, u32>,
}
// O(K * V * V *  V * T)
struct Iteration {
    location: String,
    turned_on: BTreeSet<String>,
    score: u32,
    visited: BTreeSet<String>,
}

fn main() -> io::Result<()> {
    let mut valves = HashMap::new();
    for line in io::stdin().lines() {
        let line = line.unwrap();
        let mut words = line.split(" ");
        words.advance_by(1).unwrap();
        let name = words.next().unwrap().to_owned();
        words.advance_by(2).unwrap();
        let r = words.next().unwrap();
        let r = r[5..(r.len() - 1)].parse().unwrap();
        words.advance_by(4).unwrap();
        let mut conn = Vec::new();
        for w in words {
            let w = w.split(",").next().unwrap();
            conn.push(w.to_owned());
        }

        let valve = Valve { r, conn };
        dbg!(&name, &valve);
        valves.insert(name, valve);
    }

    let mut node_scores = HashMap::new();
    let mut start = HashMap::new();
    start.insert(BTreeSet::new(), 0);
    node_scores.insert("AA".to_owned(), start);
    // let mut iterations = Vec::new();
    // let mut lastscore = 0;
    for i in 0..MAX_TIME {
        node_scores = solve(&valves, &node_scores);

        // let ns = node_scores.clone();
        // let mut max_score = 0;

        // for (k, v) in ns {
        //     for (k, v) in v {
        //         max_score = max_score.max(v);
        //     }
        // }
        // println!("{} - {}", max_score, max_score - lastscore);
        // lastscore = max_score;
    }
    // dbg!(&node_scores.get(&"CC".to_owned()).unwrap());

    // dbg!(&node_scores.get(&"CC".to_owned()).unwrap().iter().filter(
    //     |(k, v)| {
    //         k.contains(&"BB".to_owned())
    //             && k.contains(&"DD".to_owned())
    //             && k.contains(&"EE".to_owned())
    //             && k.contains(&"HH".to_owned())
    //             && k.contains(&"JJ".to_owned())
    //             && k.contains(&"CC".to_owned())
    //     }
    // ));

    // bdehjc

    let mut max_score = 0;
    for (k, v) in node_scores {
        for (k, v) in v {
            max_score = max_score.max(v);
        }
    }
    println!("{}", max_score);
    Ok(())
}
/// I think the idea is a backtracking alorithm that keeps track of an
/// additional arrayed state of having not visited any particular node.
fn solve(
    valves: &HashMap<String, Valve>,
    // location: &String,
    node_scores: &HashMap<String, HashMap<BTreeSet<String>, u32>>,
    // next_iterations: &Vec<Iteration>,
) -> HashMap<String, HashMap<BTreeSet<String>, u32>> {
    let mut next_node_scores = HashMap::new();

    for location in valves.keys() {
        let connecting = connecting_locations(valves, location);

        let active_node_scores = None;
        let active_node_scores =
            if let Some(node_scores_for_moving_to_location) =
                merge_connection_node_scores(&connecting, node_scores)
            {
                let node_scores_for_moving_to_location =
                    advance_scores(&node_scores_for_moving_to_location, valves);

                if let Some(active_node_scores) = active_node_scores {
                    Some(merge_scores(
                        &active_node_scores,
                        &node_scores_for_moving_to_location,
                    ))
                } else {
                    Some(node_scores_for_moving_to_location)
                }
            } else {
                None
            };

        // Started on location
        let active_node_scores = if let Some(started_on_location) =
            node_scores.get(location)
        {
            let started_on_location =
                advance_scores(&started_on_location, valves);

            // turn on for all started on location
            let started_on_location = started_on_location.iter().fold(
                HashMap::new(),
                |mut acc, (k, v)| {
                    let mut new_key = k.clone();
                    new_key.insert(location.clone());
                    let new_val = if let Some(prev_val) = acc.get(&new_key) {
                        *v.max(prev_val)
                    } else {
                        *v
                    };
                    acc.insert(new_key, new_val);
                    acc
                },
            );

            if let Some(active_node_scores) = active_node_scores {
                Some(merge_scores(&active_node_scores, &started_on_location))
            } else {
                Some(started_on_location)
            }
        } else {
            active_node_scores
        };

        if let Some(active_node_scores) = active_node_scores {
            let cleaned_scores = cleanup_scores(&active_node_scores);
            next_node_scores.insert(location.clone(), cleaned_scores);
        }
    }

    next_node_scores
}

fn rate_per_time(
    valves: &HashMap<String, Valve>,
    on_nodes: &BTreeSet<String>,
) -> u32 {
    on_nodes
        .iter()
        .map(|k| valves.get(k).unwrap().r)
        .fold(0, |acc, r| acc + r)
}

fn connecting_locations(
    valves: &HashMap<String, Valve>,
    location: &String,
) -> Vec<String> {
    valves.get(location).unwrap().conn.clone()
}
fn cleanup_scores(
    scores: &HashMap<BTreeSet<String>, u32>,
) -> HashMap<BTreeSet<String>, u32> {
    let mut cleaned = scores.clone();
    for (k, v) in scores {
        for (k2, v2) in scores {
            if k.is_superset(&k2) && v > v2 {
                cleaned.remove(k2);
            }
        }
    }
    cleaned
}
fn merge_scores(
    scores1: &HashMap<BTreeSet<String>, u32>,
    scores2: &HashMap<BTreeSet<String>, u32>,
) -> HashMap<BTreeSet<String>, u32> {
    scores2.iter().fold(scores1.clone(), |mut acc, (k, v)| {
        let val = acc.get(k).cloned().unwrap_or_default();
        acc.insert(k.clone(), val.max(*v));
        acc
    })
}

fn merge_connection_node_scores(
    connections: &Vec<String>,
    node_scores: &HashMap<String, HashMap<BTreeSet<String>, u32>>,
) -> Option<HashMap<BTreeSet<String>, u32>> {
    let filtered_for_connections = node_scores
        .iter()
        .filter_map(|(k, v)| {
            if connections.contains(k) {
                Some(v)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    if filtered_for_connections.len() > 0 {
        Some(filtered_for_connections.iter().fold(
            HashMap::new(),
            |mut acc, x| {
                x.iter().for_each(|(k, v)| {
                    let val = acc.get(k).cloned().unwrap_or_default();
                    acc.insert(k.clone(), val.max(*v));
                });
                acc
            },
        ))
    } else {
        None
    }
}

fn advance_scores(
    scores: &HashMap<BTreeSet<String>, u32>,
    valves: &HashMap<String, Valve>,
) -> HashMap<BTreeSet<String>, u32> {
    let mut new_scores = HashMap::new();
    for (k, score) in scores.iter() {
        let next_score = score + rate_per_time(valves, k);
        new_scores.insert(k.clone(), next_score);
    }
    new_scores
}
