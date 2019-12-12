use std::collections::{HashMap, HashSet};

use std::io::{self, Read};
use std::sync::mpsc::{sync_channel, SyncSender, Receiver, RecvError, TryRecvError};
use std::thread;

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let program = input.split(",").map(|x| x.trim().parse().unwrap()).collect::<Vec<i64>>();

    println!("Part 01: {}", run(&program, 0).1);
    println!("Part 02:");
    let (painted, _) = run(&program, 1);
    draw(&painted);

    Ok(())
}


#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Dir {
    North,
    South,
    West,
    East,
}

impl Dir {
    fn rotate(&self, i: i64) -> Dir {
        match *self {
            Dir::North => if i == 0 { Dir::West } else { Dir::East },
            Dir::East => if i == 0 { Dir::North } else { Dir::South },
            Dir::South => if i == 0 { Dir::East } else { Dir::West },
            Dir::West => if i == 0 { Dir::South } else { Dir::North },
        }
    }

    fn advance(&self, pos: &mut (i64, i64)) {
        match *self {
            Dir::North => pos.1 += 1,
            Dir::East => pos.0 += 1,
            Dir::South => pos.1 -= 1,
            Dir::West => pos.0 -= 1,
        };
    }
}

fn draw(painted: &HashMap<(i64, i64), i64>) {
    let painted = painted.iter().filter(|(_pos, painted)| *painted == &1).map(|(pos, _painted)| *pos).collect::<Vec<_>>();

    // flip the x axis
    let painted = painted.iter().map(|(x, y)| (x, y * -1)).collect::<Vec<_>>();

    let min_x = painted.iter().map(|(x, _y)| *x).min().unwrap();
    let min_y = painted.iter().map(|(_x, y)| *y).min().unwrap();
    let max_x = painted.iter().map(|(x, _y)| *x).max().unwrap();
    let max_y = painted.iter().map(|(_x, y)| *y).max().unwrap();

    let painted = painted.iter().map(|(x, y)| (*x - min_x,  *y - min_y)).collect::<HashSet<_>>();
    let width = max_x - min_x + 1;
    let height = max_y - min_y + 1;

    for y in 0..height {
        for x in 0..width {
            if painted.contains(&(x as i64, y as i64)) {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!("");
    }
}

fn run(program: &[i64], initial_color: i64) -> (HashMap<(i64, i64), i64>, usize) {
    let (inputs_sender, inputs_receiver) = sync_channel(0);
    let (outputs_sender, outputs_receiver) = sync_channel(0);
    let mut state = State::from(0, program, inputs_receiver, outputs_sender);
    thread::spawn(move || {
        while let Next::Continue = exec_op(&mut state) {}
    });

    let mut painted = HashMap::new();
    painted.insert((0, 0), initial_color);
    let mut pos: (i64, i64) = (0, 0);
    let mut dir = Dir::North;

    loop {
        if let Ok(()) = inputs_sender.send(*painted.get(&pos).unwrap_or(&0)) {
        }

        match outputs_receiver.recv() {
            Ok(color) => {
                painted.insert(pos.clone(), color);
                let rotation = outputs_receiver.recv().unwrap();
                dir = dir.rotate(rotation);
                dir.advance(&mut pos);
            },
            Err(RecvError) => break,
            _ => {}
        };

    }
    let l = painted.len();

    (painted, l)
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
    fn example_part1() {
        let program: &[i64] = &[104,1125899906842624,99][..];
        assert_eq!(run(program, 1), 1125899906842624);
    }

    #[test]
    fn example_part2() {
    }
}


