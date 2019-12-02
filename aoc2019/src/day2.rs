// Runs the machine on the input state, returning the new program counter (if any).
// Could be nicer; will write a nicer one when more `Intcode` problems show up.
fn step(pc: usize, state: &mut [usize]) -> Option<usize> {
    let opcode = state[pc];
    match opcode {
        1 | 2 => {
            let in1 = state[state[pc + 1]];
            let in2 = state[state[pc + 2]];
            let out = state[pc + 3];
            match opcode {
                1 => {
                    state[out] = in1 + in2;
                }
                2 => {
                    state[out] = in1 * in2;
                }
                _ => unreachable!(),
            }
            Some(pc + 4)
        }
        99 => None,
        _ => panic!("Unrecognised opcode {} at pc {}", opcode, pc),
    }
}

fn run(state: &mut [usize]) {
    let mut pc = 0;
    while let Some(new_pc) = step(pc, state) {
        pc = new_pc;
    }
}

#[test]
fn problem_1_examples() {
    let mut state = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
    run(&mut state);
    assert_eq!(&state, &[3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);

    let mut state = vec![1, 0, 0, 0, 99];
    run(&mut state);
    assert_eq!(&state, &[2, 0, 0, 0, 99]);

    let mut state = vec![2, 3, 0, 3, 99];
    run(&mut state);
    assert_eq!(&state, &[2, 3, 0, 6, 99]);

    let mut state = vec![2, 4, 4, 5, 99, 0];
    run(&mut state);
    assert_eq!(&state, &[2, 4, 4, 5, 99, 9801]);

    let mut state = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
    run(&mut state);
    assert_eq!(&state, &[30, 1, 1, 4, 2, 5, 6, 0, 99]);
}

#[test]
fn problem_1() {
    let mut state = input();
    state[1] = 12;
    state[2] = 2;
    run(&mut state);

    assert_eq!(state[0], 4930687);
}

#[test]
fn problem_2() {
    let mut result = None;
    'done: for noun in 0..=99 {
        for verb in 0..=99 {
            let mut state = input();
            state[1] = noun;
            state[2] = verb;
            run(&mut state);
            if state[0] == 19690720 {
                result = Some(100 * noun + verb);
                break 'done;
            }
        }
    }
    assert_eq!(result, Some(5335));
}

fn input() -> Vec<usize> {
    vec![
        1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 13, 1, 19, 1, 10, 19, 23, 1, 23, 9, 27,
        1, 5, 27, 31, 2, 31, 13, 35, 1, 35, 5, 39, 1, 39, 5, 43, 2, 13, 43, 47, 2, 47, 10, 51, 1,
        51, 6, 55, 2, 55, 9, 59, 1, 59, 5, 63, 1, 63, 13, 67, 2, 67, 6, 71, 1, 71, 5, 75, 1, 75, 5,
        79, 1, 79, 9, 83, 1, 10, 83, 87, 1, 87, 10, 91, 1, 91, 9, 95, 1, 10, 95, 99, 1, 10, 99,
        103, 2, 103, 10, 107, 1, 107, 9, 111, 2, 6, 111, 115, 1, 5, 115, 119, 2, 119, 13, 123, 1,
        6, 123, 127, 2, 9, 127, 131, 1, 131, 5, 135, 1, 135, 13, 139, 1, 139, 10, 143, 1, 2, 143,
        147, 1, 147, 10, 0, 99, 2, 0, 14, 0,
    ]
}
