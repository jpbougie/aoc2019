use std::io::{self, Read};
use std::thread;
use std::sync::mpsc::{sync_channel, TryRecvError};

use ::aoc2019::intcode;

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut program = input.split(",").map(|x| x.trim().parse().unwrap()).collect::<Vec<i64>>();
    let out = run(&program);
    println!("{}", out);
    let calibration = calibrate(&out);
    println!("Part 01: {}", calibration);
    let cmds = commands(&out);
    let mut it = cmds.iter().peekable();
    while let Some(x) = it.next() {
        print!("{}", x);
        if it.peek().is_some() {
            print!(",");
        }
    }
    println!("");

    program[0] = 2;
    let out = "A,B,B,C,B,C,B,C,A,A\nL,6,R,8,L,4,R,8,L,12\nL,12,R,10,L,4\nL,12,L,6,L,4,L,4\nn\n";
    println!("Part 02: {:?}", run_with_program(&program, &out));
    Ok(())
}

fn calibrate(input: &str) -> usize {
    let lines = input.lines().map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let mut out = 0;

    for (y, l) in lines.iter().enumerate() {
        for (x, &c) in l.iter().enumerate() {
            let addr = Addr{x: x as i64, y: y as i64};
            if c == '#' && addr.neighbours().iter().all(|(_, n)| {
                n.y >= 0 && (n.y as usize) < lines.len() && n.x >= 0  && (n.x as usize) < lines[n.y as usize].len() && lines[n.y as usize][n.x as usize] == '#'
            }) {
                out += y * x;
            }
        }
    }

    out
}


#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Dir {
    North,
    South,
    West,
    East,
}

impl Dir {
    fn left(&self) -> Self {
        match *self {
            Dir::North => Dir::West,
            Dir::South => Dir::East,
            Dir::West => Dir::South,
            Dir::East => Dir::North,
        }
    }

    fn right(&self) -> Self {
        match *self {
            Dir::North => Dir::East,
            Dir::South => Dir::West,
            Dir::West => Dir::North,
            Dir::East => Dir::South,
        }
    }

    fn advance(&self, pos: &mut Addr) {
        match *self {
            Dir::North => pos.y -= 1,
            Dir::East => pos.x += 1,
            Dir::South => pos.y += 1,
            Dir::West => pos.x -= 1,
        };
    }

    fn inverse(&self) -> Self {
        match *self {
            Dir::North => Dir::South,
            Dir::South => Dir::North,
            Dir::West => Dir::East,
            Dir::East => Dir::West,
        }
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
struct Addr{
    x: i64,
    y: i64,
}


impl Addr {
    fn north(&self) -> Addr {
        Addr{x: self.x, y: self.y - 1}
    }

    fn south(&self) -> Addr {
        Addr{x: self.x, y: self.y + 1}
    }

    fn east(&self) -> Addr {
        Addr{x: self.x + 1, y: self.y}
    }

    fn west(&self) -> Addr {
        Addr{x: self.x - 1, y: self.y}
    }

    fn neighbours(&self) -> Vec<(Dir, Addr)> {
        vec![(Dir::North, self.north()), (Dir::West, self.west()), (Dir::East, self.east()), (Dir::South, self.south())]
    }
}

#[derive(Debug)]
enum Command {
    Left,
    Right,
    Advance(usize)
}

use std::fmt;
impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            Command::Left => "L".to_string(),
            Command::Right => "R".to_string(),
            Command::Advance(steps) => steps.to_string()
        })
    }
}

use std::collections::HashSet;
fn commands(input: &str) -> Vec<Command> {
    let mut out = Vec::new();
    let scaffolds = input.lines().enumerate().flat_map(|(y, line)| line.chars().enumerate().filter(|(_x, ch)| *ch == '^' || *ch == '#').map(|(x, _)| Addr{x: x as i64, y: y as i64}).collect::<HashSet<Addr>>()).collect::<HashSet<Addr>>();
    let starting_position = input.lines().enumerate().filter_map(|(y, line)| line.chars().enumerate().filter_map(|(x, ch)| {
        if ch == '^' {
            Some(Addr{x: x as i64, y: y as i64})
        } else {
            None
        }
    }).next()).next().unwrap();

    let mut dir = Dir::North;
    let mut pos = starting_position.clone();

    loop {
        // try to see if we would have a cell if we rotated left
        let mut cell = pos.clone();
        dir.left().advance(&mut cell);

        if scaffolds.contains(&cell) {
            out.push(Command::Left);
            dir = dir.left();
        } else {
            let mut cell = pos.clone();
            dir.right().advance(&mut cell);
            if scaffolds.contains(&cell) {
                dir = dir.right();
                out.push(Command::Right);
            } else {
                break;
            }
        }

        let mut steps = 0;
        loop {
            let mut cell = pos.clone();
            dir.advance(&mut cell);

            if !scaffolds.contains(&cell) {
                out.push(Command::Advance(steps));
                break
            }

            steps += 1;
            pos = cell;
        }
    }

    out
}

fn run_with_program(program: &[i64], code: &str) -> i64 {
    let (inputs_sender, inputs_receiver) = sync_channel(code.len());
    let (outputs_sender, outputs_receiver) = sync_channel(0);
    let mut state = intcode::State::from(0, program, inputs_receiver, outputs_sender);
    thread::spawn(move || state.run());
    let mut output = String::new();
    let mut it = code.chars();
    let mut n = it.next();

    for ch in code.chars() {
        inputs_sender.send(ch as i64);
    }

    let mut out = 0;
    while let Ok(o) = outputs_receiver.recv() {
        out = o;
    }

    out
}

fn run(program: &[i64]) -> String {
    let (inputs_sender, inputs_receiver) = sync_channel(0);
    let (outputs_sender, outputs_receiver) = sync_channel(0);
    let mut state = intcode::State::from(0, program, inputs_receiver, outputs_sender);
    thread::spawn(move || state.run());
    let mut output = String::new();
    loop {
        match outputs_receiver.recv() {
            Ok(character) => {
                output.push((character as u8) as char);
            },
            Err(_) => break
        }
    }

    output
}
