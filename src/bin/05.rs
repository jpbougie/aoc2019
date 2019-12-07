use std::collections::HashMap;

use std::io::{self, Read};

fn main() -> std::io::Result<()>{
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let program = input.split(",").map(|x| x.trim().parse().unwrap()).collect::<Vec<i64>>();

    println!("Part 01: {}", run(&program, 1));
    println!("Part 02: {}", run(&program, 5));

    Ok(())
}

#[derive(Default, Debug)]
struct State {
    inputs: Vec<i64>,
    outputs: Vec<i64>,
    program: HashMap<i64, i64>,
    pc: i64,
}

impl State {
    pub fn from(program: &[i64], input: i64) -> Self {
        let mut state = State{inputs: vec![input], outputs: Vec::new(), pc: 0, program: HashMap::with_capacity(program.len())};

        for (i, op) in program.iter().enumerate() {
            state.program.insert(i as i64, *op);
        }

        state
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Next {
    Continue,
    Exit(i64),
}

const POSITION: i64 = 0;
const IMMEDIATE: i64 = 1;

fn exec_op(state: &mut State) -> Next {
    let op = state.program.get(&state.pc).unwrap();
    match op % 100 {
        99 => {
            return Next::Exit(state.outputs.pop().unwrap_or(0))
        },
        1 => {
            let first_param = *state.program.get(&(state.pc + 1)).unwrap();
            let first_value = if (op / 100) % 10 == POSITION {
                *state.program.get(&first_param).unwrap_or(&0)
            } else {
                first_param
            };
            let second_param = *state.program.get(&(state.pc + 2)).unwrap();
            let second_value = if (op / 1000) % 10 == POSITION {
                *state.program.get(&second_param).unwrap_or(&0)
            } else {
                second_param
            };

            let addr = *state.program.get(&(state.pc + 3)).unwrap();
            let sum = first_value + second_value;

            state.program.insert(addr, sum);

            state.pc += 4;

            Next::Continue
        },
        2 => {
            let first_param = *state.program.get(&(state.pc + 1)).unwrap();
            let first_value = if (op / 100) % 10 == 0 {
                *state.program.get(&first_param).unwrap_or(&0)
            } else {
                first_param
            };
            let second_param = *state.program.get(&(state.pc + 2)).unwrap();
            let second_value = if (op / 1000) % 10 == 0 {
                *state.program.get(&second_param).unwrap_or(&0)
            } else {
                second_param
            };

            let addr = *state.program.get(&(state.pc + 3)).unwrap();
            let product = first_value * second_value;
            state.program.insert(addr, product);
            state.pc += 4;

            Next::Continue
        },
        3 => {
            let addr = *state.program.get(&(state.pc + 1)).unwrap();
            state.program.insert(addr, state.inputs.pop().unwrap());
            state.pc += 2;
            Next::Continue
        },
        4 => {
            let param = *state.program.get(&(state.pc + 1)).unwrap_or(&0);
            let value = if (op / 100) % 10 == POSITION {
                *state.program.get(&param).unwrap_or(&0)
            } else {
                param
            };
            state.outputs.push(value);
            state.pc += 2;
            Next::Continue
        },
        5 => {
            let param = *state.program.get(&(state.pc + 1)).unwrap_or(&0);
            let value = if (op / 100) % 10 == POSITION {
                *state.program.get(&param).unwrap_or(&0)
            } else {
                param
            };

            if value != 0 {
                let param = *state.program.get(&(state.pc + 2)).unwrap_or(&0);
                let value = if (op / 1000) % 10 == POSITION {
                    *state.program.get(&param).unwrap_or(&0)
                } else {
                    param
                };
                state.pc = value;
            } else {
                state.pc += 3;
            }
            Next::Continue
        },
        6 => {
            let param = *state.program.get(&(state.pc + 1)).unwrap_or(&0);
            let value = if (op / 100) % 10 == POSITION {
                *state.program.get(&param).unwrap_or(&0)
            } else {
                param
            };

            if value == 0 {
                let param = *state.program.get(&(state.pc + 2)).unwrap_or(&0);
                let value = if (op / 1000) % 10 == POSITION {
                    *state.program.get(&param).unwrap_or(&0)
                } else {
                    param
                };
                state.pc = value;
            } else {
                state.pc += 3;
            }
            Next::Continue
        },
        7 => {
            let first_param = *state.program.get(&(state.pc + 1)).unwrap();
            let first_value = if (op / 100) % 10 == 0 {
                *state.program.get(&first_param).unwrap_or(&0)
            } else {
                first_param
            };
            let second_param = *state.program.get(&(state.pc + 2)).unwrap();
            let second_value = if (op / 1000) % 10 == 0 {
                *state.program.get(&second_param).unwrap_or(&0)
            } else {
                second_param
            };

            let addr = *state.program.get(&(state.pc + 3)).unwrap();
            if first_value < second_value {
                state.program.insert(addr, 1);
            } else {
                state.program.insert(addr, 0);
            }
            state.pc += 4;
            Next::Continue
        },
        8 => {
            let first_param = *state.program.get(&(state.pc + 1)).unwrap();
            let first_value = if (op / 100) % 10 == 0 {
                *state.program.get(&first_param).unwrap_or(&0)
            } else {
                first_param
            };
            let second_param = *state.program.get(&(state.pc + 2)).unwrap();
            let second_value = if (op / 1000) % 10 == 0 {
                *state.program.get(&second_param).unwrap_or(&0)
            } else {
                second_param
            };

            let addr = *state.program.get(&(state.pc + 3)).unwrap();
            if first_value == second_value {
                state.program.insert(addr, 1);
            } else {
                state.program.insert(addr, 0);
            }
            state.pc += 4;
            Next::Continue
        },
        _ => unreachable!(),
    }
}

fn run(program: &[i64], input: i64) -> i64 {
    let mut state = State::from(program, input);
    loop {
        if state.outputs.iter().any(|x| *x != 0) {
            println!("Outputs: {:?}", state.outputs);
        }
        if let Next::Exit(value) = exec_op(&mut state) {
            return value;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn example_part1() {
        let mut state = State::from(&vec![1002,4,3,4,33], 1);
        assert_eq!(Next::Continue, exec_op(&mut state));
        assert_eq!(Some(&99), state.program.get(&4));

        let mut state = State::from(&vec![1101,100,-1,4,0], 1);
        assert_eq!(Next::Continue, exec_op(&mut state));
        assert_eq!(Some(&99), state.program.get(&4));
    }

    #[test]
    fn example_part2() {
        assert_eq!(1, run(&vec![3,9,8,9,10,9,4,9,99,-1,8], 8));
        assert_eq!(0, run(&vec![3,9,8,9,10,9,4,9,99,-1,8], 9));
        assert_eq!(1, run(&vec![3,9,7,9,10,9,4,9,99,-1,8], 7));
        assert_eq!(0, run(&vec![3,9,7,9,10,9,4,9,99,-1,8], 8));
        assert_eq!(1, run(&vec![3,3,1108,-1,8,3,4,3,99], 8));
        assert_eq!(0, run(&vec![3,3,1108,-1,8,3,4,3,99], 9));
        assert_eq!(1, run(&vec![3,3,1107,-1,8,3,4,3,99], 7));
        assert_eq!(0, run(&vec![3,3,1107,-1,8,3,4,3,99], 9));

    
        assert_eq!(1, run(&vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9], 123));
        assert_eq!(0, run(&vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9], 0));
        assert_eq!(1, run(&vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1], 123));
        assert_eq!(0, run(&vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1], 0));


        assert_eq!(999, run(&vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99], 0));
        assert_eq!(1000, run(&vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99],8));
        assert_eq!(1001, run(&vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99],9));
    }
}


