use std::io::{self, Read};
fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input);
    let digits = input.trim().chars().map(|ch| ch.to_digit(10).unwrap() as u8).collect::<Vec<u8>>();
    println!("Part 01: {}", to_number(&apply_phases(&digits, 100)[0..8]));

    let offset = to_number(&digits[0..7]);
    let digits = (0..10000).flat_map(|_| digits.clone()).skip((offset - 1) as usize).collect::<Vec<u8>>();
    println!("offset: {}", offset);
    println!("Part 02: {}", solve_at_offset(&digits, 100, 8));
    Ok(())
}

fn to_number(i: &[u8]) -> u64 {
    i.iter().fold(0, |acc, i| acc * 10 + (*i as u64))
}

fn apply_phases(input: &[u8], n: usize) -> Vec<u8> {
    let pattern = vec![0, 1, 0, -1];
    (0..n).fold(input.to_vec(), |acc, _| apply_phase(&pattern, &acc))
}

use itertools;
fn apply_phase(base_pattern: &[i8], input: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(input.len());

    for i in 1..=input.len() {
        let pattern = base_pattern.iter().flat_map(|x| itertools::repeat_n(*x, i)).collect::<Vec<i8>>();
        let pattern = pattern.into_iter().cycle().skip(1);
        let result = input.iter().zip(pattern).map(|(x, y)| (*x as i64) * (y as i64)).sum::<i64>().abs() % 10;
        out.push(result as u8);
    }

    out
}

// assume that we can skip the pattern as the offset is far enough ahead that we only will have 
fn solve_at_offset(input: &[u8], iters: usize, outsize: usize) -> u64 {
    let l = input.len();
    let mut input = input.to_vec();
    for _i in 0..iters {
        input = input.iter().rev().scan(0, |acc, x| {
            *acc = *acc + (*x as u64);
            Some(*acc)
        }).map(|x| (x % 10) as u8).collect();
        input.reverse();
        assert_eq!(l, input.len());
    }

    println!("{:?}", &input[0..outsize]);
    to_number(&input[1..=outsize])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        assert_eq!(apply_phase(&[0, 1, 0, -1][..], &[1,2,3,4,5,6,7,8][..]), vec![4,8,2,2,6,1,5,8]);
        assert_eq!(apply_phases(&[1,2,3,4,5,6,7,8][..], 4), vec![0,1,0,2,9,4,9,8]);
    }

    #[test]
    fn shortcut() {
        assert_eq!(solve_at_offset(&[5, 6, 7, 8][..], 1, 4), 6158);
        assert_eq!(solve_at_offset(&[5, 6, 7, 8][..], 4, 4), 9498);
    }
}
