use num;
use std::io::{self, BufRead};
fn main() -> io::Result<()> {
    let mut planets = io::stdin().lock().lines().map(|line| {
        let line = line?;
        Ok(Planet{pos: line.parse()?, vel: V3(0, 0, 0)})
    }).collect::<Result<Vec<Planet>, io::Error>>()?;

    // Part 01
    run(&mut planets, 1000);
    println!("Part 01: {}", planets.iter().map(|planet| planet.energy()).sum::<i64>());

    // Part 02
    let lengths = cycle_length(&mut planets);
    let cycle = num::integer::lcm((lengths.0).1, num::integer::lcm((lengths.1).1, (lengths.2).1));
    println!("Part 02: {}", cycle);

    Ok(())
}


fn run(planets: &mut Vec<Planet>, steps: usize) {
    for _i in 0..steps {
        step(planets);
    }
}

use std::collections::HashMap;
fn cycle_length(planets: &[Planet]) -> ((usize, usize), (usize, usize), (usize, usize)) {
    let mut result = ((0, 0), (0, 0), (0, 0));
    // X
    let mut pls = planets.iter().cloned().collect::<Vec<Planet>>();
    let mut seen = HashMap::new();
    let mut i = 0;
    loop {
        let alignment = pls.iter().map(|planet| (planet.pos.0, planet.vel.0)).collect::<Vec<(i64, i64)>>();
        if let Some(base) = seen.get(&alignment) {
            (result.0).0 = *base;
            (result.0).1 = i - *base;
            break
        }

        seen.insert(alignment, i);
        i += 1;

        step(&mut pls);
    }

    // Y
    let mut pls = planets.iter().cloned().collect::<Vec<Planet>>();
    let mut seen = HashMap::new();
    let mut i = 0;
    loop {
        let alignment = pls.iter().map(|planet| (planet.pos.1, planet.vel.1)).collect::<Vec<(i64, i64)>>();
        if let Some(base) = seen.get(&alignment) {
            (result.1).0 = *base;
            (result.1).1 = i - *base;
            break
        }

        seen.insert(alignment, i);
        i += 1;

        step(&mut pls);
    }

    // Y
    let mut pls = planets.iter().cloned().collect::<Vec<Planet>>();
    let mut seen = HashMap::new();
    let mut i = 0;
    loop {
        let alignment = pls.iter().map(|planet| (planet.pos.2, planet.vel.2)).collect::<Vec<(i64, i64)>>();
        if let Some(base) = seen.get(&alignment) {
            (result.2).0 = *base;
            (result.2).1 = i - *base;
            break
        }

        seen.insert(alignment, i);
        i += 1;

        step(&mut pls);
    }

    result
}


fn step(planets: &mut Vec<Planet>) {
    apply_gravity(planets);
    apply_velocity(planets);
}

fn apply_gravity(planets: &mut Vec<Planet>) {
    let l = planets.len();
    for i in 0..l {
        for j in (i+1)..l {
            // X
            if planets[i].pos.0 < planets[j].pos.0 {
                planets[i].vel.0 += 1;
                planets[j].vel.0 -= 1;
            } else if planets[i].pos.0 > planets[j].pos.0 {
                planets[i].vel.0 -= 1;
                planets[j].vel.0 += 1;
            }
            // Y
            if planets[i].pos.1 < planets[j].pos.1 {
                planets[i].vel.1 += 1;
                planets[j].vel.1 -= 1;
            } else if planets[i].pos.1 > planets[j].pos.1 {
                planets[i].vel.1 -= 1;
                planets[j].vel.1 += 1;
            }
            // Z
            if planets[i].pos.2 < planets[j].pos.2 {
                planets[i].vel.2 += 1;
                planets[j].vel.2 -= 1;
            } else if planets[i].pos.2 > planets[j].pos.2 {
                planets[i].vel.2 -= 1;
                planets[j].vel.2 += 1;
            }
        }
    }
}

fn apply_velocity(planets: &mut Vec<Planet>) {
    for planet in planets {
        planet.pos.0 += planet.vel.0;
        planet.pos.1 += planet.vel.1;
        planet.pos.2 += planet.vel.2;
    }
}

use std::str::FromStr;

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
struct V3(i64, i64, i64);

use regex::Regex;

impl FromStr for V3 {
    type Err = io::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r#"<x=(?P<x>-?\d+), y=(?P<y>-?\d+), z=(?P<z>-?\d+)>"#).unwrap();
        let caps = re.captures(input).unwrap();
        Ok(V3(caps["x"].parse().unwrap(), caps["y"].parse().unwrap(), caps["z"].parse().unwrap()))
    }
}

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
struct Planet{
    pos: V3,
    vel: V3,
}

impl Planet {
    fn energy(&self) -> i64 {
        (self.pos.0.abs() + self.pos.1.abs() + self.pos.2.abs()) *
        (self.vel.0.abs() + self.vel.1.abs() + self.vel.2.abs())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_vector() {
        let v3: V3 = "<x=1, y=2, z=3>".parse().unwrap();
        assert_eq!(V3(1, 2, 3), v3);

        let v3: V3 = "<x=-1, y=-2, z=33>".parse().unwrap();
        assert_eq!(V3(-1, -2, 33), v3);
    }
}
