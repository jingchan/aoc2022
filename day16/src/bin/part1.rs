#![feature(iter_advance_by)]
#![feature(option_get_or_insert_default)]
use std::{
    collections::{BTreeSet, HashMap, HashSet},
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
        valves.insert(name, valve);
    }

    // let mut valves2 = HashMap::new();
    // for (name, v) in valves {
    //     let conn = v.conn.iter().map(|c| (c.clone(), 1)).collect();
    //     let valve2 = Valve2 { r: v.r, conn };
    //     valves2.insert(name.clone(), valve2);
    // }

    let useful = valves
        .iter()
        .filter_map(|(k, v)| {
            if v.r > 0 || k == "AA" {
                Some((k.to_owned(), v.r))
            } else {
                None
            }
        })
        .collect::<HashMap<String, u32>>();

    let keys: Vec<String> = valves.keys().cloned().collect();
    let num_nodes = keys.len();
    let mut adjmat =
        Grid::new_with_value(num_nodes as u32, num_nodes as u32, 999999);

    for (key1, v) in valves {
        let i1 = keys.iter().position(|k| k == &key1).unwrap();

        v.conn.iter().for_each(|key2| {
            let i2 = keys.iter().position(|k| k == key2).unwrap();
            adjmat.set(i1 as u32, i2 as u32, 1);
            adjmat.set(i2 as u32, i1 as u32, 1);
        });
    }

    for i in 0..num_nodes {
        adjmat.set(i as u32, i as u32, 0);
    }

    // floyd
    for k in 0..num_nodes {
        for i in 0..num_nodes {
            for j in 0..num_nodes {
                let testdist = adjmat.get(i as u32, k as u32)
                    + adjmat.get(k as u32, j as u32);
                if adjmat.get(i as u32, j as u32) > testdist {
                    adjmat.set(i as u32, j as u32, testdist);
                }
            }
        }
    }

    let rates: HashMap<u32, u32> = useful
        .iter()
        .map(|(k, v)| {
            let index = keys.iter().position(|k2| k == k2).unwrap();
            let rate = v;

            (index as u32, *rate)
        })
        .collect();

    // let dist = Grid::new(useful.len(), useful.len());
    // for u in useful {

    // }

    // Add third action of staying still and banking movement
    // Clear 0 rate nodes
    // adjacency matrix?

    // println!("{}", max_score);

    // let mut turned_on = BTreeSet::new();
    // let mut visited = BTreeSet::new();
    // let mut visited = HashMap::new();

    let mut node_scores = HashMap::new();
    let mut start = HashMap::new();
    start.insert(BTreeSet::new(), (0, 0));
    let start_position: u32 =
        u32::try_from(keys.iter().position(|k| k == "AA").unwrap()).unwrap();
    node_scores.insert(start_position, start);
    // let mut iterations = Vec::new();
    let mut lastscore = 0;
    dbg!(&adjmat);
    dbg!(&rates);
    for i in 0..MAX_TIME {
        node_scores = solve(&rates, &adjmat, &node_scores);

        let ns = node_scores.clone();
        let mut max_score = 0;

        for (k, v) in ns {
            for (k, v) in v {
                max_score = max_score.max(v.0);
            }
        }
        println!("{} - {}", max_score, max_score - lastscore);
        lastscore = max_score;
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
            max_score = max_score.max(v.0);
        }
    }
    println!("{}", max_score);
    Ok(())
}
/// I think the idea is a backtracking alorithm that keeps track of an
/// additional arrayed state of having not visited any particular node.
fn solve(
    rates: &HashMap<u32, u32>,
    adjmat: &Grid<u32>,
    node_scores: &HashMap<u32, HashMap<BTreeSet<u32>, (u32, u32)>>,
) -> HashMap<u32, HashMap<BTreeSet<u32>, (u32, u32)>> {
    let mut next_node_scores = HashMap::new();
    let adv_node_scores: HashMap<u32, HashMap<BTreeSet<u32>, (u32, u32)>> =
        node_scores
            .iter()
            .map(|(k, v)| {
                (
                    *k,
                    v.iter()
                        .map(|(inner_k, v)| {
                            (
                                inner_k.clone(),
                                (v.0 + rate_per_time(rates, inner_k), v.1 + 1),
                            )
                        })
                        .collect(),
                )
            })
            .collect();
    for location in rates.keys() {
        let mut active_node_scores =
            merge_connection_node_scores(*location, adjmat, &adv_node_scores);

        // Started on location
        if let Some(started_on_location) = adv_node_scores.get(location) {
            active_node_scores =
                merge_scores(&active_node_scores, started_on_location);
        };

        if !active_node_scores.is_empty() {
            next_node_scores.insert(location.clone(), active_node_scores);
            // let cleaned_scores = cleanup_scores(&active_node_scores);
            // next_node_scores.insert(location.clone(), cleaned_scores);
        }
    }

    next_node_scores
}

fn rate_per_time(rates: &HashMap<u32, u32>, on_nodes: &BTreeSet<u32>) -> u32 {
    on_nodes
        .iter()
        .map(|k| rates.get(k).unwrap())
        .fold(0, |acc, r| acc + r)
}

// fn connecting_locations(
//     flows: &HashMap<u32, Valve>,
//     adjmat: &Grid<u32>,
//     moves: &HashMap<u32, Valve>,
//     location: &u32,
// ) -> Vec<u32> {
//     let dist = flows.get(location).unwrap();
// }
fn cleanup_scores(
    scores: &HashMap<BTreeSet<u32>, (u32, u32)>,
) -> HashMap<BTreeSet<u32>, (u32, u32)> {
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
    scores1: &HashMap<BTreeSet<u32>, (u32, u32)>,
    scores2: &HashMap<BTreeSet<u32>, (u32, u32)>,
) -> HashMap<BTreeSet<u32>, (u32, u32)> {
    scores2.iter().fold(scores1.clone(), |mut acc, (k, v)| {
        let val = acc.get(k).cloned().unwrap_or_default();
        acc.insert(k.clone(), val.max(*v));
        acc
    })
}

fn merge_connection_node_scores(
    location: u32,
    adjmat: &Grid<u32>,
    node_scores: &HashMap<u32, HashMap<BTreeSet<u32>, (u32, u32)>>,
) -> HashMap<BTreeSet<u32>, (u32, u32)> {
    node_scores
        .iter()
        .flat_map(|(key, scores)| {
            let dist_plus_on = adjmat.get(location, *key) + 1;
            scores.iter().filter(move |(k, v)| {
                !k.contains(&location) && v.1 == dist_plus_on
            })
        })
        .fold(
            HashMap::<BTreeSet<u32>, (u32, u32)>::new(),
            |mut acc, (k, v)| {
                let mut value = v.clone();
                let mut new_key = k.to_owned();
                new_key.insert(location);

                if let Some(val) = acc.get(&new_key) {
                    if value.0 < val.0 {
                        value = *val;
                    }
                }
                value.1 = 0;

                acc.insert(new_key, value);
                acc
            },
        )
}

// fn advance_scores(
//     scores: &HashMap<BTreeSet<String>, u32>,
//     valves: &HashMap<String, Valve>,
// ) -> HashMap<BTreeSet<String>, u32> {
//     let mut new_scores = HashMap::new();
//     for (k, score) in scores.iter() {
//         let next_score = score + rate_per_time(valves, k);
//         new_scores.insert(k.clone(), next_score);
//     }
//     new_scores
// }
