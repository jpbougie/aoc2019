use std::collections::HashMap;

use std::io::{self, Read};
use std::sync::mpsc::{sync_channel, SyncSender, Receiver};
use std::thread;

use permutohedron::heap_recursive;

fn main() -> std::io::Result<()>{
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;


    let program = input.split(",").map(|x| x.trim().parse().unwrap()).collect::<Vec<i64>>();

    println!("Part 01: {}", max(&program, 5));

    println!("Part 02: {}", max_loop(&program, 5..=9));

    Ok(())
}

fn max(program: &[i64], amplifiers: i64) -> i64 {
    let mut inputs = (0..amplifiers).collect::<Vec<i64>>();
    let mut variants = Vec::new();
    heap_recursive(&mut inputs, |permutation| variants.push(permutation.to_vec()));

    variants.iter().fold(i64::min_value(), |max, input| {
        let (sender, receiver) = chain(&program, &input);
        let output = run(sender, receiver);
        if output > max {
            output
        } else {
            max
        }
    })
}

use core::ops::RangeInclusive;
fn max_loop(program: &[i64], inputs: RangeInclusive<i64>) -> i64 {
    let mut inputs = inputs.collect::<Vec<i64>>();
    let mut variants = Vec::new();
    heap_recursive(&mut inputs, |permutation| variants.push(permutation.to_vec()));

    variants.iter().fold(i64::min_value(), |max, input| {
        let (sender, receiver) = chain_loop(&program, &input);
        let output = run(sender, receiver);
        if output > max {
            output
        } else {
            max
        }
    })
}

fn chain(program: &[i64], inputs: &[i64]) -> (SyncSender<i64>, Receiver<i64>) {
    let (mut sender, mut receiver) = sync_channel(2);
    let first_sender = sender.clone();

    for (index, input) in inputs.iter().enumerate() {
        let (new_sender, new_receiver) = sync_channel(2);
        let i = *input;
        sender.send(i).unwrap();
        let mut state = State::from(index, program, receiver, new_sender.clone());
        thread::spawn(move || {
            while let Next::Continue = exec_op(&mut state) {}
        });

        receiver = new_receiver;
        sender = new_sender;
    }

    (first_sender, receiver)
}

fn chain_loop(program: &[i64], inputs: &[i64]) -> (SyncSender<i64>, Receiver<i64>) {
    let (final_sender, final_receiver) = sync_channel(0);
    let (mut sender, mut receiver) = sync_channel(2);
    let first_sender = sender.clone();

    for (index, input) in inputs[0..inputs.len() - 1].iter().enumerate() {
        let i = *input;
        let (new_sender, new_receiver) = sync_channel(2);
        sender.send(i).unwrap();
        let mut state = State::from(index, program, receiver, new_sender.clone());
        thread::spawn(move || {
            while let Next::Continue = exec_op(&mut state) {}
        });

        receiver = new_receiver;
        sender = new_sender;

    }

    let i = inputs[inputs.len() - 1];
    sender.send(i).unwrap();
    let mut state = State::from(inputs.len() - 1, program, receiver, first_sender.clone());
    thread::spawn(move || {
        loop {
            if let Next::Exit(exit_code) = exec_op(&mut state) {
                if let Some(exit_code) = exit_code {
                    final_sender.send(exit_code);
                }
            }
        }
    });

    (first_sender, final_receiver)
}

fn run(sender: SyncSender<i64>, receiver: Receiver<i64>) -> i64 {
    sender.send(0).unwrap();
    receiver.recv().unwrap()
}

#[derive(Debug)]
struct State {
    id: usize,
    inputs: Receiver<i64>,
    outputs: SyncSender<i64>,
    program: HashMap<i64, i64>,
    pc: i64,
}

impl State {
    pub fn from(id: usize, program: &[i64], inputs: Receiver<i64>, outputs: SyncSender<i64>) -> Self {
        let mut state = State{id, inputs , outputs, pc: 0, program: HashMap::with_capacity(program.len())};

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

fn exec_op(state: &mut State) -> Next {
    let op = state.program.get(&state.pc).unwrap();
    match op % 100 {
        99 => {
            return Next::Exit(None)
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
            //println!("[{}] Waiting for a message on the input channel", state.id);
            let value = state.inputs.recv().unwrap();
            //println!("[{}] Got a {} from the input channel", state.id, value);
            state.program.insert(addr, value);
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
            //println!("[{}] About to send {}", state.id, value);
            if let Err(_) = state.outputs.send(value) {
                // receiver has dropped, it's time to bail out
                return Next::Exit(Some(value))
            }
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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn example_part1() {
        let program: &[i64] = &[3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0][..];
        let (sender, receiver) = chain(program, &[4,3,2,1,0][..]);
        assert_eq!(run(sender, receiver), 43210);
        assert_eq!(max(&program, 5), 43210);
    }

    #[test]
    fn example_part2() {
        let program: &[i64] = &[3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5][..];
        let (sender, receiver) = chain_loop(program, &[9,8,7,6,5][..]);
        assert_eq!(run(sender, receiver), 139629729);
        assert_eq!(max_loop(&program, 5..=9), 139629729);
    }
}


