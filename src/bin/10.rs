use std::io::{self, Read};


#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Cell {
    Empty,
    Meteor,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            Cell::Empty => ".",
            Cell::Meteor => "#",
        })
    }
}

struct World {
    cells: Vec<Cell>,
    w: usize,
    h: usize,
}

struct Positions<'a> {
    world: &'a World,
    x: usize,
    y: usize,
    filter: Option<Cell>,
}

impl<'a> Iterator for Positions<'a> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.x += 1;
            if self.x >= self.world.w {
                self.y += 1;
                self.x = 0;
            }

            if self.y >= self.world.h {
                return None
            }

            if let Some(filter) = self.filter {
                if self.world.cells[self.y * self.world.w + self.x] == filter {
                    return Some((self.x, self.y))
                }
            }
        }
    }
}

impl World {
    fn meteors(&self) -> Positions {
        Positions{world: self, x: 0, y: 0, filter: Some(Cell::Meteor)}
    }

    fn emptys(&self) -> Positions {
        Positions{world: self, x: 0, y: 0, filter: Some(Cell::Empty)}
    }
}

use std::fmt;

impl fmt::Display for World {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.h {
            for x in 0..self.w {
                write!(f, "{}", self.cells[y*self.w + x])?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}


use std::str::FromStr;
impl FromStr for World {
    type Err = io::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut w = 0;
        let mut h = 0;
        let mut cells = Vec::with_capacity(input.len());
        for line in input.lines() {
            let trimmed = line.trim();
            if trimmed.len() == 0 { continue }
            h += 1;
            w = trimmed.len();

            for c in trimmed.chars() {
                cells.push(match c {
                    '#' => Cell::Meteor,
                    '.' => Cell::Empty,
                    _ => unreachable!(),
                });
            }
        }
        Ok(World{cells, w, h})
    }
}

use std::collections::HashSet;

fn nil_angle(a: (i64, i64), b: (i64, i64)) -> bool {
    a.0 * b.1 == b.0 * a.1 && (a.0 * b.0 + a.1 * b.1 >= 0)
}

fn part1(world: &World) -> (usize, (usize, usize)) {
    world.meteors().fold((0, (0, 0)), |acc, potential_meteor| {
        let mut rays: HashSet<(i8, i8)> = HashSet::new();
        for meteor in world.meteors() {
            let (dist_x, dist_y) = ((potential_meteor.0 as i8) - (meteor.0 as i8), (potential_meteor.1 as i8) - (meteor.1 as i8));


            if rays.iter().all(|(x, y)| !nil_angle((dist_x as i64, dist_y as i64), (*x as i64, *y as i64))) {
                rays.insert((dist_x, dist_y));
            }
        }

        if rays.len() > acc.0 {
            (rays.len(), potential_meteor)
        } else {
            acc
        }
    })
}




use std::cmp::Ordering;
fn sort_meteors(center: (i64, i64), a: (i64, i64), b: (i64, i64)) -> Ordering {
    // a is right of the center point and b is left
    if a.0 - center.0 >= 0 && b.0 - center.0 < 0 {
        return Ordering::Less;
    }

    // a is right of the center point and b is left
    if a.0 - center.0 < 0 && b.0 - center.0 >= 0 {
        return Ordering::Greater;
    }

    // a is over the center point and b is lower
    if a.0 - center.0 < 0 && b.0 - center.0 >= 0 {
        return Ordering::Less;
    }

    // a is over the center point and b is lower
    if a.0 - center.0 >= 0 && b.0 - center.0 < 0 {
        return Ordering::Greater;
    }

    // They are both on the same axis as the center point
    if a.0 == center.0 && b.0 == center.0 {
        if a.1 - center.1 <= 0 || b.1 - center.1 < 0 {
            return b.1.cmp(&a.1);
        }
        return a.1.cmp(&b.1);
    }

    // compute the cross product of vectors (center -> a) x (center -> b)
    let det = (a.0 - center.0) * (b.1 - center.1) - (b.0 - center.0) * (a.1 - center.1);
    if det < 0 {
        return Ordering::Greater;
    }
    if det > 0 {
        return Ordering::Less;
    }

    // points a and b are on the same line from the center
    // check which point is closer to the center
    let d1 = (a.0 - center.0) * (a.0 - center.0) + (a.1 - center.1) * (a.1 - center.1);
    let d2 = (b.0 - center.0) * (b.0 - center.0) + (b.1 - center.1) * (b.1 - center.1);
    d1.cmp(&d2)
}


fn relative(a: (i64, i64), center: (usize, usize)) -> (i64, i64) {
    (a.0 - center.0 as i64, a.1 - center.1 as i64)
}

fn part2(world: &World, station: (usize, usize)) ->  usize {
    let mut meteors = world.meteors().filter(|m| station != *m).map(|(x, y)| (x as i64, y as i64)).collect::<Vec<(_, _)>>();

    meteors.sort_by(|a, b| sort_meteors((station.0 as i64, station.1 as i64), *a, *b));

    let mut destroyed = 0;
    let mut i = 0;
    let mut last_destroyed = (0, 0);
    let mut turned = false;

    while destroyed < 200  {
        last_destroyed = meteors.remove(i);
        destroyed += 1;
        turned = false;

        if meteors.len() == 0 {
            break
        }

        if i >= meteors.len() {
            turned = true;
            i = 0;
        }

        loop {
            if turned || !nil_angle(relative(last_destroyed, station), relative(meteors[i], station)) {
                break
            }

            i += 1;
            if i >= meteors.len() {
                turned = true;
                i = 0;
            }
        }
    }

    (last_destroyed.0 * 100 + last_destroyed.1) as usize
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let world: World = input.parse()?;

    let (visible, station) = part1(&world);

    println!("part 01: {:?}", visible);
    println!("part 02: {:?}", part2(&world, station));
    Ok(())

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = ".#..#\n.....\n#####\n....#\n...##";
        let world: World = input.parse().unwrap();

        assert_eq!(part1(&world), (8, (3, 4)));

        let input = r#"......#.#.
                       #..#.#....
                       ..#######.
                       .#.#.###..
                       .#..#.....
                       ..#....#.#
                       #..#....#.
                       .##.#..###
                       ##...#..#.
                       .#....####"#;
        let world: World = input.parse().unwrap();

        assert_eq!(part1(&world), (33, (5, 8)));

    }

    #[test]
    fn test_part2() {
        let input = r#".#....#####...#..
                       ##...##.#####..##
                       ##...#...#.#####.
                       ..#.....#...###..
                       ..#.#.....#....##"#;
        let world: World = input.parse().unwrap();
        assert_eq!(part2(&world, (8, 3)), 1403);


        let input = r#".#..##.###...#######
                       ##.############..##.
                       .#.######.########.#
                       .###.#######.####.#.
                       #####.##.#.##.###.##
                       ..#####..#.#########
                       ####################
                       #.####....###.#.#.##
                       ##.#################
                       #####.##.###..####..
                       ..######..##.#######
                       ####.##.####...##..#
                       .#####..#.######.###
                       ##...#.##########...
                       #.##########.#######
                       .####.#.###.###.#.##
                       ....##.##.###..#####
                       .#.#.###########.###
                       #.#.#.#####.####.###
                       ###.##.####.##.#..##"#;

        let world: World = input.parse().unwrap();
        assert_eq!(part2(&world, (11, 13)), 802);
    }
}
