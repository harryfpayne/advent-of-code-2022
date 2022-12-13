use std::collections::{HashMap, VecDeque};
use crate::parse::{MonkeyAround, Operation, Operator, parse_input};

#[derive(Debug)]
struct Monkey {
    id: String,
    items: VecDeque<i64>,
    operation: (Operator, Operation),
    divisor_test: i64,
    receiver_if_true: String,
    receiver_if_false: String,
}

impl Monkey {
    fn who_will_thrown_to(&self, global_mod: i64) -> Vec<(String, i64)> {
        let mut thrown_items = vec![];
        for unmod_item in self.items.iter() {
            let item = unmod_item % global_mod;
            let after_operation = match self.operation.0 {
                Operator::Old => {
                    match self.operation.1 {
                        Operation::Add => item + item,
                        Operation::Multiply => item * item,
                    }
                },
                Operator::N(x) => {
                    match self.operation.1 {
                        Operation::Add => item + x as i64,
                        Operation::Multiply => item * x as i64,
                    }
                },
            };

            let after_inspection = after_operation;

            if after_inspection % self.divisor_test == 0 {
                thrown_items.push((self.receiver_if_true.clone(), after_inspection))
            } else {
                thrown_items.push((self.receiver_if_false.clone(), after_inspection))
            }
        }

        thrown_items
    }
    fn throw(&mut self) {
        self.items.pop_front();
    }
    fn catch(&mut self, item: i64) {
        self.items.push_back(item)
    }
}
impl MonkeyAround<Monkey> for Monkey {
    fn from(id: String, items: VecDeque<i32>, operation: (Operator, Operation), divisor_test: i32, receiver_if_true: String, receiver_if_false: String) -> Monkey {
        Monkey{
            id,
            items: items.into_iter().map(|a| a as i64).collect::<VecDeque<i64>>(),
            operation,
            divisor_test: divisor_test as i64,
            receiver_if_true,
            receiver_if_false,
        }
    }
}

pub fn question_2(input: &str) -> i64 {
    let (mut monkeys, order) = parse_input::<Monkey>(input);

    let mut global_mod = 1;
    for (_, m) in monkeys.iter() {
        global_mod *= m.divisor_test
    }

    let mut monkey_inspection_count: HashMap<&String, i32> = HashMap::new();
    for round_num in 0..10000 {
        for monkey_id in order.iter() {
            let curr_monkey = monkeys.get(monkey_id).expect("invalid monkey_id");
            let thrown_items = curr_monkey.who_will_thrown_to(global_mod);
            let count = monkey_inspection_count.entry(monkey_id).or_insert(0);
            *count += thrown_items.len() as i32;

            for (to_monkey_id, item) in thrown_items {
                let [m1, m2] = monkeys.
                    get_many_mut([monkey_id.as_str(), to_monkey_id.as_str()]).
                    expect("invalid monkey ids");
                m1.throw();
                m2.catch(item);
            }
        }
    }
    let mut counts = monkey_inspection_count.values().collect::<Vec<&i32>>();
    counts.sort();
    let most = counts[counts.len()-1];
    let second_most = counts[counts.len()-2];

    (*most as i64) * (*second_most as i64)
}

