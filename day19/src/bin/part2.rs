#![feature(iter_advance_by)]
use std::{
    collections::{BTreeSet, HashSet},
    hash::Hash,
    io,
};

const MAX_NUM: u32 = 9999;
const TOTAL_TIME: u32 = 32;

#[derive(Default, PartialEq, Eq, Hash, Copy, Clone, PartialOrd, Ord)]
struct Strategy {
    or: u32,
    cl: u32,
    ob: u32,
    ge: u32,

    /// a Robot is effective -1 for each associated cost
    or_robot: u32,
    cl_robot: u32,
    ob_robot: u32,
    ge_robot: u32,

    // Waiting and building something you could have makes no sense
    just_waited: bool,
}

fn main() -> io::Result<()> {
    // Blueprint 1:
    //   Each ore robot costs 4 ore.
    //   Each clay robot costs 4 ore.
    //   Each obsidian robot costs 4 ore and 20 clay.
    //   Each geode robot costs 2 ore and 12 obsidian.

    // Blueprint 22:
    //   Each ore robot costs 4 ore.
    //   Each clay robot costs 4 ore.
    //   Each obsidian robot costs 3 ore and 6 clay.
    //   Each geode robot costs 2 ore and 14 obsidian.

    let mut score = 1;
    let mut blueprint = 1;
    for line in io::stdin().lines() {
        let line = line.unwrap();
        let mut words = line.split(" ");

        words.advance_by(6).unwrap();
        let or_or_cost: u32 = words.next().unwrap().parse().unwrap();
        words.advance_by(5).unwrap();
        let cl_or_cost: u32 = words.next().unwrap().parse().unwrap();
        words.advance_by(5).unwrap();
        let ob_or_cost: u32 = words.next().unwrap().parse().unwrap();
        words.advance_by(2).unwrap();
        let ob_cl_cost: u32 = words.next().unwrap().parse().unwrap();
        words.advance_by(5).unwrap();
        let ge_or_cost: u32 = words.next().unwrap().parse().unwrap();
        words.advance_by(2).unwrap();
        let ge_ob_cost: u32 = words.next().unwrap().parse().unwrap();

        /// Greedily choose strategies by cutting redundant inferior ones
        let mut strats = BTreeSet::new();

        strats.insert(Strategy {
            or_robot: 1,
            ..Default::default()
        });

        for i in 0..TOTAL_TIME {
            strats = solve(
                &mut strats,
                or_or_cost,
                cl_or_cost,
                ob_or_cost,
                ob_cl_cost,
                ge_or_cost,
                ge_ob_cost,
                TOTAL_TIME - i,
            );

            println!("{}: {:?}", i, strats.len());
            strats = cull_strats(&mut strats);
            println!("aftercull: {:?}", strats.len());
            // let mut max_or = 0;
            // let mut max_cl = 0;
            // let mut max_ob = 0;
            // let mut max_ge = 0;
            // for s in strats.iter() {
            //     max_or = max_or.max(s.or);
            //     max_cl = max_cl.max(s.cl);
            //     max_ob = max_ob.max(s.ob);
            //     max_ge = max_ge.max(s.ge);
            // }
            // println!("Max Or: {}", max_or);
            // println!("Max Cl: {}", max_cl);
            // println!("Max Ob: {}", max_ob);
            // println!("Max Ge: {}", max_ge);
        }

        let mut max_ge = 0;
        for s in strats.iter() {
            max_ge = max_ge.max(s.ge);
        }
        println!("Blueprint: {} Max Ge: {}", blueprint, max_ge);
        score *= max_ge;
        blueprint += 1;
    }

    println!("Score: {}", score);

    Ok(())
}

fn solve(
    strats: &mut BTreeSet<Strategy>,
    or_or_cost: u32,
    cl_or_cost: u32,
    ob_or_cost: u32,
    ob_cl_cost: u32,
    ge_or_cost: u32,
    ge_ob_cost: u32,
    time_left: u32,
) -> BTreeSet<Strategy> {
    let mut next_strats = BTreeSet::new();

    // * if you can build geode, you will always take it
    //   - any strat with less geo bots can be cut (is this true?)
    // * optimal time to build is once you receive enough resources
    //   - but you can get multiple available at once.

    // Spend
    for s in strats.iter() {
        let mut or = MAX_NUM.min(s.or + s.or_robot);
        let mut cl = MAX_NUM.min(s.cl + s.cl_robot);
        let mut ob = MAX_NUM.min(s.ob + s.ob_robot);
        let ge = s.ge + s.ge_robot;
        let or_robot = s.or_robot;
        let cl_robot = s.cl_robot;
        let ob_robot = s.ob_robot;
        let ge_robot = s.ge_robot;

        if s.or + s.or_robot * (time_left - 1)
            >= or_or_cost.max(cl_or_cost).max(ob_or_cost).max(ge_or_cost)
                * time_left
        {
            or = MAX_NUM;
        }
        if s.cl + s.cl_robot * (time_left - 1) >= ob_cl_cost * time_left {
            cl = MAX_NUM;
        }
        if s.ob + s.ob_robot * (time_left - 1) >= ge_ob_cost * time_left {
            ob = MAX_NUM;
        }

        if !(s.just_waited && s.or - s.or_robot >= or_or_cost)
            && s.or < MAX_NUM
            && or_or_cost <= s.or
        {
            next_strats.insert(Strategy {
                or: or - or_or_cost,
                cl,
                ob,
                ge,
                or_robot: or_robot + 1,
                cl_robot,
                ob_robot,
                ge_robot,
                ..Default::default()
            });
        }
        if !(s.just_waited && s.or - s.or_robot >= cl_or_cost)
            && s.cl < MAX_NUM
            && cl_or_cost <= s.or
        {
            next_strats.insert(Strategy {
                or: or - cl_or_cost,
                cl,
                ob,
                ge,
                or_robot,
                cl_robot: cl_robot + 1,
                ob_robot,
                ge_robot,
                ..Default::default()
            });
        }
        if !(s.just_waited
            && (s.or - s.or_robot >= ob_or_cost)
            && (s.cl - s.cl_robot >= ob_cl_cost))
            && s.ob < MAX_NUM
            && (ob_or_cost <= s.or && ob_cl_cost <= s.cl)
        {
            next_strats.insert(Strategy {
                or: or - ob_or_cost,
                cl: cl - ob_cl_cost,
                ob,
                ge,
                or_robot,
                cl_robot,
                ob_robot: ob_robot + 1,
                ge_robot,
                ..Default::default()
            });
        }
        if !(s.just_waited
            && (s.or - s.or_robot >= ge_or_cost)
            && (s.ob - s.ob_robot >= ge_ob_cost))
            && ge_or_cost <= s.or
            && ge_ob_cost <= s.ob
        {
            next_strats.insert(Strategy {
                or: or - ge_or_cost,
                cl,
                ob: ob - ge_ob_cost,
                ge,
                or_robot,
                cl_robot,
                ob_robot,
                ge_robot: ge_robot + 1,
                ..Default::default()
            });
        }

        // Wait
        next_strats.insert(Strategy {
            or,
            cl,
            ob,
            ge,
            or_robot,
            cl_robot,
            ob_robot,
            ge_robot,
            just_waited: true,
            ..Default::default()
        });
    }

    next_strats
}

// // * if strat is better in every way the worse one is uselss
// fn dedup_strats(strats: &mut BTreeSet<Strategy>) {
//     let set: HashSet<_> = strats.drain(..).collect();
//     strats.extend(set.into_iter());
// }

fn cull_strats(strats: &mut BTreeSet<Strategy>) -> BTreeSet<Strategy> {
    // dedup_strats(strats);
    let mut removed = BTreeSet::new();
    {
        for (i, s1) in strats.iter().enumerate() {
            for (j, s2) in strats.iter().enumerate() {
                if i == j {
                    continue;
                }

                if ((s1.or == MAX_NUM && s2.or == MAX_NUM)
                    || s1.or >= s2.or && s1.or_robot >= s2.or_robot)
                    && ((s1.cl == MAX_NUM && s2.cl == MAX_NUM)
                        || s1.cl >= s2.cl && s1.cl_robot >= s2.cl_robot)
                    && ((s1.ob == MAX_NUM && s2.ob == MAX_NUM)
                        || s1.ob >= s2.ob && s1.ob_robot >= s2.ob_robot)
                    && s1.ge >= s2.ge
                    && s1.ge_robot >= s2.ge_robot
                    && (s1.just_waited || !s2.just_waited)
                {
                    removed.insert(s2.clone());
                }
            }
        }
    }

    let mut newstrat = BTreeSet::new();
    for s in strats.iter() {
        if removed.contains(s) {
        } else {
            newstrat.insert(s.clone());
        }
    }
    newstrat
}
