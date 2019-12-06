use lazy_static::lazy_static;
use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
pub struct Machine {
    pub pc: usize,
    pub memory: Vec<isize>,
    // Specifying the input events ahead of time might come back to bite us
    // but it's good enough for now.
    // It's hard to predict how we'll need to change this for "interactive"
    // problems.
    //
    // This is a vecdeque solely so we can pop from the front.
    pub input: VecDeque<isize>,
    pub output: Vec<isize>,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Step {
    Continue,
    Halt,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum ArgMode {
    Immediate,
    Position,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Opcode(usize);

type InstructionImpl = fn(&mut Machine, Vec<ArgMode>) -> Step;

lazy_static! {
    static ref INSTRUCTION: HashMap<Opcode, InstructionImpl> = {
        let mut m = HashMap::new();
        m.insert(Opcode(1), Machine::add as InstructionImpl);
        m.insert(Opcode(2), Machine::mul);
        m.insert(Opcode(3), Machine::input);
        m.insert(Opcode(4), Machine::output);
        m.insert(Opcode(99), Machine::halt);
        m
    };
}

// We could do something fancy and only return as many arg modes as
// are needed for the given opcode, or we could be lazy and just return
// the maximum number needed.
fn parse_opcode(code: isize) -> (Opcode, Vec<ArgMode>) {
    assert!(code >= 0);
    let opcode = Opcode((code % 100) as usize); // get last two digits
    let mut mode = code / 100;
    let mut result = Vec::new();
    while mode > 0 {
        if mode % 2 == 1 {
            result.push(ArgMode::Immediate)
        } else {
            result.push(ArgMode::Position)
        }
        mode /= 10;
    }
    while result.len() < 3 {
        result.push(ArgMode::Position);
    }
    (opcode, result)
}

#[test]
fn check_parse_opcode() {
    use ArgMode::*;
    assert_eq!(
        parse_opcode(3),
        (Opcode(3), vec![Position, Position, Position])
    );
    assert_eq!(
        parse_opcode(1002),
        (Opcode(2), vec![Position, Immediate, Position])
    );
    assert_eq!(
        parse_opcode(144),
        (Opcode(44), vec![Immediate, Position, Position])
    );
}

impl Machine {
    pub fn from_mem_spec(mem: &str) -> Self {
        let memory = mem.split(',').map(|s| s.parse().unwrap()).collect();
        Machine {
            pc: 0,
            input: VecDeque::new(),
            output: Vec::new(),
            memory,
        }
    }

    fn read(&self, addr: usize, mode: ArgMode) -> isize {
        match mode {
            ArgMode::Immediate => self.memory[addr],
            ArgMode::Position => {
                let addr = self.memory[addr];
                debug_assert!(addr >= 0);
                self.memory[addr as usize]
            }
        }
    }

    pub fn write(&mut self, addr: usize, value: isize) {
        let target = self.memory[addr];
        debug_assert!(target >= 0);
        self.memory[target as usize] = value;
    }

    fn add(&mut self, modes: Vec<ArgMode>) -> Step {
        // Could we factor out this common logic?
        // Probably, but then the spec would probably change and we'd be screwed.
        let in1 = self.read(self.pc + 1, modes[0]);
        let in2 = self.read(self.pc + 2, modes[1]);
        self.write(self.pc + 3, in1 + in2);
        self.pc += 4;
        Step::Continue
    }

    fn mul(&mut self, modes: Vec<ArgMode>) -> Step {
        let in1 = self.read(self.pc + 1, modes[0]);
        let in2 = self.read(self.pc + 2, modes[1]);
        self.write(self.pc + 3, in1 * in2);
        self.pc += 4;
        Step::Continue
    }

    fn halt(&mut self, _: Vec<ArgMode>) -> Step {
        Step::Halt
    }

    fn input(&mut self, _: Vec<ArgMode>) -> Step {
        let in1 = self.input.pop_front().unwrap();
        self.write(self.pc + 1, in1);
        self.pc += 2;
        Step::Continue
    }

    fn output(&mut self, modes: Vec<ArgMode>) -> Step {
        let out1 = self.read(self.pc + 1, modes[0]);
        self.output.push(out1);
        self.pc += 2;
        Step::Continue
    }

    fn step(&mut self) -> Step {
        let code = self.memory[self.pc];
        let (code, modes) = parse_opcode(code);
        let instruction = INSTRUCTION[&code];
        instruction(self, modes)
    }

    pub fn run(&mut self) {
        while self.step() == Step::Continue {}
    }
}

#[test]
fn problem_1() {
    let mut machine = Machine::from_mem_spec(INPUT);
    machine.input.push_front(1);
    machine.run();
    dbg!(&machine.output);
    assert_eq!(machine.output.pop(), Some(7839346));
}

static INPUT: &str = "3,225,1,225,6,6,1100,1,238,225,104,0,1102,89,49,225,1102,35,88,224,101,-3080,224,224,4,224,102,8,223,223,1001,224,3,224,1,223,224,223,1101,25,33,224,1001,224,-58,224,4,224,102,8,223,223,101,5,224,224,1,223,224,223,1102,78,23,225,1,165,169,224,101,-80,224,224,4,224,102,8,223,223,101,7,224,224,1,224,223,223,101,55,173,224,1001,224,-65,224,4,224,1002,223,8,223,1001,224,1,224,1,223,224,223,2,161,14,224,101,-3528,224,224,4,224,1002,223,8,223,1001,224,7,224,1,224,223,223,1002,61,54,224,1001,224,-4212,224,4,224,102,8,223,223,1001,224,1,224,1,223,224,223,1101,14,71,225,1101,85,17,225,1102,72,50,225,1102,9,69,225,1102,71,53,225,1101,10,27,225,1001,158,34,224,101,-51,224,224,4,224,102,8,223,223,101,6,224,224,1,223,224,223,102,9,154,224,101,-639,224,224,4,224,102,8,223,223,101,2,224,224,1,224,223,223,4,223,99,0,0,0,677,0,0,0,0,0,0,0,0,0,0,0,1105,0,99999,1105,227,247,1105,1,99999,1005,227,99999,1005,0,256,1105,1,99999,1106,227,99999,1106,0,265,1105,1,99999,1006,0,99999,1006,227,274,1105,1,99999,1105,1,280,1105,1,99999,1,225,225,225,1101,294,0,0,105,1,0,1105,1,99999,1106,0,300,1105,1,99999,1,225,225,225,1101,314,0,0,106,0,0,1105,1,99999,108,226,226,224,102,2,223,223,1006,224,329,101,1,223,223,1007,677,677,224,1002,223,2,223,1005,224,344,1001,223,1,223,8,226,677,224,1002,223,2,223,1006,224,359,1001,223,1,223,108,226,677,224,1002,223,2,223,1005,224,374,1001,223,1,223,107,226,677,224,102,2,223,223,1006,224,389,101,1,223,223,1107,226,226,224,1002,223,2,223,1005,224,404,1001,223,1,223,1107,677,226,224,102,2,223,223,1005,224,419,101,1,223,223,1007,226,226,224,102,2,223,223,1006,224,434,1001,223,1,223,1108,677,226,224,1002,223,2,223,1005,224,449,101,1,223,223,1008,226,226,224,102,2,223,223,1005,224,464,101,1,223,223,7,226,677,224,102,2,223,223,1006,224,479,101,1,223,223,1008,226,677,224,1002,223,2,223,1006,224,494,101,1,223,223,1107,226,677,224,1002,223,2,223,1005,224,509,1001,223,1,223,1108,226,226,224,1002,223,2,223,1006,224,524,101,1,223,223,7,226,226,224,102,2,223,223,1006,224,539,1001,223,1,223,107,226,226,224,102,2,223,223,1006,224,554,101,1,223,223,107,677,677,224,102,2,223,223,1006,224,569,101,1,223,223,1008,677,677,224,1002,223,2,223,1006,224,584,1001,223,1,223,8,677,226,224,1002,223,2,223,1005,224,599,101,1,223,223,1108,226,677,224,1002,223,2,223,1005,224,614,101,1,223,223,108,677,677,224,102,2,223,223,1005,224,629,1001,223,1,223,8,677,677,224,1002,223,2,223,1005,224,644,1001,223,1,223,7,677,226,224,102,2,223,223,1006,224,659,1001,223,1,223,1007,226,677,224,102,2,223,223,1005,224,674,101,1,223,223,4,223,99,226";
