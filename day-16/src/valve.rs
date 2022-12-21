use std::collections::{HashMap};

#[derive(Debug)]
pub struct Valve {
    pub(crate) name: String,
    pub(crate) flow_rate: i32,
    pub(crate) leads_to: Vec<usize>,
}

#[derive(Clone, Copy, Debug)]
pub struct State {
    pub(crate) position: usize,
    pub(crate) turns_until_can_move: i32,
    pub(crate) score_once_activated: i32,
}
impl State {
    pub fn is_waiting(&self) -> bool {
        self.turns_until_can_move == 0
    }
    pub fn dec(&self) -> State {
        State {
            position: self.position,
            turns_until_can_move: self.turns_until_can_move - 1,
            score_once_activated: self.score_once_activated,
        }
    }
}

pub fn get_distance_matrix(valves: &Vec<Valve>) -> Vec<Vec<i32>> {
    // floyd-warshall
    let num = valves.len();
    // Having to add to inf later which causes panic
    // In fully connected, uniform weight graph, max distance is num
    let max_distance = (num * num) as i32;
    let mut dist = vec![vec![max_distance; num]; num];

    for (i, valve) in valves.iter().enumerate() {
        dist[i][i] = 0;
        for j in valve.leads_to.iter() {
            dist[i][*j] = 1;
        }
    }

    for k in 0..num {
        for i in 0..num {
            for j in 0..num {
                if dist[i][j] > dist[i][k] + dist[k][j] {
                    dist[i][j] = dist[i][k] + dist[k][j]
                }
            }
        }
    }

    dist
}


pub fn parse_input(input: &str) -> (Vec<Valve>, usize) {
    let mut valves = vec![];
    let mut leads_to_list: HashMap<usize, Vec<String>> = HashMap::new();
    let mut letter_map: HashMap<String, usize> = HashMap::new();

    for line in input.lines() {
        let name = line.get(6..8).unwrap();
        let (left, right) = line.split_once("; ").unwrap();
        let (_, rate_str) = left.split_once("=").unwrap();
        let flow_rate = rate_str.parse::<i32>().unwrap();
        let leads_to = right
            .get(22..)
            .unwrap()
            .trim()
            .split(", ")
            .map(|s|String::from(s))
            .collect::<Vec<String>>();

        leads_to_list.insert(valves.len(), leads_to);
        letter_map.insert(name.to_string(), valves.len());
        valves.push(Valve {
            name: name.to_string(),
            flow_rate,
            leads_to: vec![],
        });
    }

    for i in 0..valves.len() {
        let leads_to = leads_to_list.get(&i).unwrap();
        for lead in leads_to {
            let idx = letter_map.get(lead).unwrap();
            valves[i].leads_to.push(*idx);
        }
    }
    (valves, *letter_map.get("AA").unwrap())
}