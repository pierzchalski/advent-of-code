use crate::intcode::Machine;
use itertools::Itertools;

struct Amp {
    machine: Machine,
    phase: isize,
}

impl Amp {
    fn run(&mut self, input: isize) -> isize {
        self.machine.input.push_back(self.phase);
        self.machine.input.push_back(input);
        self.machine.run();
        self.machine.output.pop().unwrap()
    }
}

struct AmpChain {
    amps: Vec<Amp>,
}

impl AmpChain {
    fn from_phases(machine: &Machine, phases: impl Iterator<Item = isize>) -> Self {
        let amps = phases
            .map(|phase| Amp {
                machine: machine.clone(),
                phase,
            })
            .collect();
        AmpChain { amps }
    }

    fn get_signal(&mut self) -> isize {
        let mut signal = 0;
        for amp in self.amps.iter_mut() {
            signal = amp.run(signal);
        }
        signal
    }
}

struct Problem(Machine);

impl Problem {
    fn from_program(spec: &str) -> Self {
        Problem(Machine::from_mem_spec(spec))
    }

    fn max_signal(&self) -> isize {
        let phases = [0, 1, 2, 3, 4];
        let machine = &self.0;
        phases
            .iter()
            .permutations(5)
            .map(|perm| AmpChain::from_phases(machine, perm.into_iter().copied()).get_signal())
            .max()
            .unwrap()
    }
}

#[test]
fn problem_1_examples() {
    let program = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
    let problem = Problem::from_program(program);
    assert_eq!(problem.max_signal(), 43210);

    let program = "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0";
    let problem = Problem::from_program(program);
    assert_eq!(problem.max_signal(), 54321);

    let program = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0";
    let problem = Problem::from_program(program);
    assert_eq!(problem.max_signal(), 65210);
}

#[test]
fn problem_1() {
    assert_eq!(Problem::from_program(INPUT).max_signal(), 17790);
}

static INPUT: &str = "3,8,1001,8,10,8,105,1,0,0,21,38,63,72,85,110,191,272,353,434,99999,3,9,102,4,9,9,101,2,9,9,102,3,9,9,4,9,99,3,9,1001,9,4,9,102,2,9,9,1001,9,5,9,1002,9,5,9,101,3,9,9,4,9,99,3,9,1001,9,2,9,4,9,99,3,9,1001,9,3,9,102,2,9,9,4,9,99,3,9,101,2,9,9,102,2,9,9,1001,9,2,9,1002,9,4,9,101,2,9,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,101,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,99,3,9,1001,9,1,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,1001,9,1,9,4,9,99,3,9,1001,9,1,9,4,9,3,9,1001,9,1,9,4,9,3,9,1001,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,101,2,9,9,4,9,99,3,9,1001,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,101,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,101,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,2,9,9,4,9,99";
