use std::collections::HashMap;

use std::io::{self, Read};

fn main() -> std::io::Result<()>{
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut program = input.split(",").map(|x| x.trim().parse().unwrap()).collect::<Vec<usize>>();

    program[1] = 12;
    program[2] = 2;

    println!("Part 01: {}", run(&program));

    let target = 19690720;
    'l: for noun in 0..=99 {
        for verb in 0..=99 {
            program[1] = noun;
            program[2] = verb;
            if run(&program) == target {
                println!("Part 02: {}", 100 * noun + verb);
                break 'l;
            }
        }
    }
    Ok(())
}

#[derive(Default, Debug)]
struct State {
    program: HashMap<usize, usize>,
    pc: usize,
}

impl State {
    pub fn from(program: &[usize]) -> Self {
        let mut state = State{pc: 0, program: HashMap::with_capacity(program.len())};

        for (i, op) in program.iter().enumerate() {
            state.program.insert(i, *op);
        }

        state
    }
}

enum Next {
    Continue,
    Exit(usize),
}

fn exec_op(state: &mut State) -> Next {
    let op = state.program.get(&state.pc).unwrap();
    match op {
        99 => {
            return Next::Exit(*state.program.get(&0).unwrap_or(&0))
        },
        1 => {
            let sum = 
                state.program.get(state.program.get(&(state.pc + 1)).unwrap_or(&0)).unwrap_or(&0) +
                state.program.get(&state.program.get(&(state.pc + 2)).unwrap_or(&0)).unwrap_or(&0);

            state.program.insert(*state.program.get(&(state.pc + 3)).unwrap_or(&0), sum);

            state.pc += 4;

            Next::Continue
        },
        2 => {
            let sum = 
                state.program.get(&state.program.get(&(state.pc + 1)).unwrap_or(&0)).unwrap_or(&0) *
                state.program.get(&state.program.get(&(state.pc + 2)).unwrap_or(&0)).unwrap_or(&0);

            state.program.insert(*state.program.get(&(state.pc + 3)).unwrap_or(&0), sum);

            state.pc += 4;

            Next::Continue
        },
        _ => unreachable!(),
    }
}

fn run(program: &[usize]) -> usize {
    let mut state = State::from(program);
    loop {
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
        assert_eq!(run(&[1,9,10,3,2,3,11,0,99,30,40,50]), 3500);
        assert_eq!(run(&[1,0,0,0,99]), 2);
        assert_eq!(run(&[1,1,1,4,99,5,6,0,99]), 30);
    }
}


