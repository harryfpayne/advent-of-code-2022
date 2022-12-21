// using 0 indexed turns
pub fn get_next_rock(turn: usize) -> Vec<(usize, usize)> {
    let formations: [Vec<(usize, usize)>; FORMATIONS_LENGTH] = [
        vec![(0,0), (0,1), (0,2), (0,3)],
        vec![(0,1), (1,0), (1,1), (1,2), (2,1)],
        vec![(0,0), (0,1), (0,2), (1,2), (2,2)],
        vec![(0,0), (1,0), (2,0), (3,0)],
        vec![(0,0), (0,1), (1,0), (1,1)],
    ];

    formations[turn % formations.len()].clone()
}

pub const FORMATIONS_LENGTH: usize = 5;