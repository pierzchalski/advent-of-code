fn fuel_requirement(mass: isize) -> isize {
    (mass / 3) - 2
}

fn complete_fuel_requirements(mass: isize) -> isize {
    let mut total = fuel_requirement(mass);
    let mut extra = total;
    loop {
        extra = fuel_requirement(extra);
        if extra <= 0 {
            break;
        };
        total += extra;
    }
    total
}

#[test]
fn problem_1_examples() {
    assert_eq!(fuel_requirement(12), 2);
    assert_eq!(fuel_requirement(14), 2);
    assert_eq!(fuel_requirement(1969), 654);
    assert_eq!(fuel_requirement(100756), 33583);
}

#[test]
fn problem_1() {
    let result: isize = INPUT.iter().copied().map(fuel_requirement).sum();
    assert_eq!(result, PROBLEM_1_SOLUTION);
}

#[test]
fn problem_2_examples() {
    assert_eq!(complete_fuel_requirements(1969), 966);
    assert_eq!(complete_fuel_requirements(100756), 50346);
}

#[test]
fn problem_2() {
    let result: isize = INPUT.iter().copied().map(complete_fuel_requirements).sum();
    assert_eq!(result, PROBLEM_2_SOLUTION);
}

static PROBLEM_1_SOLUTION: isize = 3295539;

static PROBLEM_2_SOLUTION: isize = 4940441;

static INPUT: &[isize] = &[
    50951, 69212, 119076, 124303, 95335, 65069, 109778, 113786, 124821, 103423, 128775, 111918,
    138158, 141455, 92800, 50908, 107279, 77352, 129442, 60097, 84670, 143682, 104335, 105729,
    87948, 59542, 81481, 147508, 62687, 64212, 66794, 99506, 137804, 135065, 135748, 110879,
    114412, 120414, 72723, 50412, 124079, 57885, 95601, 74974, 69000, 66567, 118274, 136432,
    110395, 88893, 124962, 74296, 106148, 59764, 123059, 106473, 50725, 116256, 80314, 60965,
    134002, 53389, 82528, 144323, 87791, 128288, 109929, 64373, 114510, 116897, 84697, 75358,
    109246, 110681, 94543, 92590, 69865, 83912, 124275, 94276, 98210, 69752, 100315, 142879, 94783,
    111939, 64170, 83629, 138743, 141238, 77068, 119299, 81095, 96515, 126853, 87563, 101299,
    130240, 62693, 139018,
];
