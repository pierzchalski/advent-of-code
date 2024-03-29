#[derive(Debug, PartialEq, Eq)]
struct Spec {
    start: usize,
    end: usize,
}

// turns "123" into vec![0, 0, 0, 1, 2, 3]
fn digits(x: usize) -> Vec<u8> {
    // yes this is icky, suck it up
    format!("{:06}", x)
        .into_bytes()
        .into_iter()
        .map(|b| (b as char).to_digit(10).unwrap() as u8)
        .collect()
}

fn increasing(ds: &[u8]) -> bool {
    ds.windows(2).all(|x| x[0] <= x[1])
}

#[test]
fn test_increasing() {
    assert!(increasing(&[1, 1, 1, 1]));
    assert!(increasing(&[0, 1, 1]));
    assert!(!increasing(&[2, 2, 3, 4, 5, 0]));
}

// Suspicous because we aren't checking adjacency,
// but because the numbers are nondecreasing, any valid
// number with a group only has one group.
fn has_doubles(ds: &[u8]) -> bool {
    let mut counts = [0u8; 10];
    for d in ds.iter().copied() {
        counts[d as usize] += 1;
        if counts[d as usize] >= 2 {
            return true;
        }
    }
    false
}

fn has_exact_double(ds: &[u8]) -> bool {
    let mut counts = [0u8; 10];
    for d in ds.iter().copied() {
        counts[d as usize] += 1;
    }
    // translation: there is at least one group of exactly 2 digits.
    counts.iter().filter(|count| **count == 2).count() > 0
}

fn valid(candidate: usize) -> bool {
    let ds = digits(candidate);
    increasing(&ds) && has_doubles(&ds)
}

impl Spec {
    fn from_str(input: &str) -> Self {
        let mut fields = input.split('-');
        let start = fields.next().unwrap().parse().unwrap();
        let end = fields.next().unwrap().parse().unwrap();
        Spec { start, end }
    }

    fn count_valid(&self) -> usize {
        (self.start..=self.end).filter(|x| valid(*x)).count()
    }

    fn count_valider(&self) -> usize {
        (self.start..=self.end)
            .filter(|candidate| {
                let ds = digits(*candidate);
                increasing(&ds) && has_exact_double(&ds)
            })
            .count()
    }
}

#[test]
fn spec_from_str() {
    assert_eq!(
        Spec::from_str(INPUT),
        Spec {
            start: 231832,
            end: 767346
        }
    )
}

#[test]
fn problem_1_examples() {
    assert!(valid(111111));
    assert!(valid(11111));
    assert!(!valid(223450));
    assert!(!valid(123789));
}

#[test]
fn problem_1() {
    assert_eq!(Spec::from_str(INPUT).count_valid(), 1330)
}

#[test]
fn problem_2_examples() {
    assert!(has_exact_double(&[1, 1, 2, 2, 3, 3]));
    assert!(!has_exact_double(&[1, 2, 3, 4, 4, 4]));
    assert!(has_exact_double(&[1, 1, 1, 1, 2, 2]));
}

#[test]
fn problem_2() {
    assert_eq!(Spec::from_str(INPUT).count_valider(), 876)
}

static INPUT: &str = "231832-767346";
