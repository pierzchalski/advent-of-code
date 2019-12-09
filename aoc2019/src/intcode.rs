use lazy_static::lazy_static;
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone)]
pub struct Machine {
    pub pc: usize,
    pub relative_base: isize,
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
pub enum Step {
    Continue,
    Input,
    Output(isize),
    Halt,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum ArgMode {
    Immediate,
    Position,
    Relative,
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
        m.insert(Opcode(9), Machine::adjust_relative_base);
        m.insert(Opcode(99), Machine::halt);
        m
    };
}

// We could do something fancy and only return as many arg modes as
// are needed for the given opcode, or we could be lazy and just return
// the maximum number needed.
fn parse_opcode(code: isize) -> (Opcode, Vec<ArgMode>) {
    use ArgMode::*;
    assert!(code >= 0);
    let opcode = Opcode((code % 100) as usize); // get last two digits
    let mut mode = code / 100;
    let mut result = Vec::new();
    while mode > 0 {
        result.push(match mode % 10 {
            0 => Position,
            1 => Immediate,
            2 => Relative,
            other => panic!("Unexpected opcode argument mode {}", other),
        });
        mode /= 10;
    }
    result.resize(3, Position);
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
    assert_eq!(
        parse_opcode(20200),
        (Opcode(0), vec![Relative, Position, Relative])
    );
}

impl Machine {
    pub fn from_mem_spec(mem: &str) -> Self {
        let memory: Vec<isize> = mem.split(',').map(|s| s.parse().unwrap()).collect();
        Machine {
            pc: 0,
            relative_base: 0,
            input: VecDeque::new(),
            output: Vec::new(),
            memory,
        }
    }

    fn grow_mem(&mut self, addr: usize) {
        if addr >= self.memory.len() {
            self.memory.resize(addr + 1, 0);
        }
    }

    fn read(&mut self, addr: usize, mode: ArgMode) -> isize {
        match mode {
            ArgMode::Immediate => {
                self.grow_mem(addr);
                self.memory[addr]
            },
            ArgMode::Position => {
                let addr = self.memory[addr];
                debug_assert!(addr >= 0);
                self.grow_mem(addr as usize);
                self.memory[addr as usize]
            },
            ArgMode::Relative => {
                let addr = self.memory[addr];
                let addr = addr as isize + self.relative_base;
                debug_assert!(addr >= 0);
                self.grow_mem(addr as usize);
                self.memory[addr as usize]
            },
        }
    }

    fn write(&mut self, addr: usize, value: isize, mode: ArgMode) {
        match mode {
            ArgMode::Position => {
                self.grow_mem(addr);
                let target = self.memory[addr];
                debug_assert!(target >= 0);
                self.grow_mem(target as usize);
                self.memory[target as usize] = value;
            },
            ArgMode::Relative => {
                self.grow_mem(addr);
                let addr = self.memory[addr];
                let addr = addr + self.relative_base;
                debug_assert!(addr >= 0);
                self.grow_mem(addr as usize);
                self.memory[addr as usize] = value;
            },
            ArgMode::Immediate => panic!("Writing out in immediate mode!"),
        }
    }

    fn add(&mut self, modes: Vec<ArgMode>) -> Step {
        // Could we factor out this common logic?
        // Probably, but then the spec would probably change and we'd be screwed.
        let in1 = self.read(self.pc + 1, modes[0]);
        let in2 = self.read(self.pc + 2, modes[1]);
        self.write(self.pc + 3, in1 + in2, modes[2]);
        self.pc += 4;
        Step::Continue
    }

    fn mul(&mut self, modes: Vec<ArgMode>) -> Step {
        let in1 = self.read(self.pc + 1, modes[0]);
        let in2 = self.read(self.pc + 2, modes[1]);
        self.write(self.pc + 3, in1 * in2, modes[2]);
        self.pc += 4;
        Step::Continue
    }

    fn halt(&mut self, _: Vec<ArgMode>) -> Step {
        Step::Halt
    }

    fn input(&mut self, modes: Vec<ArgMode>) -> Step {
        if let Some(in1) = self.input.pop_front() {
            self.write(self.pc + 1, in1, modes[0]);
            self.pc += 2;
            Step::Continue
        } else {
            Step::Input
        }
    }

    fn output(&mut self, modes: Vec<ArgMode>) -> Step {
        let out1 = self.read(self.pc + 1, modes[0]);
        self.output.push(out1);
        self.pc += 2;
        Step::Output(out1)
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
        self.write(self.pc + 3, result, modes[2]);
        self.pc += 4;
        Step::Continue
    }

    fn equals(&mut self, modes: Vec<ArgMode>) -> Step {
        let in1 = self.read(self.pc + 1, modes[0]);
        let in2 = self.read(self.pc + 2, modes[1]);
        let result = if in1 == in2 { 1 } else { 0 };
        self.write(self.pc + 3, result, modes[2]);
        self.pc += 4;
        Step::Continue
    }

    fn adjust_relative_base(&mut self, modes: Vec<ArgMode>) -> Step {
        let in1 = self.read(self.pc + 1, modes[0]);
        self.relative_base += in1;
        self.pc += 2;
        Step::Continue
    }

    pub fn step(&mut self) -> Step {
        let code = self.memory[self.pc];
        let (code, modes) = parse_opcode(code);
        let instruction = INSTRUCTION[&code];
        instruction(self, modes)
    }

    // Runs to completion, assumes self.input and self.output are set up
    // as needed.
    pub fn run(&mut self) {
        use Step::*;
        loop {
            match self.step() {
                Output(_) | Continue => continue,
                Halt => break,
                Input => panic!("waiting for input during complete run!"),
            }
        }
    }

    // runs until the next output step, or halts.
    pub fn run_to_output(&mut self) -> Option<isize> {
        use Step::*;
        loop {
            match self.step() {
                Continue => continue,
                Output(output) => return Some(output),
                Halt => return None,
                Input => panic!("waiting for input during output run!"),
            }
        }
    }
}
