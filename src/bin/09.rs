use std::collections::HashMap;

use std::io::{self, Read};
use std::sync::mpsc::{sync_channel, SyncSender, Receiver};
use std::thread;

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let program = input.split(",").map(|x| x.trim().parse().unwrap()).collect::<Vec<i64>>();

    println!("Part 01: {}", run(&program, 1));
    println!("Part 02: {}", run(&program, 2));

    Ok(())
}

fn run(program: &[i64], input: i64) -> i64 {
    let (inputs_sender, inputs_receiver) = sync_channel(1);
    let (outputs_sender, outputs_receiver) = sync_channel(0);
    let mut state = State::from(0, program, inputs_receiver, outputs_sender);
    thread::spawn(move || {
        while let Next::Continue = exec_op(&mut state) {}
    });

    inputs_sender.send(input).unwrap();

    let mut last_output = 0;
    while let Ok(output) = outputs_receiver.recv() {
        println!("DEBUG: Output {}", output);
        last_output = output;
    }

    last_output
}

#[derive(Debug)]
struct State {
    id: usize,
    inputs: Receiver<i64>,
    outputs: SyncSender<i64>,
    program: HashMap<i64, i64>,
    pc: i64,
    relative_base: i64,
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
enum Next {
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
            *state.program.get(&param).expect(&format!("Expected to get value from {} at {}", param, state.pc))
        },
        RELATIVE => {
            *state.program.get(&(param + state.relative_base)).expect(&format!("Expected to get value from {} ({} + {}), got none", param + state.relative_base, param, state.relative_base))
        },
        _ => unreachable!()
    }
}

fn exec_op(state: &mut State) -> Next {
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
    fn test_relative_base() {
        let program: &[i64] = &[104,1125899906842624,99][..];
        let (sender, receiver) = sync_channel(1);
        let mut state = State::from(0, &[9, 0, 109, 10, 209, -16][..], receiver, sender);

        assert_eq!(exec_op(&mut state), Next::Continue);
        assert_eq!(state.pc, 2);
        assert_eq!(state.relative_base, 9);

        assert_eq!(exec_op(&mut state), Next::Continue);
        assert_eq!(state.pc, 4);
        assert_eq!(state.relative_base, 19);

        assert_eq!(exec_op(&mut state), Next::Continue);
        assert_eq!(state.pc, 6);
        assert_eq!(state.relative_base, 29);
    }

    #[test]
    fn example_part1() {
        let program: &[i64] = &[104,1125899906842624,99][..];
        assert_eq!(run(program, 1), 1125899906842624);
    }

    #[test]
    fn example_part2() {
    }
}


