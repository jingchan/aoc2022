#![feature(iter_advance_by)]
#![feature(option_get_or_insert_default)]

use std::{
    array,
    collections::{BTreeSet, HashMap, HashSet},
    io,
};

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
// O(K * V * V *  V * T)

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

        valves.insert(name, Valve { r, conn });
    }

    // let mut watch = BTreeSet::new();
    // watch.insert("BB".to_owned());
    // watch.insert("CC".to_owned());
    // watch.insert("DD".to_owned());
    // watch.insert("EE".to_owned());
    // watch.insert("HH".to_owned());
    // watch.insert("JJ".to_owned());
    let mut scores = HashMap::new();
    // for i in 0..4 {
    for i in 0..MAX_TIME {
        scores = solve(&mut valves, scores);

        // println!("-------{}-----------", i);
        // scores.iter().for_each(|(k, v)| {
        //     println!("::{}::", k);
        //     v.iter()
        //         .filter(|(k, v)| k.is_subset(&watch))
        //         .for_each(|(k, v)| {
        //             k.iter().for_each(|s| {
        //                 print!("{}", s);
        //             });
        //             println!(": {}", v);
        //         });
        // });
    }

    let scores_for_start = scores.get("AA").unwrap();

    let max_score = scores_for_start.iter().fold(0, |acc, (k, v)| acc.max(*v));

    // let start = valves.get("AA").unwrap();
    // let mut max_score = 0;
    // for cn in &start.conn {
    //     let c = valves.get(cn).unwrap();
    //     dbg!(c);

    //     let score = c.scores[(MAX_TIME - 1) as usize];
    //     max_score = max_score.max(score);

    //     let score = (MAX_TIME - 1) * start.r
    //         + c.scores_without[(MAX_TIME - 2) as usize].get("AA").unwrap();
    //     max_score = max_score.max(score);
    // }

    println!("{}", max_score);

    Ok(())
}
/// I think the idea is a backtracking alorithm that keeps track of an
/// additional arrayed state of having not visited any particular node.
fn solve(
    valves: &mut HashMap<String, Valve>,
    scores: HashMap<String, HashMap<BTreeSet<String>, u32>>,
) -> HashMap<String, HashMap<BTreeSet<String>, u32>> {
    let mut new_scores = HashMap::new();

    for k in valves.keys() {
        // Move
        if let Some(last_round_scores) = scores.get(k) {
            for connection in valves.get(k).unwrap().conn.iter() {
                if !new_scores.contains_key(connection) {
                    new_scores.insert(connection.clone(), HashMap::new());
                }
                let conn_scores = new_scores.get(connection).unwrap();

                let local_scores_adv =
                    advance_scores(&last_round_scores, valves);
                new_scores.insert(
                    connection.clone(),
                    update_scores(&local_scores_adv, &conn_scores),
                );
            }
        }

        // Turn on
        if valves.get(k).unwrap().r > 0 {
            let mut turn_on_scores = HashMap::new();
            let mut new_key = BTreeSet::new();
            new_key.insert(k.clone());
            turn_on_scores.insert(new_key, 0);
            if let Some(last_round_scores) = scores.get(k) {
                let local_scores_adv =
                    advance_scores(&last_round_scores, valves);
                local_scores_adv
                    .iter()
                    .filter(|(inner_k, v)| inner_k.contains(k))
                    .map(|(inner_k, v)| {
                        let mut new_key = inner_k.clone();
                        new_key.insert(k.clone());
                        (new_key, *v)
                    })
                    .for_each(|(inner_k, v)| {
                        turn_on_scores.insert(inner_k.clone(), v);
                    });
            }

            let new_scores_for_node =
                if let Some(new_score_for_node) = new_scores.get(k) {
                    update_scores(new_score_for_node, &turn_on_scores)
                } else {
                    turn_on_scores
                };
            new_scores.insert(k.clone(), new_scores_for_node);
        }
    }

    new_scores
}

//       start
// 2t      |
// 1000f  /
// 1000-1<-@
//      |  cost 3t, 1000flow; 4t, 1500f
//      * 500 - 4t
//      |
//      1000 - 2t, 1000f

// t left, per location
// t=2
// t=3
// t=4

//         AA
//         |
//  4000---10 --- 3000
//  \      |
//   \     |
//    \---5000

// t=3
//         AA
//6k         |
//  3000---10 --- 3000 6k
//  \      | 5k
//   \     |
//    \---5000 10k

// t=4
//             AA
//9k + 5k       |
//  3000-------10 --- 3000 9k + 10
//  \          | 5k + 30
//   \         |
//    \-------5000 15k + 3k

// For each edge, there are 4 possible states, one side is on x2, both on, or both off

// fn equals(a: HashSet<u32>, b: HashSet<u32>) {
//     a.eq(&b);
// }
fn rate_per_time(
    on_nodes: &BTreeSet<String>,
    valves: &HashMap<String, Valve>,
) -> u32 {
    on_nodes
        .iter()
        .map(|k| valves.get(k).unwrap().r)
        .fold(0, |acc, r| acc + r)
}

fn advance_scores(
    scores: &HashMap<BTreeSet<String>, u32>,
    valves: &HashMap<String, Valve>,
) -> HashMap<BTreeSet<String>, u32> {
    let mut new_scores = HashMap::new();
    for (k, score) in scores.iter() {
        let next_score = score + rate_per_time(k, valves);
        new_scores.insert(k.clone(), next_score);
    }
    new_scores
}

fn update_scores(
    move_out_scores: &HashMap<BTreeSet<String>, u32>,
    dest_scores: &HashMap<BTreeSet<String>, u32>,
) -> HashMap<BTreeSet<String>, u32> {
    let mut new_dest_scores = HashMap::new();
    for (k, score) in move_out_scores.iter() {
        let next_score = if let Some(dest_score) = dest_scores.get(k) {
            *dest_score.max(&score)
        } else {
            *score
        };
        new_dest_scores.insert(k.clone(), next_score);
    }
    new_dest_scores
}
