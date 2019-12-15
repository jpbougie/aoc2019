use std::io::{self, Read};
use std::thread;
use std::sync::mpsc::{sync_channel, TryRecvError};
use std::collections::HashMap;

use ::aoc2019::intcode;

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut program = input.split(",").map(|x| x.trim().parse().unwrap()).collect::<Vec<i64>>();

    let (mut world, path) = run_droid(&program);
    println!("Part 01: {:?}", path.unwrap().len() - 1);
    println!("Part 02: {}", world.fill_with_oxgygen());
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
    fn command(&self) -> i64 {
        match *self {
            Dir::North => 1,
            Dir::South => 2,
            Dir::West => 3,
            Dir::East => 4,
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

impl From<i64> for Dir {
    fn from(i: i64) -> Self {
        match i {
            1 => Dir::North,
            2 => Dir::South,
            3 => Dir::West,
            4 => Dir::East,
            _ => unreachable!()
        }

    }
}



#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Status {
    BlockedByWall,
    Moved,
    MovedOnOxy,
}


#[derive(Debug, Eq, PartialEq)]
enum Tile {
    Wall,
    Oxygen,
    Empty,
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

    fn distance(&self, other: &Addr) -> u64 {
        (absdiff(self.x, other.x) + absdiff(self.y, other.y)) as u64
    }
}

#[derive(Debug)]
struct World {
    tiles: HashMap<Addr, Tile>,
    movements: Vec<Dir>,
    droid: Addr,
    oxygen: Option<Addr>,
    backtracking: bool,
}

use pathfinding::prelude::{astar,absdiff};

impl World {
    fn new() -> Self {
        let mut w = World{tiles: HashMap::new(), droid: Addr{x: 0, y: 0}, movements: Vec::new(), backtracking: false, oxygen: None};
        w.tiles.insert(Addr{x: 0, y: 0}, Tile::Empty);
        w
    }
    fn update(&mut self, command_sent: Dir, status: Status) {
        if status == Status::BlockedByWall {
            let mut wall_position = self.droid.clone();
            command_sent.advance(&mut wall_position);
            self.tiles.insert(wall_position, Tile::Wall);
        } else {
            command_sent.advance(&mut self.droid);
            if !self.backtracking {
                self.movements.push(command_sent);
            } else {
                self.backtracking = false;
            }
            if status == Status::MovedOnOxy {
                self.oxygen = Some(self.droid.clone());
            }
            self.tiles.insert(self.droid.clone(), if status == Status::Moved { Tile::Empty } else { Tile::Oxygen });
        }
    }

    fn backtrack(&mut self) -> Option<Dir> {
        self.backtracking = true;
        self.movements.pop().map(|x| x.inverse())
    }

    fn hidden_neighbours(&self) -> Vec<(Dir, Addr)> {
        self.droid.neighbours().iter().filter(|(_, n)| self.tiles.get(&n).is_none()).cloned().collect()
    }

    fn path_between(&self, origin: &Addr, dest: &Addr) -> Option<Vec<Addr>> {
        astar(origin, |p| p.neighbours().iter().filter_map(|n| self.tiles.get(&n.1).and_then(|x| if *x != Tile::Wall { Some((n.1.clone(), 1)) } else { None })).collect::<Vec<(Addr, u64)>>(),
              |p| p.distance(dest), |p| p == dest).map(|x| x.0)
    }

    fn fill_with_oxgygen(&mut self) -> usize {
        let mut steps = 0;

        loop {
            // get all the oxygen tiles
            let oxygen_cells = self.tiles.iter().filter_map(|(pos, tile)| if *tile == Tile::Oxygen { Some(pos.clone()) } else { None }).collect::<HashSet<_>>();
            // get all their empty neighbours
            let propagating_cells = oxygen_cells.iter().flat_map(|pos| pos.neighbours().into_iter().filter_map(|(_, n)| self.tiles.get(&n).filter(|t| **t == Tile::Empty).map(|_| n.clone()))).collect::<HashSet<_>>();

            if propagating_cells.len() == 0 {
                break
            }

            propagating_cells.into_iter().for_each(|pos| { self.tiles.insert(pos, Tile::Oxygen);} );
            steps += 1
        }

        steps
    }
}
use std::collections::HashSet;

use std::fmt::{self, Display, Formatter};
impl Display for World {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut tiles = self.tiles.keys().collect::<Vec<_>>();
        tiles.push(&self.droid);
        let min_x = tiles.iter().map(|addr| addr.x).min().unwrap();
        let min_y = tiles.iter().map(|addr| addr.y).min().unwrap();
        let max_x = tiles.iter().map(|addr| addr.x).max().unwrap();
        let max_y = tiles.iter().map(|addr| addr.y).max().unwrap();

        let width = max_x - min_x + 1;
        let height = max_y - min_y + 1;

        for y in 0..height {
            for x in 0..width {
                let adjusted_x = x as i64 + min_x;
                let adjusted_y = y as i64 + min_y;
                if self.droid.x == adjusted_x && self.droid.y == adjusted_y {
                    write!(f,"D")?;
                } else {
                    match self.tiles.get(&Addr{x: adjusted_x, y: adjusted_y}) {
                        None => write!(f, " "),
                        Some(Tile::Oxygen) => write!(f, "O"),
                        Some(Tile::Empty) => write!(f, "."),
                        Some(Tile::Wall) => write!(f, "#"),
                    }?;

                }
            }
            write!(f,"\n")?;
        }

        Ok(())
    }

}

impl From<i64> for Status {
    fn from(i: i64) -> Self {
        match i {
            0 => Status::BlockedByWall,
            1 => Status::Moved,
            2 => Status::MovedOnOxy,
            _ => unreachable!()
        }
    }
}

use termion::input::TermRead;
fn run_droid(program: &[i64]) -> (World, Option<Vec<Addr>>) {
    let (inputs_sender, inputs_receiver) = sync_channel(0);
    let (outputs_sender, outputs_receiver) = sync_channel(0);
    let mut state = intcode::State::from(0, program, inputs_receiver, outputs_sender);
    let mut world = World::new();

    thread::spawn(move || state.run());

    let mut step = 0;
    loop {
        let mut potential = world.hidden_neighbours();
        let next_command = match potential.pop() {
            Some((command, _)) => {
                command
            },
            None => {
                // Backtrack
                match world.backtrack() {
                    Some(x) => x,
                    None => break
                }
            }
        };
        inputs_sender.send(next_command.command());

        match outputs_receiver.recv() {
            Ok(status_code) => {
                world.update(next_command, status_code.into());
            },
            Err(_) => break
        }
    }

    world.droid = Addr{x: 0, y: 0};
    println!("{}", world);
    println!("Oxygen is at {:?}", world.oxygen);
    let oxygen = world.oxygen.as_ref().unwrap().clone();
    let path = world.path_between(&Addr{x: 0, y:0}, &oxygen);
    (world, path)
}
