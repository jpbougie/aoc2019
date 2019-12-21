use std::io::{self, Read};
use pathfinding::prelude::{astar,absdiff};
use std::collections::{BinaryHeap, HashSet, HashMap};

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut maze: Maze = input.parse()?;
    println!("{}", maze);
    println!("Part 01: {}", shortest_path(&maze).unwrap());

    multiply_robots(&mut maze);
    println!("Part 02: {}", shortest_path(&maze).unwrap());
    Ok(())
}


fn multiply_robots(maze: &mut Maze) {
    let entrance = maze.entrances()[0].1;

    maze.update_tile(&entrance, Tile::Wall);
    maze.update_tile(&entrance.north(), Tile::Wall);
    maze.update_tile(&entrance.south(), Tile::Wall);
    maze.update_tile(&entrance.east(), Tile::Wall);
    maze.update_tile(&entrance.west(), Tile::Wall);
    maze.update_tile(&entrance.west().north(), Tile::Entrance('1'));
    maze.update_tile(&entrance.west().south(), Tile::Entrance('2'));
    maze.update_tile(&entrance.east().north(), Tile::Entrance('3'));
    maze.update_tile(&entrance.east().south(), Tile::Entrance('4'));
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct SearchState {
    cost_so_far: u64,
    origins: Vec<char>,
    keys_obtained: HashSet<char>,
}

use std::cmp::{Ord, PartialOrd, Ordering};
impl Ord for SearchState {
    fn cmp(&self, other: &SearchState) -> Ordering {
        other.cost_so_far.cmp(&self.cost_so_far).then_with(|| self.keys_obtained.len().cmp(&other.keys_obtained.len()))
    }
}

impl PartialOrd for SearchState {
    fn partial_cmp(&self, other: &SearchState) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn shortest_path(maze: &Maze) -> Option<u64> {
    let positions = maze.entrances();
    let paths = maze.paths();
    let key_positions = maze.keys();

    let mut result_paths = HashMap::new();
    let mut final_states = Vec::new();

    let mut states_to_visit = BinaryHeap::new();
    let initial_state = SearchState{cost_so_far: 0, origins: positions.into_iter().map(|x| x.0).collect(), keys_obtained: HashSet::new()};
    states_to_visit.push(initial_state);

    while let Some(state) = states_to_visit.pop() {
        if state.keys_obtained.len() == key_positions.len() {
            final_states.push(state.cost_so_far);
        }
        for (i, origin) in state.origins.iter().enumerate() {
            for k in key_positions.keys().cloned() {
                if state.keys_obtained.contains(&k) {
                    continue
                }
                if let Some((deps, cost)) = paths.get(&(*origin, k)) {
                    if deps.is_subset(&state.keys_obtained) {
                        let mut new_state = state.clone();
                        new_state.cost_so_far += cost;
                        new_state.keys_obtained.insert(k);
                        let mut keys_obtained = new_state.keys_obtained.iter().cloned().collect::<Vec<char>>();
                        keys_obtained.sort();
                        let keys_obtained: String = keys_obtained.into_iter().collect();

                        new_state.origins.remove(i);
                        new_state.origins.push(k);

                        let mut positions = new_state.origins.iter().cloned().collect::<Vec<char>>();
                        positions.sort();
                        let positions: String = positions.into_iter().collect();
                        //println!("For next key {}, checking path that gives '{}'", k, keys_obtained);
                        let entry = result_paths.entry((keys_obtained, positions)).or_insert(u64::max_value());
                        if *entry > new_state.cost_so_far {
                            *entry = new_state.cost_so_far;
                        } else {
                            //println!("Skipping {:?}", new_state);
                            continue;
                        }
                        states_to_visit.push(new_state);
                    }
                }
            }
        }
    }

    final_states.iter().min().map(|x| *x)
}

fn accessible_keys(maze: &Maze, pos: &Addr) -> Vec<(char, Addr)> {
    let mut to_visit = vec![*pos];
    let mut visited = HashSet::new();
    let mut keys = Vec::new();

    while let Some(addr) = to_visit.pop() {
        if visited.contains(&addr) {
            continue
        }

        visited.insert(addr);

        if let Some(tile) = maze.tile_at(&addr) {
            if let Tile::Key(k) = tile {
                keys.push((k, addr.clone()));
            }
        }

        let neighbours = addr.neighbours();
        let mut neighbours = neighbours.iter().filter(|n| maze.tile_at(&n).map(|t| t.walkable()).unwrap_or(false)).cloned().collect::<Vec<_>>();
        to_visit.append(&mut neighbours);
    }

    keys
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Maze {
    tiles: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
    keys_left: usize,
}

impl Maze {
    fn keys(&self) -> HashMap<char, Addr> {
        self.tiles.iter().enumerate().flat_map(|(y, line)| line.iter().enumerate().filter_map(|(x, t)| {
            match *t {
                Tile::Key(ch) => Some((ch, Addr{x: x as i64, y: y as i64})),
            _ => None
        }}).collect::<HashMap<char, Addr>>()).collect()
    }

    fn doors(&self) -> Vec<Addr> {
        self.tiles.iter().enumerate().flat_map(|(y, line)| line.iter().enumerate().filter_map(|(x, t)| {
            match *t {
                Tile::Key(_) => Some(Addr{x: x as i64, y: y as i64}),
            _ => None
        }}).collect::<Vec<Addr>>()).collect()
    }

    fn tile(&self, tile: &Tile) -> Option<Addr> {
        self.tiles.iter().enumerate().filter_map(|(y, line)| line.iter().enumerate().find(|(_x, t)| t == &tile).map(|(x, _)| Addr{x: x as i64, y: y as i64})).next()
    }

    fn tiles(&self, tile_spec: &Tile) -> Vec<Addr> {
        self.tiles.iter().enumerate().flat_map(|(y, line)| line.iter().enumerate().filter(|(_x, t)| t == &tile_spec).map(|(x, _)| Addr{x: x as i64, y: y as i64}).collect::<Vec<_>>()).collect()
    }

    fn tile_at(&self, addr: &Addr) -> Option<Tile> {
        if addr.y < 0 || addr.y as usize >= self.height {
            return None
        }

        if addr.x < 0 || addr.x as usize >= self.width {
            return None
        }

        Some(self.tiles[addr.y as usize][addr.x as usize])
    }

    fn path_between(&self, origin: &Addr, dest: &Addr) -> Option<(Vec<Addr>, u64)> {
        let successors = |node: &Addr| node.neighbours().iter().filter(|n| self.tile_at(&n).map(|t| t != Tile::Wall).unwrap_or(false)).cloned().map(|addr| (addr, 1)).collect::<Vec<_>>();
        astar(origin, successors, |p| p.distance(dest), |p| p == dest)
    }

    fn entrances(&self) -> Vec<(char, Addr)> {
        self.tiles.iter().enumerate().flat_map(|(y, line)| line.iter().enumerate().filter(|(_x, t)| match t {
            Tile::Entrance(_) => true,
            _ => false
        }).map(|(x, t)| (t.symbol(), Addr{x: x as i64, y: y as i64})).collect::<Vec<_>>()).collect()
    }

    fn paths(&self) -> HashMap<(char, char), (HashSet<char>, u64)> {
        let mut paths = HashMap::new();
        let keys = self.keys();

        for (entrance, entrance_addr) in self.entrances() {
            for (key, key_addr) in keys.iter() {
                if let Some((tiles, cost)) = self.path_between(&entrance_addr, key_addr) {
                    // a path is invalid if we would pick up a key on the way
                    if tiles.iter().any(|t| {
                        match self.tile_at(t) {
                            Some(Tile::Key(k)) if k != *key => true,
                            _ => false
                        }}) {
                        println!("Skipping path between origin and {}", key);
                        continue
                    }
                    let keys_required = tiles.into_iter().filter_map(|addr| if let Some(Tile::Door(ch)) = self.tile_at(&addr) { Some(ch.to_ascii_lowercase()) } else { None }).collect();
                    paths.insert((entrance, key.clone()), (keys_required, cost));
                }
            }
        }

        for (key_orig, key_orig_addr) in keys.iter() {
            for (key_dest, key_dest_addr) in keys.iter() {
                if key_orig != key_dest {
                    if let Some((tiles, cost)) = self.path_between(key_orig_addr, key_dest_addr) {
                        let keys_required: HashSet<char> = tiles.iter().filter_map(|addr| if let Some(Tile::Door(ch)) = self.tile_at(&addr) { Some(ch.to_ascii_lowercase()) } else { None }).collect();
                        // a path is invalid if we would pick up a key on the way
                        if keys_required.len() == 0 && tiles.iter().any(|t| {
                            match self.tile_at(t) {
                                Some(Tile::Key(k)) if k != *key_orig && k != *key_dest => true,
                                _ => false
                            }}) {
                            println!("Skipping path between {} and {}", key_orig, key_dest);
                            continue
                        }
                        paths.insert((key_orig.clone(), key_dest.clone()), (keys_required, cost));
                    }
                }
            }
        }

        paths
    }

    fn update_tile(&mut self, addr: &Addr, tile: Tile) {
        self.tiles[addr.y as usize][addr.x as usize] = tile;
    }

    fn unlock_door(&mut self, key: char) {
        if let Some(key_addr) = self.tile(&Tile::Key(key)) {
            self.keys_left -= 1;
            self.tiles[key_addr.y as usize][key_addr.x as usize] = Tile::Passage;
            if let Some(door_addr) = self.tile(&Tile::Door(key.to_ascii_uppercase())) {
                self.tiles[door_addr.y as usize][door_addr.x as usize] = Tile::Passage;
            }
        }
    }
}

impl std::str::FromStr for Maze {
    type Err = io::Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut entrance = b'0';
        let mut keys_left = 0;
        let tiles = input.lines().map(|line| line.chars().map(|ch| match ch {
            '#' => Tile::Wall,
            '@' => {
                entrance += 1;
                Tile::Entrance(entrance as char)
            },
            '.' => Tile::Passage,
            'a' ... 'z' => {
                keys_left += 1;
                Tile::Key(ch)
            },
            'A' ... 'Z' => Tile::Door(ch),
            _ => unreachable!()
        }).collect::<Vec<Tile>>()).collect::<Vec<_>>();
        let width = tiles[0].len();
        let height = tiles.len();

        Ok(Maze{tiles, width, height, keys_left})
    }

}

use std::fmt;
impl fmt::Display for Maze {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.tiles.iter() {
            for tile in line {
                write!(f, "{}", match tile {
                    Tile::Wall => '#',
                    Tile::Passage => '.',
                    Tile::Entrance(_) => '@',
                    Tile::Door(ch) => *ch,
                    Tile::Key(ch) => *ch,
                })?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}


#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
enum Tile {
    Wall,
    Passage,
    Entrance(char),
    Key(char),
    Door(char),
}

impl Tile {
    fn walkable(&self) -> bool {
        match *self {
            Tile::Wall => false,
            Tile::Door(_) => false,
            _ => true
        }
    }

    fn symbol(&self) -> char {
        match *self {
            Tile::Entrance(x) => x,
            Tile::Door(x) => x,
            Tile::Key(x) => x,
            _ => unreachable!()
        }
    }
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

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
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

    fn neighbours(&self) -> Vec<Addr> {
        vec![self.north(), self.west(), self.east(),  self.south()]
    }

    fn neighbours_with_dir(&self) -> Vec<(Dir, Addr)> {
        vec![(Dir::North, self.north()), (Dir::West, self.west()), (Dir::East, self.east()), (Dir::South, self.south())]
    }

    fn distance(&self, other: &Addr) -> u64 {
        (absdiff(self.x, other.x) + absdiff(self.y, other.y)) as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let input = "#########\n#b.A.@.a#\n#########";
        let maze: Maze = input.parse().unwrap();
        assert_eq!(shortest_path(&maze), Some(8));
    }

    #[test]
    fn test_bigger() {
        let input = "########################\n#f.D.E.e.C.b.A.@.a.B.c.#\n######################.#\n#d.....................#\n########################";
        let maze: Maze = input.parse().unwrap();
        assert_eq!(shortest_path(&maze), Some(86));
    }
}
