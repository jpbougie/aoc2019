fn main() {
    println!("Part 01: {}", (171309..=643603).filter(|x| matches(&digits(*x))).count());
    println!("Part 02: {}", (171309..=643603).filter(|x| matches02(&digits(*x))).count());
}


fn digits(input: usize) -> Vec<usize> {
    let mut digits = Vec::with_capacity(6);
    let mut input = input;
    loop {
        if input == 0 {
            break
        }

        digits.push(input % 10);
        input /= 10;
    }

    digits.reverse();
    digits
}

fn matches(input: &[usize]) -> bool {
    let pairs = input.windows(2).map(|wdw| (wdw[0], wdw[1])).collect::<Vec<(usize, usize)>>();

    pairs.iter().any(|(x, y)| x == y) && pairs.iter().all(|(x, y)| x <= y )
}


#[derive(Debug, Clone, Copy)]
struct Run {
    value: usize,
    len: usize,
}

fn runs(input: &[usize]) -> Vec<Run> {
    let mut it = input.iter();
    let mut values = Vec::new();
    let mut cur = Run{value: *it.next().unwrap(), len: 1};
    for val in it {
        if *val == cur.value {
            cur.len += 1;
        } else {
            values.push(cur);
            cur = Run{value: *val, len: 1};
        }
    }

    values.push(cur);

    values
}

fn matches02(input: &[usize]) -> bool {
    let runs = runs(input);

    runs.windows(2).all(|rs| rs[0].value < rs[1].value) &&
        runs.iter().any(|r| r.len == 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digits() {
        assert_eq!(digits(123456), &[1,2,3,4,5,6]);
    }

    #[test]
    fn test_matches() {
        assert!(matches(&vec![1, 1, 1, 1, 1, 1]));
        assert!(!matches(&digits(223450)));
        assert!(!matches(&digits(123789)));
    }

    #[test]
    fn test_matches02() {
        assert!(!matches02(&vec![1, 1, 1, 1, 1, 1]));
        assert!(!matches02(&digits(123444)));
        assert!(matches02(&digits(111122)));
    }

}
