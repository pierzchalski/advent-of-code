use lazy_static::lazy_static;
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone)]
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
        m.insert(Opcode(5), Machine::jump_if_true);
        m.insert(Opcode(6), Machine::jump_if_false);
        m.insert(Opcode(7), Machine::less_than);
        m.insert(Opcode(8), Machine::equals);
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

    fn jump_if_true(&mut self, modes: Vec<ArgMode>) -> Step {
        let in1 = self.read(self.pc + 1, modes[0]);
        let in2 = self.read(self.pc + 2, modes[1]);
        if in1 != 0 {
            debug_assert!(in2 >= 0);
            self.pc = in2 as usize;
        } else {
            self.pc += 3;
        }
        Step::Continue
    }

    fn jump_if_false(&mut self, modes: Vec<ArgMode>) -> Step {
        let in1 = self.read(self.pc + 1, modes[0]);
        let in2 = self.read(self.pc + 2, modes[1]);
        if in1 == 0 {
            debug_assert!(in2 >= 0);
            self.pc = in2 as usize;
        } else {
            self.pc += 3;
        }
        Step::Continue
    }

    fn less_than(&mut self, modes: Vec<ArgMode>) -> Step {
        let in1 = self.read(self.pc + 1, modes[0]);
        let in2 = self.read(self.pc + 2, modes[1]);
        let result = if in1 < in2 { 1 } else { 0 };
        self.write(self.pc + 3, result);
        self.pc += 4;
        Step::Continue
    }

    fn equals(&mut self, modes: Vec<ArgMode>) -> Step {
        let in1 = self.read(self.pc + 1, modes[0]);
        let in2 = self.read(self.pc + 2, modes[1]);
        let result = if in1 == in2 { 1 } else { 0 };
        self.write(self.pc + 3, result);
        self.pc += 4;
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
