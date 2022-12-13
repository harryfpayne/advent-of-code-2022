use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
pub enum Operation {
    Add,
    Multiply
}
#[derive(Debug)]
pub enum Operator {
    Old,
    N(i32),
}

pub trait MonkeyAround<T> {
    fn from(
        id: String,
        items: VecDeque<i32>,
        operation: (Operator, Operation),
        divisor_test: i32,
        receiver_if_true: String,
        receiver_if_false: String,
    ) -> T;
}

pub fn parse_operation_string(op_str: &str) -> (Operator, Operation) {
    let important_bit = op_str.get(21..).expect("invalid op_str");
    let (operator, multiplier_str) = important_bit.split_at(1);
    let multiplier = multiplier_str.trim_start().parse::<i32>();

    let o = match operator {
        "+" => Operation::Add,
        "*" => Operation::Multiply,
        _ => panic!("invalid operation {}", operator)
    };
    let m = match multiplier {
        Ok(n) => Operator::N(n),
        Err(_) => Operator::Old,
    };

    (m,o)
}

pub fn parse_input<T: MonkeyAround<T>>(input: &str) -> (HashMap<String, T>, Vec<String>) {
    let monkey_descriptions: Vec<Vec<&str>> = input.lines().fold(vec![vec![]], |mut acc: Vec<Vec<&str>>, line: &str| {
        if line == "" {
            acc.push(vec![]);
            return acc;
        } else {
            let last = acc.len() - 1;
            acc[last].push(line.trim());
            acc
        }
    });

    let mut monkeys = HashMap::new();
    let mut monkey_order = vec![];
    for desc in monkey_descriptions {
        let id = desc[0].trim_end_matches(":");
        let items = desc[1].split_at(16).1.split(", ").map(|i| i.parse::<i32>().expect("invalid number")).collect::<VecDeque<i32>>();
        let operation = parse_operation_string(desc[2]);
        let divisor = desc[3].split_at(19).1.parse::<i32>().expect("invalid divisor");
        let mut t = String::from("M");
        t.push_str(desc[4].split_at(19).1);
        let mut f = String::from("M");
        f.push_str(desc[5].split_at(20).1);

        let m = T::from(
            String::from(id),
            items,
            operation,
            divisor,
            t,
            f,
        );
        monkeys.insert(String::from(id), m);
        monkey_order.push(String::from(id));
    }

    (monkeys, monkey_order)
}