use std::collections::{HashMap, HashSet, VecDeque};
use crate::valve::{get_distance_matrix, parse_input, State, Valve};
mod valve;
mod question1;

fn main() {
    let (valves, start) = parse_input(PUZZLE_INPUT);
    let distances = get_distance_matrix(&valves);
    let valves_worth_visiting = valves
        .iter()
        .enumerate()
        .filter(|(i, v)| v.flow_rate > 0)
        .map(|(i,_)| i)
        .collect::<HashSet<usize>>();

    let m_current = State {
        position: start.clone(),
        turns_until_can_move: 0,
        score_once_activated: 0,
    };
    let e_current = State {
        position: start.clone(),
        turns_until_can_move: 0,
        score_once_activated: 0,
    };

    println!("{:?}", search_q2(
        &valves,
        &distances,
        valves_worth_visiting,
        m_current,
        e_current,
        26,
        0,
    ))
}

fn search_q2(
    valves: &Vec<Valve>,
    distances: &Vec<Vec<i32>>,
    valves_worth_visiting: HashSet<usize>,
    m_current: State,
    e_current: State,
    turns_left: i32,
    score: i32,
) -> i32  {
    let mut score = score;
    let mut moved_someone = false;

    let mut best_score = score;
    if m_current.is_waiting() {
        score = score + m_current.score_once_activated;
        best_score = score;

        for valve in valves_worth_visiting.iter() {
            let valve = *valve;
            let distance_to_activate = distances[m_current.position][valve] + 1;
            let turns_left_after_activate = turns_left - distance_to_activate;
            if turns_left_after_activate < 0 {
                continue
            }
            let next_valve = valves.get(valve).unwrap();
            let score_once_activated = next_valve.flow_rate * turns_left_after_activate;
            let mut next_valves_worth_visiting = valves_worth_visiting.clone();
            next_valves_worth_visiting.remove(&valve);
            let next_state = State {
                position: valve,
                turns_until_can_move: distance_to_activate,
                score_once_activated,
            };
            moved_someone = true;

            let next_score = search_q2(
                valves,
                distances,
                next_valves_worth_visiting,
                next_state,
                e_current,
                turns_left,
                score,
            );
            if next_score > best_score {
                best_score = next_score;
            }
        }
    } else if e_current.is_waiting() {
        score = score + e_current.score_once_activated;
        best_score = score.clone();
        for valve in valves_worth_visiting.iter() {
            let valve = *valve;
            let distance_to_activate = distances[e_current.position][valve] + 1;
            let turns_left_after_activate = turns_left - distance_to_activate;
            if turns_left_after_activate < 0 {
                continue
            }
            let next_valve = valves.get(valve).unwrap();
            let score_once_activated = next_valve.flow_rate * turns_left_after_activate;
            let mut next_valves_worth_visiting = valves_worth_visiting.clone();
            next_valves_worth_visiting.remove(&valve);
            let next_state = State {
                position: valve,
                turns_until_can_move: distance_to_activate,
                score_once_activated,
            };
            moved_someone = true;

            let next_score = search_q2(
                valves,
                distances,
                next_valves_worth_visiting,
                m_current,
                next_state,
                turns_left,
                score,
            );
            if next_score > best_score {
                best_score = next_score;
            }
        }
    }

    if turns_left > 0 && !moved_someone {
        let next_score = search_q2(
            valves,
            distances,
            valves_worth_visiting,
            m_current.dec(),
            e_current.dec(),
            turns_left - 1,
            score,
        );
        if next_score > best_score {
            best_score = next_score;
        }
    }

    best_score
}

fn question_2() {

}

const TEST_INPUT: &str = "\
Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

const PUZZLE_INPUT: &str = "\
Valve ED has flow rate=0; tunnels lead to valves PS, AW
Valve SI has flow rate=0; tunnels lead to valves AA, HX
Valve LX has flow rate=22; tunnels lead to valves DY, YH
Valve CR has flow rate=0; tunnels lead to valves BE, HX
Valve BI has flow rate=0; tunnels lead to valves GC, AY
Valve PB has flow rate=4; tunnels lead to valves IX, YG, RI, KR, BV
Valve YY has flow rate=0; tunnels lead to valves PH, GJ
Valve PH has flow rate=11; tunnels lead to valves YY, VE, ZG, MM
Valve DY has flow rate=0; tunnels lead to valves LX, AW
Valve SD has flow rate=0; tunnels lead to valves AY, EC
Valve SV has flow rate=24; tunnels lead to valves CC, GF
Valve RL has flow rate=0; tunnels lead to valves OW, IN
Valve GF has flow rate=0; tunnels lead to valves RQ, SV
Valve BE has flow rate=5; tunnels lead to valves CR, JC, MF, IT
Valve PR has flow rate=0; tunnels lead to valves BV, GJ
Valve AW has flow rate=21; tunnels lead to valves VE, DY, TR, ED
Valve FY has flow rate=17; tunnels lead to valves GG, KJ
Valve GC has flow rate=0; tunnels lead to valves BI, GJ
Valve RI has flow rate=0; tunnels lead to valves PB, AY
Valve RQ has flow rate=0; tunnels lead to valves HH, GF
Valve IT has flow rate=0; tunnels lead to valves MZ, BE
Valve XG has flow rate=0; tunnels lead to valves BL, AA
Valve MK has flow rate=0; tunnels lead to valves HX, DV
Valve IX has flow rate=0; tunnels lead to valves PB, JC
Valve BV has flow rate=0; tunnels lead to valves PR, PB
Valve TR has flow rate=0; tunnels lead to valves CD, AW
Valve PS has flow rate=0; tunnels lead to valves ED, AY
Valve HH has flow rate=12; tunnels lead to valves RQ, NL, ZQ
Valve AA has flow rate=0; tunnels lead to valves KR, SI, XG, EC, ZG
Valve FT has flow rate=0; tunnels lead to valves IN, YH
Valve YG has flow rate=0; tunnels lead to valves PB, HX
Valve HX has flow rate=14; tunnels lead to valves MK, ZQ, YG, SI, CR
Valve DV has flow rate=0; tunnels lead to valves MK, QR
Valve GJ has flow rate=3; tunnels lead to valves PR, CD, YY, GC, BL
Valve BL has flow rate=0; tunnels lead to valves GJ, XG
Valve CD has flow rate=0; tunnels lead to valves TR, GJ
Valve GG has flow rate=0; tunnels lead to valves FY, NL
Valve JC has flow rate=0; tunnels lead to valves IX, BE
Valve JN has flow rate=0; tunnels lead to valves OW, QR
Valve RM has flow rate=18; tunnel leads to valve KJ
Valve NL has flow rate=0; tunnels lead to valves GG, HH
Valve QR has flow rate=20; tunnels lead to valves CC, DV, PN, JN
Valve ZG has flow rate=0; tunnels lead to valves AA, PH
Valve AY has flow rate=6; tunnels lead to valves RI, PS, SD, BI, MM
Valve VE has flow rate=0; tunnels lead to valves PH, AW
Valve OW has flow rate=25; tunnels lead to valves MZ, RL, JN
Valve MM has flow rate=0; tunnels lead to valves AY, PH
Valve KJ has flow rate=0; tunnels lead to valves RM, FY
Valve MF has flow rate=0; tunnels lead to valves BE, PN
Valve YH has flow rate=0; tunnels lead to valves LX, FT
Valve ZQ has flow rate=0; tunnels lead to valves HX, HH
Valve KR has flow rate=0; tunnels lead to valves AA, PB
Valve PN has flow rate=0; tunnels lead to valves MF, QR
Valve CC has flow rate=0; tunnels lead to valves SV, QR
Valve MZ has flow rate=0; tunnels lead to valves OW, IT
Valve EC has flow rate=0; tunnels lead to valves SD, AA
Valve IN has flow rate=16; tunnels lead to valves RL, FT";
