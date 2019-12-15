use std::io::{self, Read};
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let reactions = parse(&input)?;
    let req = ore_required(&reactions, Item::new(1, "FUEL"), "ORE");
    println!("Part 01: {}", req);
    println!("Part 02: {}", bsearch(&reactions, 1000000000000, 1000000000000 / req, 2 * 1000000000000 / req));
    Ok(())
}

fn bsearch(reactions: &[Reaction], limit: usize, lower: usize, upper: usize) -> usize {
    let mut lower = lower;
    let mut upper = upper;
    while lower < upper - 1 {
        let pivot = (lower + upper) / 2;
        let req = ore_required(&reactions, Item::new(pivot, "FUEL"), "ORE");
        println!("lower: {}, upper: {}, pivot: {}, req: {}", lower, upper, pivot, req);
        if req == limit {
            return pivot
        } else if req < limit {
            lower = pivot;
        } else {
            upper = pivot;
        }
    }

    lower
}

fn fuel_produced(reactions: &[Reaction], goal: &str, base_item: &str) -> usize {
    let mut inventory: HashMap<String, usize> = HashMap::new();
    inventory.insert(base_item.to_string(), 1000000000000);
    let mut required = Vec::new();
    let mut produced = 0;

    loop {
        required.push(Item::new(1, goal));
        while let Some(req) = required.pop() {
            let mut current_stock = inventory.entry(req.name.clone()).or_insert(0);

            if *current_stock >= req.quantity {
                *current_stock -= req.quantity;
                continue
            }

            let qty_required = req.quantity - *current_stock;
            *current_stock = 0;

            let reaction = match reactions.iter().find(|reac| reac.output.name == req.name) {
                Some(reaction) => reaction,
                None => {
                    return produced
                }
            };


            let multiplier = if qty_required % reaction.output.quantity == 0 {
                qty_required / reaction.output.quantity
            } else {
                qty_required / reaction.output.quantity + 1
            };

            let total_produced = multiplier * reaction.output.quantity;

            *current_stock +=  total_produced - qty_required;

            for input in reaction.inputs.iter() {
                let qty_required = multiplier * input.quantity;
                required.push(Item::new(qty_required, &input.name));
            }
        }

        produced += 1;
        if produced % 1000 == 0 {
            println!("{} fuel produced, {} ore left", produced, inventory.get(base_item).unwrap_or(&0));
        }
    }

    0
}

fn ore_required(reactions: &[Reaction], goal: Item, base_item: &str) -> usize {
    let mut inventory: HashMap<String, usize> = HashMap::new();
    let mut required = Vec::new();
    required.push(goal);

    while let Some(req) = required.pop() {
        let mut current_stock = inventory.entry(req.name.clone()).or_insert(0);

        if req.name == base_item {
            *current_stock += req.quantity;
            continue
        }

        if *current_stock >= req.quantity {
            *current_stock -= req.quantity;
            continue
        }

        let qty_required = req.quantity - *current_stock;
        *current_stock = 0;

        let reaction = reactions.iter().find(|reac| reac.output.name == req.name).unwrap();


        let multiplier = if qty_required % reaction.output.quantity == 0 {
            qty_required / reaction.output.quantity
        } else {
            qty_required / reaction.output.quantity + 1
        };

        let total_produced = multiplier * reaction.output.quantity;

        *current_stock +=  total_produced - qty_required;

        for input in reaction.inputs.iter() {
            let qty_required = multiplier * input.quantity;
            required.push(Item::new(qty_required, &input.name));
        }
    }

    *inventory.get(base_item).unwrap_or(&0)
}

fn parse(input: &str) -> io::Result<Vec<Reaction>> {
    input.lines().map(FromStr::from_str).collect()
}

#[derive(Debug, Eq, PartialEq)]
struct Item {
    quantity: usize,
    name: String
}

impl Item {
    fn new(quantity: usize, name: &str) -> Self {
        Item{quantity, name: name.to_string()}
    }
}

use std::str::FromStr;
use std::num::ParseIntError;
impl FromStr for Item {
    type Err = io::Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut parts = input.split(" ");
        let quantity = parts.next().unwrap();
        let name = parts.next().unwrap();
        Ok(Item{ quantity: quantity.parse().map_err(|e: ParseIntError| io::Error::new(io::ErrorKind::InvalidData, Box::new(e)))?, name: name.to_string()})
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Reaction {
    inputs: Vec<Item>,
    output: Item,
}

impl FromStr for Reaction {
    type Err = io::Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut parts = input.split(" => ");
        let inputs = parts.next().unwrap();
        let output = parts.next().unwrap();

        Ok(Reaction{inputs: inputs.split(", ").map(|input| input.parse()).collect::<Result<_, _>>()?, output: output.parse()?})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let reaction = "7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL".parse().unwrap();
        assert_eq!(Reaction{
            output: Item::new(4, "PLWSL"),
            inputs: vec![Item::new(7, "ZLQW"), Item::new(3, "BMBT"), Item::new(9, "XCVML"), Item::new(26, "XMNCP"), Item::new(1, "WPTQ"), Item::new(2, "MZWV"), Item::new(1, "RJRHP")]}, reaction);
    }

    #[test]
    fn test_ore_required() {
        let input = parse(r#"10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL"#).unwrap();
        assert_eq!(ore_required(&input, "FUEL", "ORE"), 31);

    }
}
