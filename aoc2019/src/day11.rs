use crate::intcode::Machine;
use std::collections::HashMap;

use crate::utils::types::*;

// starts from "up", goes clockwise.
// remember that y grows down.
static DIRECTIONS: &[Vector] = &[
    Vector::new(0, -1),
    Vector::new(1, 0),
    Vector::new(0, 1),
    Vector::new(-1, 0),
];

struct Paintbot {
    machine: Machine,
    position: Point,
    orientation: usize,
    painted: HashMap<Point, isize>,
}

impl Paintbot {
    fn from_program(src: &str) -> Self {
        Paintbot {
            machine: Machine::from_mem_spec(src),
            painted: HashMap::new(),
            position: Point::new(0, 0),
            orientation: 0,
        }
    }

    fn camera_colour(&self) -> isize {
        self.painted.get(&self.position).copied().unwrap_or(0)
    }

    fn turn_left(&mut self) {
        self.orientation += 3;
        self.orientation %= 4;
    }

    fn turn_right(&mut self) {
        self.orientation += 1;
        self.orientation %= 4;
    }

    fn move_forward(&mut self) {
        self.position += DIRECTIONS[self.orientation];
    }

    fn run(&mut self) {
        use crate::intcode::Step::*;
        loop {
            match self.machine.step() {
                Continue => continue,
                Halt => break,
                Input => self.machine.input.push_back(self.camera_colour()),
                Output(color) => {
                    self.painted.insert(self.position, color);
                    let turn = self.machine.run_to_output().unwrap();
                    match turn {
                        0 => self.turn_left(),
                        1 => self.turn_right(),
                        _ => panic!("unexpected output for turning!"),
                    }
                    self.move_forward();
                }
            }
        }
    }

    fn render(&self) -> String {
        let bounds = BoundingBox::from_points(self.painted.keys());
        let xs = (bounds.min.x)..=(bounds.max.x);
        let ys = (bounds.min.y)..=(bounds.max.y);
        let mut out = String::new();
        for y in ys {
            for x in xs.clone() {
                match self.painted.get(&Point::new(x, y)).copied().unwrap_or(0) {
                    0 => out.push('.'),
                    1 => out.push('#'),
                    _ => panic!("unexpected paint colour!"),
                }
            }
            out.push('\n');
        }
        out
    }
}

#[test]
fn problem_1() {
    let mut paintbot = Paintbot::from_program(INPUT);
    paintbot.run();
    assert_eq!(paintbot.painted.values().count(), 1747);
}

#[test]
fn problem_2() {
    let mut paintbot = Paintbot::from_program(INPUT);
    paintbot.painted.insert(Point::new(0, 0), 1);
    paintbot.run();
    let target = ".####..##...##..###..#..#.#..#.#....###....
....#.#..#.#..#.#..#.#..#.#.#..#....#..#...
...#..#....#....#..#.####.##...#....###....
..#...#....#.##.###..#..#.#.#..#....#..#...
.#....#..#.#..#.#.#..#..#.#.#..#....#..#...
.####..##...###.#..#.#..#.#..#.####.###....
";
    println!("{}", paintbot.render());
    assert_eq!(&paintbot.render(), target);
}

static INPUT: &str = "3,8,1005,8,324,1106,0,11,0,0,0,104,1,104,0,3,8,102,-1,8,10,101,1,10,10,4,10,1008,8,0,10,4,10,1002,8,1,29,2,1102,17,10,3,8,102,-1,8,10,1001,10,1,10,4,10,1008,8,1,10,4,10,102,1,8,55,2,4,6,10,1,1006,10,10,1,6,14,10,3,8,1002,8,-1,10,101,1,10,10,4,10,1008,8,1,10,4,10,101,0,8,89,3,8,102,-1,8,10,1001,10,1,10,4,10,108,0,8,10,4,10,1002,8,1,110,1,104,8,10,3,8,1002,8,-1,10,1001,10,1,10,4,10,1008,8,1,10,4,10,102,1,8,137,2,9,17,10,2,1101,14,10,3,8,102,-1,8,10,101,1,10,10,4,10,1008,8,0,10,4,10,101,0,8,167,1,107,6,10,1,104,6,10,2,1106,6,10,3,8,1002,8,-1,10,101,1,10,10,4,10,108,1,8,10,4,10,1001,8,0,200,1006,0,52,1006,0,70,1006,0,52,3,8,102,-1,8,10,101,1,10,10,4,10,1008,8,1,10,4,10,1002,8,1,232,1006,0,26,1,104,19,10,3,8,102,-1,8,10,1001,10,1,10,4,10,108,0,8,10,4,10,102,1,8,260,1,2,15,10,2,1102,14,10,3,8,1002,8,-1,10,1001,10,1,10,4,10,108,0,8,10,4,10,1001,8,0,290,1,108,11,10,1006,0,36,1006,0,90,1006,0,52,101,1,9,9,1007,9,940,10,1005,10,15,99,109,646,104,0,104,1,21101,0,666412360596,1,21101,341,0,0,1105,1,445,21101,838366659476,0,1,21102,1,352,0,1106,0,445,3,10,104,0,104,1,3,10,104,0,104,0,3,10,104,0,104,1,3,10,104,0,104,1,3,10,104,0,104,0,3,10,104,0,104,1,21101,0,97713695975,1,21102,1,399,0,1106,0,445,21102,179469028392,1,1,21101,410,0,0,1105,1,445,3,10,104,0,104,0,3,10,104,0,104,0,21102,1,988220650260,1,21101,433,0,0,1105,1,445,21101,0,838345843560,1,21101,444,0,0,1106,0,445,99,109,2,22101,0,-1,1,21102,1,40,2,21102,1,476,3,21101,466,0,0,1106,0,509,109,-2,2105,1,0,0,1,0,0,1,109,2,3,10,204,-1,1001,471,472,487,4,0,1001,471,1,471,108,4,471,10,1006,10,503,1101,0,0,471,109,-2,2106,0,0,0,109,4,1202,-1,1,508,1207,-3,0,10,1006,10,526,21101,0,0,-3,22101,0,-3,1,22102,1,-2,2,21102,1,1,3,21101,0,545,0,1106,0,550,109,-4,2105,1,0,109,5,1207,-3,1,10,1006,10,573,2207,-4,-2,10,1006,10,573,21201,-4,0,-4,1106,0,641,21201,-4,0,1,21201,-3,-1,2,21202,-2,2,3,21102,592,1,0,1106,0,550,21201,1,0,-4,21101,0,1,-1,2207,-4,-2,10,1006,10,611,21101,0,0,-1,22202,-2,-1,-2,2107,0,-3,10,1006,10,633,22102,1,-1,1,21102,1,633,0,106,0,508,21202,-2,-1,-2,22201,-4,-2,-4,109,-5,2105,1,0";
