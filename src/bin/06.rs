use std::collections::{HashMap, HashSet, BinaryHeap};
use std::io::{self, Read};

fn main() -> std::io::Result<()> {
    let mut puzzle = String::new();
    io::stdin().read_to_string(&mut puzzle)?;
    let world: World = puzzle.parse()?;
    println!("Part 1: {}", world.total_orbits());

    println!("Part 2: {}", world.steps_between("YOU", "SAN").unwrap());
    Ok(())
}

#[derive(Debug)]
struct World {
    nodes: HashSet<String>,
    edges: HashMap<String, HashSet<String>>,
}

impl World {
    fn total_orbits(&self) -> usize {
        self.slow_total_orbits("COM", 0)
    }

    fn slow_total_orbits(&self, node: &str, depth: usize) -> usize {
        let res = match self.edges.get(node) {
            None => depth,
            Some(edges) => {
                edges.iter().fold(depth, |acc, destination| acc + self.slow_total_orbits(destination, depth + 1))
            }
        };
        res
    }

    fn bidirectional_edges(&self) -> HashMap<String, HashSet<String>> {
        let mut result = self.edges.clone();
        for (origin, destinations) in self.edges.iter() {
            for dest in destinations.iter() {
                let entry = result.entry(dest.to_string()).or_insert_with(HashSet::new);
                entry.insert(origin.to_string());
            }
        }

        result
    }

    fn steps_between(&self, from: &str, to: &str) -> Option<i64> {
        let all_edges = self.bidirectional_edges();
        let mut visited = HashSet::new();
        let mut to_visit = BinaryHeap::new();
        to_visit.push(Steps{node: from.to_string(), steps: -1});
        while let Some(Steps{ node, steps }) = to_visit.pop() {
            if let Some(edges) = all_edges.get(&node) {
                if edges.contains(to) {
                    return Some(steps)
                }
            }

            if visited.contains(&node) { continue; }

            visited.insert(node.to_string());

            if let Some(edges) = all_edges.get(&node) {
                for edge in edges.iter() {
                    to_visit.push(Steps{node: edge.to_string(), steps: steps + 1});
                }
            }
        }

        None
    }
}

#[derive(PartialEq, Eq)]
struct Steps {
    node: String,
    steps: i64,
}

use std::cmp::Ordering;
impl Ord for Steps {
    fn cmp(&self, other: &Steps) -> Ordering {
        other.steps.cmp(&self.steps).then_with(|| other.node.cmp(&self.node))
    }
}
impl PartialOrd for Steps {
    fn partial_cmp(&self, other: &Steps) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl std::str::FromStr for World {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut world = World{
            nodes: HashSet::new(),
            edges: HashMap::new(),
        };

        for edge in s.lines() {
            let parts = edge.split(")").collect::<Vec<&str>>();
            assert_eq!(parts.len(), 2);
            let from = parts[0].to_string();
            let to = parts[1].to_string();
            world.nodes.insert(from.to_owned());
            world.nodes.insert(to.to_owned());
            let entry = world.edges.entry(from).or_insert_with(HashSet::new);
            entry.insert(to);
        }

        Ok(world)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let input = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L";
        let world: io::Result<World> = input.parse();
        assert!(world.is_ok());
        let world = world.unwrap();
        assert_eq!(world.nodes.len(), 12);
        assert_eq!(world.edges.len(), 8);
        assert_eq!(world.edges.values().fold(0, |acc, item| acc + item.len()), 11);
    }

    #[test]
    fn solve_part1() {
        let input = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L";
        let world: World = input.parse().unwrap();
        assert_eq!(world.total_orbits(), 42);
    }

    #[test]
    fn solve_part2() {
        let input = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN";
        let world: World = input.parse().unwrap();
        assert_eq!(world.steps_between("YOU", "SAN"), Some(4));
    }
}
