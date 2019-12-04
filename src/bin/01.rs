use std::io::{self, BufRead};

fn main() {
    let masses = io::stdin().lock().lines().map(|x| x.unwrap().parse().unwrap()).collect::<Vec<i64>>();
    //part 1
    println!("Result: {}", masses.iter().fold(0, |x, y| x + fuel(*y)));
    //part 2
    println!("Result: {}", masses.iter().fold(0, |x, y| x + recursive_fuel(*y)));
}

fn fuel(mass: i64) -> i64 {
    mass / 3 - 2
}

fn recursive_fuel(mass: i64) -> i64 {
    let mut total_fuel = 0;
    let mut mass = mass;

    loop {
        let additional_fuel = fuel(mass);
        if additional_fuel <= 0 {
            break
        }
        total_fuel += additional_fuel;
        mass = additional_fuel;
    }

    total_fuel
}

#[cfg(test)]
mod tests {
    use super::{fuel, recursive_fuel};
    #[test]
    fn examples_part1() {
        assert_eq!(fuel(12), 2);
        assert_eq!(fuel(14), 2);
        assert_eq!(fuel(1969), 654);
        assert_eq!(fuel(100756), 33583);
    }

    #[test]
    fn examples_part2() {
        assert_eq!(recursive_fuel(14), 2);
        assert_eq!(recursive_fuel(1969), 966);
        assert_eq!(recursive_fuel(100756), 50346);
    }
}
