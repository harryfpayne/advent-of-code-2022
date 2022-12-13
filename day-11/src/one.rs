use std::collections::{HashMap, VecDeque};
use crate::parse::{Operation, Operator, parse_input, MonkeyAround};

#[derive(Debug)]
struct Monkey {
    id: String,
    items: VecDeque<i32>,
    operation: (Operator, Operation),
    divisor_test: i32,
    receiver_if_true: String,
    receiver_if_false: String,
}
impl Monkey {
    fn who_will_thrown_to(&self) -> Vec<(String, i32)> {
        let mut thrown_items = vec![];
        for item in self.items.iter() {
            let after_operation = match self.operation.0 {
                Operator::Old => {
                    match self.operation.1 {
                        Operation::Add => item + item,
                        Operation::Multiply => item * item,
                    }
                },
                Operator::N(x) => {
                    match self.operation.1 {
                        Operation::Add => item + x,
                        Operation::Multiply => item * x,
                    }
                },
            };

            let after_inspection = after_operation.div_floor(3);

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
    fn catch(&mut self, item: i32) {
        self.items.push_back(item)
    }
}
impl MonkeyAround<Monkey> for Monkey {
    fn from(id: String, items: VecDeque<i32>, operation: (Operator, Operation), divisor_test: i32, receiver_if_true: String, receiver_if_false: String) -> Monkey {
        Monkey{
            id,
            items,
            operation,
            divisor_test,
            receiver_if_true,
            receiver_if_false,
        }
    }
}

pub fn question_1(input: &str) -> i32 {
    let (mut monkeys, order) = parse_input::<Monkey>(input);

    let mut monkey_inspection_count: HashMap<&String, i32> = HashMap::new();
    for round_num in 0..20 {
        for monkey_id in order.iter() {
            let curr_monkey = monkeys.get(monkey_id).expect("invalid monkey_id");
            let thrown_items = curr_monkey.who_will_thrown_to();
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

    most * second_most
}