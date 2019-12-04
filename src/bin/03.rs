use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let l1 = lines.next().unwrap().unwrap();
    let l2 = lines.next().unwrap().unwrap();

    println!("Part 1: {}", run(&l1, &l2).unwrap());
    println!("Part 2: {}", run02(&l1, &l2).unwrap());

}

#[derive(Debug, PartialEq, Eq)]
enum Move {
    Right(i64),
    Left(i64),
    Up(i64),
    Down(i64),
}

type Point = (i64, i64);

fn mh_dist(pt: &Point) -> i64 {
    pt.0.abs() + pt.1.abs()
}


#[derive(Debug, Clone, Copy)]
struct PointWithStep {
    point: Point,
    steps: usize,
}

use std::collections::HashSet;

impl Move {
    fn append_points(&self, orig: Point, points: &mut HashSet<Point>) -> Point {
        let mut pos = orig;
        match *self {
            Move::Right(dist) => {
                for i in 1..=dist {
                    pos = (orig.0, orig.1 + i); 
                    points.insert(pos);
                }
            },
            Move::Left(dist) => {
                for i in 1..=dist {
                    pos = (orig.0, orig.1 - i); 
                    points.insert(pos);
                }

            },
            Move::Up(dist) => {
                for i in 1..=dist {
                    pos = (orig.0 + i, orig.1); 
                    points.insert(pos);
                }

            },
            Move::Down(dist) => {
                for i in 1..=dist {
                    pos = (orig.0 - i, orig.1); 
                    points.insert(pos);
                }

            }
        }

        pos
    }
    
    fn append_points_with_steps(&self, orig: PointWithStep, points: &mut Vec<PointWithStep>) -> PointWithStep {
        let mut pos = orig;
        match *self {
            Move::Right(dist) => {
                for i in 1..=dist {
                    pos = PointWithStep{point: (orig.point.0, orig.point.1 + i), steps: orig.steps + i as usize}; 
                    points.push(pos);
                }
            },
            Move::Left(dist) => {
                for i in 1..=dist {
                    pos = PointWithStep{point: (orig.point.0, orig.point.1 - i), steps: orig.steps + i as usize}; 
                    points.push(pos);
                }

            },
            Move::Up(dist) => {
                for i in 1..=dist {
                    pos = PointWithStep{point: (orig.point.0 + i, orig.point.1), steps: orig.steps + i as usize}; 
                    points.push(pos);
                }

            },
            Move::Down(dist) => {
                for i in 1..=dist {
                    pos = PointWithStep{point: (orig.point.0 - i, orig.point.1), steps: orig.steps + i as usize}; 
                    points.push(pos);
                }

            }
        }

        pos
    }
}

impl From<&str> for Move {
    fn from(i: &str) -> Self {
        let (op, dist) = i.split_at(1);
        let dist = dist.parse().expect("an int");
        match op {
            "R" => Move::Right(dist),
            "L" => Move::Left(dist),
            "U" => Move::Up(dist),
            "D" => Move::Down(dist),
            _ => unreachable!()
        }
    }
}

fn smallest_intersection(path1: &HashSet<Point>, path2: &HashSet<Point>) -> Option<i64> {
    path1.intersection(path2).filter(|pt| mh_dist(pt) != 0).min_by(|p1, p2| mh_dist(p1).cmp(&mh_dist(p2))).map(mh_dist)
}

fn run(line1: &str, line2: &str) -> Option<i64> {
    let l1 = line1.split(",").map(|x| x.into()).collect::<Vec<Move>>();
    let l2 = line2.split(",").map(|x| x.into()).collect::<Vec<Move>>();
    let mut points_l1 = HashSet::new();
    let mut pos = (0, 0);

    l1.iter().for_each(|mv| {
        pos = mv.append_points(pos, &mut points_l1);
    });

    let mut points_l2 = HashSet::new();
    let mut pos = (0, 0);
    l2.iter().for_each(|mv| {
        pos = mv.append_points(pos, &mut points_l2);
    });

    smallest_intersection(&points_l1, &points_l2)
}

fn run02(line1: &str, line2: &str) -> Option<usize> {
    let l1 = line1.split(",").map(|x| x.into()).collect::<Vec<Move>>();
    let l2 = line2.split(",").map(|x| x.into()).collect::<Vec<Move>>();
    let mut points_l1 = Vec::new();
    let mut pos = PointWithStep{point: (0, 0), steps: 0};

    l1.iter().for_each(|mv| {
        pos = mv.append_points_with_steps(pos, &mut points_l1);
    });

    let mut points_l2 = Vec::new();
    let mut pos = PointWithStep{point: (0, 0), steps: 0};
    l2.iter().for_each(|mv| {
        pos = mv.append_points_with_steps(pos, &mut points_l2);
    });

    points_l1.iter().filter_map(|point| {
        if point.point == (0, 0) {
            None
        } else {
            points_l2.iter().find(|other| point.point == other.point).map(|other| other.steps ).map(|os| point.steps + os)
        }
    }).min()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let mov: Move = "U12".into();
        assert_eq!(mov, Move::Up(12));
        let mov: Move = "D6".into();
        assert_eq!(mov, Move::Down(6));
    }

    #[test]
    fn basic() {
        let l1 = "R8,U5,L5,D3";
        let l2 = "U7,R6,D4,L4";

        assert_eq!(run(l1, l2), Some(6));
        assert_eq!(run02(l1, l2), Some(30));
    }

    #[test]
    fn moves() {
        let l1 = "R75,D30,R83,U83,L12,D49,R71,U7,L72";
        let l2 = "U62,R66,U55,R34,D71,R55,D58,R83";

        assert_eq!(run(l1, l2), Some(159));
        assert_eq!(run02(l1, l2), Some(610));

        let l1 = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51";
        let l2 = "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";

        assert_eq!(run(l1, l2), Some(135));
        assert_eq!(run02(l1, l2), Some(410));
    }
}
