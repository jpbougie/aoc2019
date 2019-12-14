use std::io::{self, Read};
use std::thread;
use std::sync::mpsc::{sync_channel, TryRecvError};

use ::aoc2019::intcode;

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut program = input.split(",").map(|x| x.trim().parse().unwrap()).collect::<Vec<i64>>();
    println!("Part 01: {}", run(&program).0);

    program[0] = 2;

    println!("{:?}", run_with_joystick(&program));
    Ok(())
}

fn run_with_joystick(program: &[i64]) -> i64 {
    let mut screen = HashMap::new();
    let mut score = 0;

    let (inputs_sender, inputs_receiver) = sync_channel(0);
    let (outputs_sender, outputs_receiver) = sync_channel(0);
    let mut state = intcode::State::from(0, program, inputs_receiver, outputs_sender);

    thread::spawn(move || {
        while let intcode::Next::Continue = intcode::exec_op(&mut state) {}
    });


    loop {
        let next_input = match screen.iter().find(|(_, tile)| tile == &&Tile::Ball) {
            Some(((ball_x, _), _)) => {
                match screen.iter().find(|(_, tile)| tile == &&Tile::Paddle) {
                    Some(((paddle_x, _), _)) => {
                        if ball_x > paddle_x {
                            1
                        } else if ball_x <  paddle_x {
                            -1
                        } else {
                            0
                        }
                    },
                    None => 0
                }

            }
            None => 0,
        };

        inputs_sender.try_send(next_input);

        match outputs_receiver.try_recv() {
            Ok(x) => {
                let y = outputs_receiver.recv().unwrap();
                let tile_type = outputs_receiver.recv().unwrap();

                if x == -1 && y == 0 {
                    score = tile_type;
                } else {
                    if tile_type == 0 {
                        screen.remove(&(x, y));
                    } else {
                        screen.insert((x, y), match tile_type {
                            1 => Tile::Wall,
                            2 => Tile::Block,
                            3 => Tile::Paddle,
                            4 => Tile::Ball,
                            _ => unreachable!()
                        });
                    }
                }
            }
            Err(TryRecvError::Disconnected) => {
                break;
            },
            _ => {}
        }
    }

    score
}

//use std::collections::BinaryHeap;
//fn high_score(program: &[i64]) -> i64 {
    //let mut attempts = BinaryHeap::new();

    //attempts.push(Attempt{init: Vec::new(), next_input: 0, score: 0, blocks_left: 0, inputs_entered: 0, previous_attempt: 0, high_score: 0});

    //let mut attempts_seen = 0;
    //let mut max_high_score = 0;

    //while let Some(mut attempt) = attempts.pop() {
        //attempt.run(program);
        //attempts_seen += 1;

        //if attempt.high_score > max_high_score {
            //max_high_score = attempt.high_score;
            //println!("Attempt #{} reached: {}", attempts_seen, max_high_score);
        //}

        //if attempt.blocks_left == 0 {
            //return attempt.score
        //}

        //if attempt.inputs_entered <= attempt.init.len() {
            //continue
        //}

        //for next_attempt in attempt.new_attempts() {
            //attempts.push(next_attempt);
        //}
    //}

    //0
//}

//#[derive(Debug, Eq, PartialEq, Clone)]
//struct Attempt {
    //init: Vec<i64>,
    //next_input: i64,
    //score: i64,
    //high_score: i64,
    //blocks_left: usize,
    //inputs_entered: usize,
    //previous_attempt: usize,
//}

//impl Attempt {
    //fn new_attempts(&self) -> Vec<Attempt> {
        //let mut results = Vec::with_capacity(3);
        //let mut new = self.clone();
        //new.previous_attempt = self.inputs_entered;
        ////for i in 0..(self.inputs_entered - self.init.len()) {
        ////}
        //let mut left = new.clone();
        //left.next_input = 0;
        //left.init.push(-1);
        //results.push(left);

        //let mut stay = new.clone();
        //stay.init.push(0);
        //stay.next_input = 0;
        //results.push(stay);

        //let mut right = new.clone();
        //right.next_input = 0;
        //right.init.push(1);
        //results.push(right);

        //results
    //}

    //fn run(&mut self, program: &[i64]) {
        //let mut screen = HashMap::new();
        //self.score = 0;
        //self.high_score = 0;
        //self.inputs_entered = 0;

        //let (inputs_sender, inputs_receiver) = sync_channel(0);
        //let (outputs_sender, outputs_receiver) = sync_channel(0);
        //let mut state = intcode::State::from(0, program, inputs_receiver, outputs_sender);

        //thread::spawn(move || {
            //while let intcode::Next::Continue = intcode::exec_op(&mut state) {}
        //});


        //loop {
            //let next_input = if self.inputs_entered >= self.init.len() {
                //self.next_input
            //} else {
                //self.init[self.inputs_entered]
            //};

            //if inputs_sender.try_send(next_input).is_ok() {
                //self.inputs_entered += 1;
            //}

            //match outputs_receiver.try_recv() {
                //Ok(x) => {
                    //let y = outputs_receiver.recv().unwrap();
                    //let tile_type = outputs_receiver.recv().unwrap();

                    //if x == -1 && y == 0 {
                        //self.score = tile_type;
                        //if self.score > self.high_score {
                            //self.high_score = self.score;
                        //}
                        //continue
                    //}

                    //if tile_type == 0 {
                        //screen.remove(&(x, y));
                    //} else {
                        //screen.insert((x, y), match tile_type {
                            //1 => Tile::Wall,
                            //2 => Tile::Block,
                            //3 => Tile::Paddle,
                            //4 => Tile::Ball,
                            //_ => unreachable!()
                        //});
                    //}
                //}
                //Err(TryRecvError::Disconnected) => {
                    //break;
                //},
                //_ => {}
            //}
        //}

        //self.blocks_left = screen.iter().filter(|(_, tile)| tile == &&Tile::Block).count();
    //}
//}

//use std::cmp::{Ord, Ordering};
//impl Ord for Attempt {
    //fn cmp(&self, other: &Attempt) -> Ordering {
        //self.high_score.cmp(&other.high_score).then_with(|| other.blocks_left.cmp(&self.blocks_left)).then_with(|| self.previous_attempt.cmp(&other.previous_attempt)).then_with(|| other.init.cmp(&self.init))
    //}
//}

//impl PartialOrd for Attempt {
    //fn partial_cmp(&self, other: &Attempt) -> Option<Ordering> {
        //Some(self.cmp(&other))
    //}

//}

use std::collections::HashMap;
fn run(program: &[i64]) -> (usize, i64) {
    let mut screen = HashMap::new();
    let mut score = 0;

    let (_inputs_sender, inputs_receiver) = sync_channel(0);
    let (outputs_sender, outputs_receiver) = sync_channel(0);
    let mut state = intcode::State::from(0, program, inputs_receiver, outputs_sender);

    thread::spawn(move || {
        while let intcode::Next::Continue = intcode::exec_op(&mut state) {}
    });

    while let Ok(x) = outputs_receiver.recv() {
        let y = outputs_receiver.recv().unwrap();
        let tile_type = outputs_receiver.recv().unwrap();

        if x == -1 && y == 0 {
            score = tile_type;
            continue
        }

        if tile_type == 0 {
            screen.remove(&(x, y));
        } else {
            screen.insert((x, y), match tile_type {
                1 => Tile::Wall,
                2 => Tile::Block,
                3 => Tile::Paddle,
                4 => Tile::Ball,
                _ => unreachable!()
            });
        }
    }

    (screen.iter().filter(|(_, tile)| tile == &&Tile::Block).count(), score)
}

#[derive(Debug, Eq, PartialEq)]
enum Tile {
    Wall,
    Block,
    Paddle,
    Ball
}
