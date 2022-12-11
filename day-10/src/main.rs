
#[derive(Debug)]
enum Operation {
    Noop,
    Addx(i32),
}

fn parse_input(input: &str) -> Vec<Operation> {
    let lines = input.lines().collect::<Vec<&str>>();
    let ops = lines.iter().map(|s: &&str| {
        if s == &"noop" {
            return Operation::Noop;
        }
        let (op, num) = s.split_at(5);
        if op != "addx " {
            panic!("invalid op");
        }
        let inc = num.parse::<i32>().expect("invalid num");
        Operation::Addx(inc)
    }).collect::<Vec<Operation>>();

    ops
}

fn question_1(input: &str) -> i32 {
    let all_operations = parse_input(input);
    let mut operations = all_operations.iter();
    let mut x_register = 1;
    let mut cycles_spent_on_current_operation = 0;
    let mut current_operation = operations.next();

    let mut sum = 0;
    for cycle_count in 1..230 {
        // println!("{cycle_count}: X:{x_register} | Op:{:?}", current_operation);
        if (cycle_count - 20) % 40 == 0 {
            sum += cycle_count * x_register;
            println!("{cycle_count}: X:{x_register} | Op:{:?}", current_operation);
        }

        match current_operation {
            Some(Operation::Noop) => {
                cycles_spent_on_current_operation = 0;
                current_operation = operations.next();
                continue
            }
            Some(Operation::Addx(x)) =>  {
                if cycles_spent_on_current_operation == 1 {
                    cycles_spent_on_current_operation = 0;
                    x_register += x;
                    current_operation = operations.next();
                    continue
                }

                cycles_spent_on_current_operation += 1;
            }
            None => continue
        }

    }
    sum
}

fn question_2(input: &str) {
    let all_operations = parse_input(input);
    let mut operations = all_operations.iter();
    let mut x_register = 1;
    let mut cycles_spent_on_current_operation = 0;
    let mut current_operation = operations.next();

    let mut image = String::new();
    for cycle_count in 0..240 {
        let x_position = cycle_count % 40;
        if cycle_count % 40 == 0 {
            image += "\n";
        }
        if x_position + 1 >= x_register && x_position - 1 <= x_register {
            image += "#";
        } else {
            image += ".";
        }

        match current_operation {
            Some(Operation::Noop) => {
                cycles_spent_on_current_operation = 0;
                current_operation = operations.next();
                continue
            }
            Some(Operation::Addx(x)) =>  {
                if cycles_spent_on_current_operation == 1 {
                    cycles_spent_on_current_operation = 0;
                    x_register += x;
                    current_operation = operations.next();
                    continue
                }

                cycles_spent_on_current_operation += 1;
            }
            None => continue
        }
    }

    println!("{}",image);

    /*
    ###..   ###..   ###..   .##..   ###..   .##..   .##..   ####.
    #..#.   #..#.   #..#.   #..#.   #..#.   #..#.   #..#.   #....
    #..#.   ###..   #..#.   #..#.   #..#.   #..#.   #....   ###..
    ###..   #..#.   ###..   ####.   ###..   ####.   #.##.   #....
    #.#..   #..#.   #....   #..#.   #.#..   #..#.   #..#.   #....
    #..#.   ###..   #....   #..#.   #..#.   #..#.   .###.   #....
    R       B       P       A       R       A       G       F
    RBPARAGF
     */
}

fn main() {
    println!("{:?}", question_2(PUZZLE_INPUT));
}

const TEST_INPUT: &str = "\
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

const PUZZLE_INPUT: &str = "\
noop
noop
noop
addx 4
addx 1
addx 5
addx 1
addx 5
noop
addx -1
addx -6
addx 11
noop
noop
noop
noop
addx 6
addx 5
noop
noop
noop
addx -30
addx 34
addx 2
addx -39
noop
addx 5
addx 2
addx 19
addx -18
addx 2
addx 5
addx 2
addx 3
noop
addx 2
addx 3
noop
addx 2
addx 3
noop
addx 2
addx 3
noop
addx 2
addx -15
addx -22
noop
noop
addx 5
addx 2
noop
noop
addx 14
addx -11
addx 5
addx 2
addx 3
noop
addx 2
addx -16
addx 17
addx 2
addx 5
addx 2
addx -6
addx -25
addx 35
addx 1
addx -36
addx 1
addx 22
addx -19
addx 5
addx 2
noop
noop
addx 5
noop
noop
noop
addx 1
addx 4
noop
noop
noop
addx 5
noop
addx 1
addx 2
addx 3
addx 4
addx -34
addx 21
addx -24
addx 2
addx 5
addx 7
addx -6
addx 2
addx 30
addx -23
addx 10
addx -9
addx 2
addx 2
addx 5
addx -12
addx 13
addx 2
addx 5
addx 2
addx -12
addx -24
addx -1
noop
addx 3
addx 3
addx 1
addx 5
addx 21
addx -16
noop
addx 19
addx -18
addx 2
addx 5
addx 2
addx 3
noop
addx 3
addx -1
addx 1
addx 2
addx -18
addx 1
noop";
