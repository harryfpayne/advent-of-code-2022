#![feature(iter_advance_by)]

fn main() {
    let (mut stack, ops) = parse_input(PUZZLE_INPUT);

    for x in ops {
        stack.move_crates_q2(x.amount, x.start, x.end);
    }

    println!("{}", stack.get_top_crates())
}

#[derive(Debug)]
struct Stack {
    crates: Vec<String>
}

impl Stack {
    fn pop_q1(&mut self, n: i32) -> Vec<String> {
        let final_length = self.crates.len().saturating_sub(n as usize);
        let mut tail = self.crates.split_off(final_length);
        tail.reverse();
        tail
    }
    fn pop_q2(&mut self, n: i32) -> Vec<String> {
        let final_length = self.crates.len().saturating_sub(n as usize);
        let mut tail = self.crates.split_off(final_length);
        tail
    }
    fn push(&mut self, new: Vec<String>) {
        self.crates.extend(new);
    }
    fn top_of_stack(&self) -> &str {
        self.crates.last().unwrap()
    }
}

#[derive(Debug)]
struct AllStack {
    all_stacks: Vec<Stack>
}

impl AllStack {
    fn move_crates_q1(&mut self, amount: i32, start: i32, end: i32) {
        let s = start as usize - 1;
        let e = end as usize - 1;
        if s > self.all_stacks.len() || e > self.all_stacks.len() {
            panic!("invalid start or end");
        }
        let moving = self.all_stacks[s].pop_q1(amount);
        self.all_stacks[e].push(moving)
    }
    fn move_crates_q2(&mut self, amount: i32, start: i32, end: i32) {
        let s = start as usize - 1;
        let e = end as usize - 1;
        if s > self.all_stacks.len() || e > self.all_stacks.len() {
            panic!("invalid start or end");
        }
        let moving = self.all_stacks[s].pop_q2(amount);
        self.all_stacks[e].push(moving)
    }

    fn get_top_crates(&self) -> String {
        let mut out = String::new();
        for x in self.all_stacks.iter() {
            out.push_str(x.top_of_stack())
        }

        out
    }
}

struct Operation {
    amount: i32,
    start: i32,
    end: i32,
}
impl Operation {
    fn from_string(s: &str) -> Operation {
        let sections = s.split(" ").collect::<Vec<&str>>();
        let amount = sections[1].parse::<i32>();
        let start = sections[3].parse::<i32>();
        let end = sections[5].parse::<i32>();

        Operation{
            amount: amount.unwrap(),
            start: start.unwrap(),
            end: end.unwrap(),
        }
    }
}

fn parse_input(input: &str) -> (AllStack, Vec<Operation>) {
    let sections = input.split("\n\n").collect::<Vec<&str>>();
    let crate_section = sections[0];
    let operation_section = sections[1];

    let mut all_stack = AllStack{all_stacks: vec![]};
    let mut crates_strings = crate_section.split("\n").collect::<Vec<&str>>();
    let last_row = crates_strings.pop().unwrap();
    for _ in last_row.split("   ") {
        all_stack.all_stacks.push(Stack{crates: vec![]})
    }
    crates_strings.reverse();

    for row in crates_strings {
        if row.len() == 0 {
            continue
        }

        let str_range = (1..row.len()-1).step_by(4);
        let crate_range = 0..str_range.len();
        for (str_idx, crate_idx) in str_range.zip(crate_range) {
            let b = row.as_bytes()[str_idx] as char;
            if b != ' ' {
                all_stack.all_stacks[crate_idx].crates.push(String::from(b))
            }
        }
    }

    let mut operations: Vec<Operation> = vec![];
    for operation_string in operation_section.split("\n") {
        operations.push(Operation::from_string(operation_string))
    }

    (all_stack, operations)
}

const TEST_INPUT: &str = "
    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

const PUZZLE_INPUT: &str = "
[M] [H]         [N]
[S] [W]         [F]     [W] [V]
[J] [J]         [B]     [S] [B] [F]
[L] [F] [G]     [C]     [L] [N] [N]
[V] [Z] [D]     [P] [W] [G] [F] [Z]
[F] [D] [C] [S] [W] [M] [N] [H] [H]
[N] [N] [R] [B] [Z] [R] [T] [T] [M]
[R] [P] [W] [N] [M] [P] [R] [Q] [L]
 1   2   3   4   5   6   7   8   9

move 1 from 7 to 6
move 1 from 9 to 4
move 4 from 9 to 6
move 1 from 2 to 3
move 7 from 8 to 6
move 1 from 6 to 3
move 6 from 2 to 9
move 1 from 2 to 9
move 3 from 5 to 6
move 4 from 5 to 4
move 1 from 1 to 6
move 8 from 9 to 4
move 1 from 5 to 1
move 7 from 3 to 9
move 11 from 4 to 1
move 1 from 9 to 3
move 1 from 3 to 6
move 9 from 1 to 2
move 1 from 4 to 8
move 1 from 8 to 2
move 5 from 9 to 4
move 8 from 2 to 1
move 10 from 6 to 3
move 5 from 4 to 3
move 9 from 3 to 2
move 1 from 9 to 5
move 1 from 6 to 1
move 4 from 1 to 8
move 5 from 7 to 6
move 1 from 5 to 9
move 2 from 4 to 3
move 13 from 6 to 1
move 1 from 6 to 3
move 3 from 1 to 7
move 9 from 2 to 7
move 2 from 4 to 6
move 25 from 1 to 9
move 2 from 2 to 7
move 2 from 3 to 5
move 1 from 6 to 5
move 2 from 5 to 2
move 2 from 8 to 9
move 2 from 2 to 5
move 23 from 9 to 5
move 1 from 8 to 5
move 1 from 8 to 9
move 6 from 3 to 7
move 3 from 5 to 7
move 1 from 3 to 1
move 1 from 1 to 5
move 11 from 7 to 6
move 9 from 6 to 2
move 1 from 7 to 1
move 1 from 1 to 7
move 2 from 6 to 8
move 8 from 2 to 3
move 4 from 7 to 1
move 7 from 7 to 6
move 6 from 9 to 6
move 1 from 1 to 5
move 5 from 6 to 8
move 2 from 7 to 6
move 2 from 3 to 2
move 24 from 5 to 8
move 1 from 3 to 5
move 4 from 3 to 2
move 1 from 5 to 6
move 31 from 8 to 6
move 1 from 5 to 6
move 1 from 3 to 6
move 2 from 1 to 9
move 2 from 9 to 6
move 1 from 1 to 9
move 46 from 6 to 5
move 1 from 9 to 4
move 35 from 5 to 1
move 28 from 1 to 5
move 24 from 5 to 3
move 1 from 3 to 4
move 1 from 6 to 3
move 19 from 3 to 4
move 2 from 3 to 8
move 3 from 1 to 8
move 4 from 2 to 1
move 4 from 8 to 6
move 6 from 1 to 5
move 1 from 8 to 5
move 3 from 4 to 1
move 5 from 1 to 7
move 23 from 5 to 2
move 21 from 2 to 8
move 6 from 8 to 2
move 2 from 2 to 5
move 2 from 5 to 6
move 5 from 4 to 5
move 6 from 6 to 7
move 4 from 5 to 2
move 1 from 7 to 9
move 3 from 3 to 2
move 1 from 5 to 2
move 2 from 8 to 5
move 11 from 2 to 5
move 3 from 2 to 7
move 13 from 7 to 4
move 11 from 8 to 1
move 1 from 9 to 5
move 23 from 4 to 2
move 1 from 4 to 9
move 10 from 1 to 2
move 1 from 9 to 5
move 1 from 1 to 3
move 2 from 8 to 6
move 4 from 5 to 9
move 19 from 2 to 5
move 3 from 9 to 2
move 28 from 5 to 7
move 1 from 3 to 5
move 1 from 9 to 5
move 15 from 7 to 5
move 2 from 6 to 4
move 2 from 4 to 3
move 19 from 5 to 9
move 5 from 7 to 5
move 8 from 7 to 8
move 1 from 8 to 1
move 14 from 9 to 6
move 2 from 8 to 5
move 1 from 3 to 8
move 3 from 5 to 9
move 1 from 1 to 9
move 3 from 9 to 6
move 8 from 6 to 5
move 1 from 8 to 1
move 1 from 8 to 3
move 13 from 2 to 4
move 4 from 9 to 8
move 4 from 4 to 1
move 1 from 6 to 1
move 2 from 3 to 4
move 2 from 1 to 7
move 10 from 5 to 1
move 2 from 5 to 2
move 7 from 4 to 7
move 6 from 6 to 7
move 1 from 9 to 7
move 3 from 7 to 1
move 7 from 2 to 7
move 1 from 6 to 3
move 1 from 6 to 9
move 8 from 7 to 8
move 2 from 7 to 6
move 8 from 7 to 9
move 17 from 1 to 7
move 13 from 8 to 5
move 2 from 7 to 1
move 2 from 6 to 3
move 9 from 7 to 6
move 5 from 7 to 6
move 1 from 4 to 5
move 3 from 5 to 9
move 4 from 9 to 2
move 2 from 8 to 6
move 1 from 7 to 9
move 4 from 9 to 1
move 12 from 6 to 2
move 10 from 2 to 6
move 4 from 9 to 4
move 6 from 1 to 6
move 2 from 7 to 8
move 2 from 8 to 4
move 1 from 8 to 1
move 8 from 4 to 7
move 5 from 5 to 2
move 3 from 4 to 1
move 3 from 2 to 8
move 2 from 8 to 4
move 1 from 4 to 5
move 3 from 2 to 1
move 2 from 9 to 8
move 11 from 6 to 5
move 4 from 7 to 2
move 1 from 3 to 7
move 1 from 8 to 5
move 8 from 6 to 4
move 2 from 3 to 7
move 1 from 6 to 2
move 15 from 5 to 3
move 15 from 3 to 5
move 5 from 1 to 6
move 12 from 2 to 8
move 4 from 7 to 3
move 4 from 6 to 3
move 7 from 4 to 3
move 8 from 3 to 8
move 1 from 6 to 8
move 10 from 5 to 3
move 8 from 5 to 4
move 15 from 3 to 9
move 1 from 1 to 3
move 9 from 4 to 9
move 1 from 7 to 3
move 2 from 7 to 6
move 1 from 9 to 7
move 19 from 8 to 2
move 1 from 1 to 9
move 4 from 3 to 9
move 1 from 5 to 6
move 4 from 8 to 1
move 1 from 4 to 1
move 3 from 1 to 3
move 1 from 1 to 9
move 4 from 9 to 7
move 2 from 6 to 1
move 2 from 1 to 2
move 1 from 6 to 3
move 1 from 1 to 4
move 3 from 7 to 5
move 21 from 2 to 8
move 1 from 7 to 8
move 2 from 5 to 3
move 1 from 4 to 3
move 3 from 3 to 1
move 1 from 7 to 5
move 1 from 1 to 2
move 1 from 1 to 2
move 2 from 3 to 2
move 1 from 3 to 8
move 2 from 5 to 6
move 1 from 3 to 9
move 4 from 2 to 8
move 12 from 9 to 6
move 1 from 1 to 4
move 14 from 6 to 1
move 3 from 9 to 1
move 1 from 4 to 7
move 4 from 8 to 6
move 3 from 6 to 4
move 3 from 4 to 7
move 15 from 1 to 5
move 1 from 6 to 5
move 12 from 5 to 4
move 10 from 9 to 8
move 3 from 7 to 8
move 1 from 9 to 1
move 2 from 1 to 7
move 17 from 8 to 5
move 10 from 4 to 2
move 16 from 5 to 8
move 30 from 8 to 7
move 4 from 5 to 2
move 4 from 7 to 1
move 1 from 5 to 8
move 4 from 8 to 4
move 5 from 4 to 8
move 8 from 7 to 8
move 19 from 7 to 5
move 4 from 1 to 4
move 7 from 5 to 3
move 10 from 2 to 3
move 5 from 5 to 1
move 1 from 5 to 3
move 4 from 2 to 8
move 4 from 4 to 6
move 1 from 5 to 7
move 3 from 7 to 1
move 1 from 4 to 2
move 7 from 3 to 7
move 2 from 5 to 1
move 1 from 2 to 8
move 3 from 5 to 2
move 3 from 2 to 7
move 11 from 1 to 9
move 9 from 9 to 6
move 1 from 3 to 8
move 2 from 9 to 6
move 3 from 3 to 7
move 3 from 7 to 1
move 5 from 6 to 7
move 14 from 7 to 6
move 1 from 7 to 2
move 5 from 3 to 5
move 1 from 3 to 4
move 2 from 1 to 4
move 1 from 6 to 9
move 1 from 3 to 8
move 1 from 9 to 2
move 1 from 1 to 4
move 4 from 4 to 9
move 1 from 2 to 3
move 5 from 5 to 9
move 1 from 9 to 5
move 1 from 5 to 3
move 11 from 6 to 3
move 2 from 9 to 1
move 1 from 1 to 7
move 5 from 6 to 4
move 4 from 3 to 9
move 1 from 3 to 7
move 1 from 4 to 2
move 1 from 4 to 5
move 2 from 2 to 1
move 1 from 4 to 5
move 2 from 1 to 6
move 1 from 3 to 6
move 8 from 9 to 6
move 19 from 8 to 7
move 2 from 7 to 4
move 1 from 1 to 3
move 6 from 6 to 5
move 1 from 8 to 6
move 8 from 5 to 9
move 1 from 9 to 8
move 1 from 4 to 6
move 1 from 9 to 1
move 4 from 7 to 5
move 2 from 4 to 7
move 1 from 4 to 5
move 8 from 9 to 5
move 3 from 8 to 2
move 8 from 6 to 8
move 5 from 3 to 1
move 6 from 8 to 3
move 9 from 5 to 7
move 3 from 2 to 4
move 1 from 6 to 1
move 2 from 3 to 9
move 2 from 8 to 1
move 1 from 4 to 7
move 1 from 5 to 6
move 1 from 9 to 3
move 8 from 3 to 8
move 2 from 4 to 9
move 2 from 5 to 7
move 5 from 8 to 3
move 2 from 6 to 9
move 1 from 9 to 5
move 3 from 9 to 3
move 3 from 6 to 5
move 1 from 9 to 6
move 1 from 8 to 3
move 4 from 5 to 4
move 24 from 7 to 5
move 8 from 3 to 1
move 24 from 5 to 2
move 3 from 4 to 6
move 5 from 6 to 3
move 1 from 3 to 1
move 1 from 5 to 2
move 4 from 2 to 1
move 5 from 3 to 9
move 1 from 4 to 3
move 5 from 2 to 3
move 3 from 1 to 2
move 1 from 7 to 1
move 4 from 7 to 8
move 1 from 1 to 2
move 5 from 2 to 8
move 2 from 9 to 8
move 19 from 1 to 7
move 9 from 8 to 9
move 2 from 3 to 5
move 8 from 9 to 6
move 5 from 6 to 2
move 1 from 3 to 8
move 2 from 9 to 5
move 3 from 5 to 9
move 5 from 9 to 4
move 2 from 6 to 4
move 2 from 8 to 3
move 1 from 5 to 6
move 3 from 8 to 4
move 1 from 6 to 9
move 8 from 4 to 3
move 19 from 7 to 5
move 5 from 3 to 6
move 1 from 4 to 5
move 1 from 4 to 7
move 1 from 9 to 1
move 4 from 6 to 8
move 1 from 7 to 5
move 2 from 6 to 4
move 4 from 8 to 5
move 6 from 3 to 1
move 6 from 5 to 8
move 5 from 5 to 1
move 2 from 4 to 7
move 2 from 3 to 2
move 7 from 5 to 2
move 1 from 7 to 9
move 3 from 2 to 6
move 7 from 2 to 1
move 4 from 1 to 7
move 7 from 1 to 7
move 11 from 2 to 4
move 3 from 6 to 7
move 2 from 8 to 5
move 8 from 7 to 3
move 6 from 3 to 5
move 4 from 2 to 3
move 3 from 7 to 6
move 3 from 2 to 5
move 7 from 5 to 1
move 10 from 1 to 6
move 1 from 2 to 8
move 3 from 6 to 7
move 4 from 4 to 1
move 2 from 3 to 6
move 3 from 3 to 9
move 1 from 3 to 6
move 4 from 1 to 4
move 3 from 9 to 6
move 2 from 4 to 1
move 9 from 4 to 7
move 11 from 7 to 4
move 6 from 1 to 6
move 6 from 4 to 7
move 5 from 4 to 7
move 4 from 8 to 1
move 1 from 8 to 6
move 1 from 9 to 7
move 4 from 6 to 4
move 5 from 5 to 4
move 5 from 5 to 9
move 5 from 1 to 6
move 1 from 5 to 6
move 4 from 9 to 7
move 1 from 9 to 8
move 7 from 7 to 1
move 1 from 7 to 8
move 4 from 1 to 5
move 5 from 4 to 1
move 1 from 4 to 8
move 6 from 1 to 2
move 11 from 6 to 8
move 2 from 8 to 9
move 1 from 5 to 9
move 6 from 2 to 8
move 1 from 1 to 2
move 2 from 7 to 8
move 1 from 9 to 2
move 2 from 2 to 8
move 1 from 7 to 8
move 10 from 8 to 3
move 3 from 5 to 9
move 4 from 8 to 5
move 4 from 8 to 2
move 7 from 7 to 8
move 2 from 5 to 9
move 1 from 5 to 1
move 2 from 7 to 8
move 5 from 3 to 5
move 1 from 1 to 3
move 1 from 1 to 6
move 1 from 2 to 4
move 7 from 6 to 4
move 2 from 2 to 3
move 3 from 8 to 4
move 2 from 3 to 1
move 3 from 5 to 6
move 3 from 6 to 8
move 1 from 1 to 9
move 3 from 3 to 1
move 8 from 8 to 1
move 1 from 2 to 9
move 1 from 6 to 2
move 3 from 5 to 1
move 1 from 8 to 3
move 3 from 4 to 1
move 4 from 8 to 9
move 1 from 7 to 1
move 7 from 1 to 6
move 8 from 9 to 6
move 1 from 8 to 9
move 4 from 9 to 8
move 15 from 6 to 5
move 3 from 1 to 6
move 2 from 1 to 2
move 1 from 2 to 7
move 1 from 9 to 6
move 3 from 8 to 1
move 1 from 4 to 9
move 11 from 5 to 9
move 1 from 7 to 1
move 1 from 2 to 3
move 2 from 3 to 4
move 6 from 1 to 7
move 7 from 4 to 5
move 2 from 6 to 7
move 1 from 4 to 5
move 2 from 4 to 1
move 13 from 9 to 1
move 2 from 3 to 2
move 1 from 3 to 7
move 2 from 4 to 1
move 4 from 6 to 9
move 1 from 8 to 4
move 4 from 6 to 8
move 1 from 4 to 9
move 9 from 1 to 6
move 8 from 6 to 9
move 4 from 5 to 3
move 1 from 8 to 4";
