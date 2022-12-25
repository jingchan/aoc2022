#![feature(iter_advance_by)]
use std::{
    array,
    collections::{HashMap, HashSet},
    hash::Hash,
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

    scores_without: [HashMap<String, u32>; MAX_TIME as usize],
    scores: [u32; MAX_TIME as usize],
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
        let scores = [0; MAX_TIME as usize];
        let scores_without = array::from_fn(|_| HashMap::new());

        let valve = Valve {
            r,
            conn,
            scores,
            scores_without,
        };
        dbg!(&valve);
        valves.insert(name, valve);
    }

    // initialize scoreswithout
    let keys: Vec<String> = valves.keys().map(|k| k.clone()).collect();
    for (k, v) in valves.iter_mut() {
        for k2 in keys.iter() {
            for i in 0..MAX_TIME {
                v.scores_without[i as usize].insert(k2.clone(), 0);
            }
        }
    }

    for i in 0..MAX_TIME {
        solve(&mut valves, i);
    }

    let start = valves.get("AA").unwrap();
    let mut max_score = 0;
    for cn in &start.conn {
        let c = valves.get(cn).unwrap();
        dbg!(c);

        let score = c.scores[(MAX_TIME - 1) as usize];
        max_score = max_score.max(score);

        let score = (MAX_TIME - 1) * start.r
            + c.scores_without[(MAX_TIME - 2) as usize].get("AA").unwrap();
        max_score = max_score.max(score);
    }

    println!("{}", max_score);

    Ok(())
}

/// I think the idea is a backtracking alorithm that keeps track of an additional arrayed state of having not visited any particular node.
/// jj
fn solve(valves: &mut HashMap<String, Valve>, time: u32) {
    // dbg!(&valves);
    let keys: Vec<String> = valves.keys().cloned().collect();

    for k in keys.iter() {
        let mut max_score = 0;
        if time > 1 {
            let mut new_scores_without = valves.get(k).unwrap().scores_without
                [(time - 1) as usize]
                .clone();

            // First option is to `move`:
            // We lose one time unit and preserve the scores from the best place we can move to.
            // This is striped by a restricted candidate. We can include all possible states including both this one being on or off.
            for cn in &valves.get(k).unwrap().conn {
                let c = valves.get(cn).unwrap();
                max_score = u32::max(max_score, c.scores[(time - 1) as usize]);

                for (k2, s2) in c.scores_without[(time - 1) as usize].iter() {
                    // New score is the max of each connected location.
                    let last_score = new_scores_without.get(k2).unwrap();
                    if *s2 > *last_score {
                        new_scores_without.insert(k2.clone(), *s2);
                    }
                }
            }

            // this is if we were to have been here already Turn on the valve
            // takes 1 time, we can compare this consider this value for all
            // scores ehappens for all restricted scores except for the one
            // where this location hasn't been durned on.  We can also the best
            // use of having come here previously (so long as turning on hasn't
            // been done before)
            let score = valves.get(k).unwrap().r * (time - 1)
                + valves.get(k).unwrap().scores_without[(time - 1) as usize]
                    .get(k)
                    .unwrap();
            for k2 in keys.iter() {
                if k == k2 {
                    continue;
                }
                let last_score = *new_scores_without.get(k2).unwrap();

                if score > last_score {
                    new_scores_without.insert(k2.to_owned(), score);
                }
            }

            // This is if we consider having moved from somewhere to turn this on.
            if time > 2 {
                for cn in &valves.get(k).unwrap().conn {
                    let c = valves.get(cn).unwrap();
                    max_score = u32::max(
                        max_score,
                        score
                            + c.scores_without[(time - 2) as usize]
                                .get(k)
                                .unwrap(),
                    );

                    for (k2, s2) in c.scores_without[(time - 2) as usize].iter()
                    {
                        if k2 == k {
                            continue;
                        }

                        let new_score =
                            if let Some(score2) = new_scores_without.get(k) {
                                u32::max(*s2, *score2)
                            } else {
                                *s2 + score
                            };

                        let last_score = *new_scores_without.get(k2).unwrap();
                        if new_score > last_score {
                            new_scores_without.insert(k2.clone(), new_score);
                        }
                    }
                }
            }

            std::mem::swap(
                &mut valves.get_mut(k).unwrap().scores_without
                    [(time - 2) as usize],
                &mut new_scores_without,
            );
        }

        valves.get_mut(k).unwrap().scores[time as usize] = max_score;
    }
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
