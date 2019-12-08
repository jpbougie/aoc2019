use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input)?;
    let layer_width = 25;
    let layer_height = 6;

    assert_eq!(input.len() % (layer_width * layer_height), 0);
    assert!(input.iter().all(|x| x >= &b'0' && x <= &b'9'));
    println!("Part 01: {:?}", part1(&input, layer_width, layer_height));

    println!("Part 02");
    part2(&input, layer_width, layer_height);
    Ok(())
}

fn part1(input: &[u8], layer_width: usize, layer_height: usize) -> Option<usize> {
    let (min_zero_layer, _zeroes) = input.chunks(layer_width * layer_height).fold((None, usize::max_value()), |acc, layer| {
        let zeroes = layer.iter().filter(|el| **el == b'0').count();
        if zeroes < acc.1 {
            (Some(layer), zeroes)
        } else {
            acc
        }
    });

    if let Some(layer) = min_zero_layer {
        Some(layer.iter().filter(|el| **el == b'1').count() * layer.iter().filter(|el| **el == b'2').count())
    } else {
        None
    }
}

fn part2(input: &[u8], layer_width: usize, layer_height: usize) {
    let layers = input.chunks(layer_width * layer_height).collect::<Vec<&[u8]>>();

    let mut out = Vec::with_capacity(layer_width * layer_height);
    for i in 0..(layer_width * layer_height) {
        out.push(layers.iter().find(|layer| layer[i] != b'2').map(|layer| layer[i]).unwrap_or(b'0'));
    }

    for row in out.chunks(layer_width) {
        println!("{}", String::from_utf8(row.iter().map(|x| if x == &b'1' { b'1' } else { b' ' }).collect::<Vec<u8>>()).unwrap());
    }
}
