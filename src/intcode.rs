use std::collections::{HashMap, HashSet};

use std::io::{self, Read};
use std::sync::mpsc::{sync_channel, SyncSender, Receiver, RecvError, TryRecvError};
use std::thread;

#[derive(Debug)]
pub struct State {
    pub id: usize,
    pub inputs: Receiver<i64>,
    pub outputs: SyncSender<i64>,
    pub program: HashMap<i64, i64>,
    pub pc: i64,
    pub relative_base: i64,
}

impl State {
    pub fn from(id: usize, program: &[i64], inputs: Receiver<i64>, outputs: SyncSender<i64>) -> Self {
        let mut state = State{id, inputs , outputs, pc: 0, program: HashMap::with_capacity(program.len()), relative_base: 0};

        for (i, op) in program.iter().enumerate() {
            state.program.insert(i as i64, *op);
        }

        state
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Next {
    Continue,
    Exit(Option<i64>),
}

const POSITION: i64 = 0;
const IMMEDIATE: i64 = 1;
const RELATIVE: i64 = 2;

fn addr_from_param(state: &State, op:i64, param_position: i64) -> i64 {
    let addr = state.pc + param_position;
    let param = *state.program.get(&addr).expect(&format!("Expected value at position: {}, got none", addr));
    let diviser = match param_position {
        1 => 100,
        2 => 1000,
        3 => 10000,
        _ => unreachable!()
    };
    let mode = (op / diviser) % 10;

    match mode {
        POSITION => {
            param
        },
        RELATIVE => {
            param + state.relative_base
        },
        _ => unreachable!()
    }
}

fn value_from_param(state: &State, op:i64, param_position: i64) -> i64 {
    let addr = state.pc + param_position;
    let param = *state.program.get(&addr).expect(&format!("Expected value at position: {}, got none", addr));
    let diviser = match param_position {
        1 => 100,
        2 => 1000,
        3 => 10000,
        _ => unreachable!()
    };
    let mode = (op / diviser) % 10;

    match mode {
        IMMEDIATE => param,
        POSITION => {
            //*state.program.get(&param).expect(&format!("Expected to get value from {} at {}", param, state.pc))
            *state.program.get(&param).unwrap_or(&0)
        },
        RELATIVE => {
            //*state.program.get(&(param + state.relative_base)).expect(&format!("Expected to get value from {} ({} + {}), got none", param + state.relative_base, param, state.relative_base))
            *state.program.get(&(param + state.relative_base)).unwrap_or(&0)
        },
        _ => unreachable!()
    }
}

pub fn exec_op(state: &mut State) -> Next {
    let op = state.program.get(&state.pc).unwrap();
    match op % 100 {
        99 => {
            return Next::Exit(None)
        },
        1 => {
            let first_value = value_from_param(&state, *op, 1);
            let second_value = value_from_param(&state, *op, 2);
            let addr = addr_from_param(&state, *op, 3);

            let sum = first_value + second_value;
            state.program.insert(addr, sum);

            state.pc += 4;

            Next::Continue
        },
        2 => {
            let first_value = value_from_param(&state, *op, 1);
            let second_value = value_from_param(&state, *op, 2);
            let addr = addr_from_param(&state, *op, 3);

            let product = first_value * second_value;
            state.program.insert(addr, product);
            state.pc += 4;

            Next::Continue
        },
        3 => {
            let addr = addr_from_param(&state, *op, 1);
            let value = state.inputs.recv().unwrap();
            //println!("[{}] Got a {} from the input channel", state.id, value);
            state.program.insert(addr, value);
            state.pc += 2;
            Next::Continue
        },
        4 => {
            let value = value_from_param(&state, *op, 1);

            if let Err(_) = state.outputs.send(value) {
                // receiver has dropped, it's time to bail out
                return Next::Exit(Some(value))
            }
            state.pc += 2;
            Next::Continue
        },
        5 => {
            let value = value_from_param(&state, *op, 1);

            if value != 0 {
                let value = value_from_param(&state, *op, 2);
                state.pc = value;
            } else {
                state.pc += 3;
            }
            Next::Continue
        },
        6 => {
            let value = value_from_param(&state, *op, 1);

            if value == 0 {
                let value = value_from_param(&state, *op, 2);
                state.pc = value;
            } else {
                state.pc += 3;
            }
            Next::Continue
        },
        7 => {
            let first_value = value_from_param(&state, *op, 1);
            let second_value = value_from_param(&state, *op, 2);
            let addr = addr_from_param(&state, *op, 3);

            if first_value < second_value {
                state.program.insert(addr, 1);
            } else {
                state.program.insert(addr, 0);
            }
            state.pc += 4;
            Next::Continue
        },
        8 => {
            let first_value = value_from_param(&state, *op, 1);
            let second_value = value_from_param(&state, *op, 2);
            let addr = addr_from_param(&state, *op, 3);

            if first_value == second_value {
                state.program.insert(addr, 1);
            } else {
                state.program.insert(addr, 0);
            }
            state.pc += 4;
            Next::Continue
        },
        9 => {
            let value = value_from_param(&state, *op, 1);
            state.relative_base += value;
            state.pc += 2;
            Next::Continue
        }
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn example_part1() {
        let program: &[i64] = &[104,1125899906842624,99][..];
        assert_eq!(run(program, 1), 1125899906842624);
    }

    #[test]
    fn example_part2() {
    }
}

