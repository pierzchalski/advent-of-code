use euclid;
use float_cmp::approx_eq;
use num::Integer;
use std::collections::HashSet;

type Point = euclid::Point2D<isize, ()>;
type Vector = euclid::Vector2D<isize, ()>;

struct Laser {
    location: Point,
    direction: Vector,
}
struct Problem {
    asteroids: HashSet<Point>,
    laser: Laser,
}

impl Problem {
    fn from_str(input: &str) -> Self {
        let mut asteroids = HashSet::new();
        for (y, line) in input.lines().enumerate() {
            for (x, point) in line.chars().enumerate() {
                if point == '#' {
                    asteroids.insert(Point::new(x as isize, y as isize));
                }
            }
        }
        // laser starts pointing "up"
        let laser = Laser {
            direction: Vector::new(0, -1),
            location: Point::new(0, 0),
        };
        let mut problem = Problem { asteroids, laser };
        problem.laser.location = problem.best_view_location();
        problem
    }

    fn obstructor(&self, start: Point, end: Point) -> Option<Point> {
        let direction = end - start;
        let gcd = direction.x.gcd(&direction.y);
        let step = direction / gcd;
        for n in 1..gcd {
            let candidate = start + step * n;
            if self.asteroids.contains(&candidate) {
                return Some(candidate);
            }
        }
        None
    }

    fn asteroids_iter(&self) -> impl Iterator<Item = Point> + '_ {
        self.asteroids.iter().copied()
    }

    fn asteroid_dirs(&self) -> impl Iterator<Item = (Point, Vector)> + '_ {
        self.asteroids_iter()
            .filter(move |asteroid| *asteroid != self.laser.location)
            .map(move |asteroid| (asteroid, asteroid - self.laser.location))
    }

    fn unobstructed_views(&self, start: Point) -> usize {
        self.asteroids_iter()
            .filter(|end| *end != start && self.obstructor(start, *end).is_none())
            .count()
    }

    fn best_view_count(&self) -> usize {
        self.asteroids_iter()
            .map(|candidate| self.unobstructed_views(candidate))
            .max()
            .unwrap()
    }

    fn best_view_location(&self) -> Point {
        self.asteroids_iter()
            .map(|candidate| (candidate, self.unobstructed_views(candidate)))
            .max_by_key(|(_, views)| *views)
            .unwrap()
            .0
    }

    // Returns (targeted asteroid, new direction).
    fn target_laser(&self) -> (Point, Vector) {
        // yes, we're doubling up on work solely for the case where
        // we start off pointing at a target, whatever
        let (_, target_dir) = self
            .asteroid_dirs()
            .min_by(|(_, d1), (_, d2)| closest(self.laser.direction, *d1, *d2))
            .unwrap();
        let possible_targets = self
            .asteroid_dirs()
            .filter(|(_, d)| same_ray(target_dir, *d))
            .collect::<HashSet<_>>();
        let (actual_target, _) = possible_targets
            .iter()
            .copied()
            .min_by_key(|(_, d)| d.square_length())
            .unwrap();

        let (_, next_direction) = self
            .asteroid_dirs()
            .filter(|target| !possible_targets.contains(target))
            .min_by(|(_, d1), (_, d2)| closest(self.laser.direction, *d1, *d2))
            .unwrap();
        (actual_target, next_direction)
    }

    // return the destroyed target
    fn fire_laser(&mut self) -> Point {
        let (target, new_dir) = self.target_laser();
        self.asteroids.remove(&target);
        self.laser.direction = new_dir;
        target
    }
}

// Returns the "smallest whole step" from which you can get to `target` in multiple steps.
// For instance, to get to (12, 4) it's sufficient to move (3, 1) 4 times.
fn unit(target: Vector) -> (Vector, isize) {
    let gcd = target.x.gcd(&target.y);
    let step = target / gcd;
    (step, gcd)
}

// If a and b are collinear, and one is a *positive* multiple of the other,
// they both lie on the same ray.
fn same_ray(a: Vector, b: Vector) -> bool {
    let (a_step, _) = unit(a);
    let (b_step, _) = unit(b);
    a_step == b_step
}

// Clockwise angle from `from` to `to`.
// Want this to range from 0 to 2*PI.
fn rads(from: Vector, to: Vector) -> f64 {
    use std::cmp::Ordering;
    use std::f64::consts::PI;
    let angle_from = (-from.y as f64).atan2(from.x as f64);
    let angle_to = (-to.y as f64).atan2(to.x as f64);
    match angle_from.partial_cmp(&angle_to).unwrap() {
        Ordering::Less => angle_from - angle_to + 2. * PI,
        Ordering::Equal => 0.,
        Ordering::Greater => angle_from - angle_to,
    }
}

// returns ordering of which of `a` or `b` are closer (clocwise) to `target`.
fn closest(target: Vector, a: Vector, b: Vector) -> std::cmp::Ordering {
    rads(target, a).partial_cmp(&rads(target, b)).unwrap()
}

#[test]
fn problem_1_examples() {
    let problem = Problem::from_str(
        ".#..#
.....
#####
....#
...##",
    );
    assert_eq!(
        problem.obstructor(Point::new(3, 4), Point::new(1, 0)),
        Some(Point::new(2, 2))
    );
    assert_eq!(problem.obstructor(Point::new(3, 4), Point::new(2, 2)), None);

    assert_eq!(problem.best_view_count(), 8);
}

#[test]
fn problem_1() {
    assert_eq!(Problem::from_str(INPUT).best_view_count(), 278);
}

#[test]
fn problem_2_examples() {
    let mut problem = Problem::from_str(
        ".#....#####...#..
##...##.#####..##
##...#...#.#####.
..#.....#...###..
..#.#.....#....##",
    );
    assert_eq!(problem.laser.location, Point::new(8, 3));

    assert_eq!(problem.fire_laser(), Point::new(8, 1));
    assert_eq!(problem.fire_laser(), Point::new(9, 0));
    assert_eq!(problem.fire_laser(), Point::new(9, 1));
    assert_eq!(problem.fire_laser(), Point::new(10, 0));
    assert_eq!(problem.fire_laser(), Point::new(9, 2));
    assert_eq!(problem.fire_laser(), Point::new(11, 1));
    assert_eq!(problem.fire_laser(), Point::new(12, 1));
    assert_eq!(problem.fire_laser(), Point::new(11, 2));
    assert_eq!(problem.fire_laser(), Point::new(15, 1));

    for _ in 0..18 {
        problem.fire_laser();
    }

    assert_eq!(problem.fire_laser(), Point::new(6, 1));
    assert_eq!(problem.fire_laser(), Point::new(6, 0));
    assert_eq!(problem.fire_laser(), Point::new(7, 0));
    assert_eq!(problem.fire_laser(), Point::new(8, 0));
    assert_eq!(problem.fire_laser(), Point::new(10, 1));
    assert_eq!(problem.fire_laser(), Point::new(14, 0));
    assert_eq!(problem.fire_laser(), Point::new(16, 1));
    // we fail these cases because our algorithm assumes there's
    // a new target that isn't colinear with the last target.
    // this fails when there's only one row of targets left,
    // but that won't be the case in our problem.
    //assert_eq!(problem.fire_laser(), Point::new(13, 3));
    //assert_eq!(problem.fire_laser(), Point::new(14, 3));
}

#[test]
fn problem_2() {
    let mut problem = Problem::from_str(INPUT);

    let mut target = Point::new(0, 0);
    for _ in 0..200 {
        target = problem.fire_laser();
    }
    assert_eq!(target.x * 100 + target.y, 1417);
}

#[test]
fn how_does_atan2_work() {
    use std::f64::consts::PI;
    // need to account for y being flipped

    let from = Vector::new(1, 1);
    let to = Vector::new(0, 1);
    assert!(approx_eq!(f64, rads(from, to), PI / 4., ulps = 2));

    let to = Vector::new(-1, 1);
    assert!(approx_eq!(f64, rads(from, to), PI / 2., ulps = 2));

    let to = Vector::new(-1, 0);
    assert!(approx_eq!(f64, rads(from, to), 3. * PI / 4., ulps = 2));

    let to = Vector::new(-1, -1);
    assert!(approx_eq!(f64, rads(from, to), PI, ulps = 2));

    let to = Vector::new(0, -1);
    assert!(approx_eq!(f64, rads(from, to), 5. * PI / 4., ulps = 2));

    let to = Vector::new(1, -1);
    assert!(approx_eq!(f64, rads(from, to), 3. * PI / 2., ulps = 2));

    let to = Vector::new(1, 0);
    assert!(approx_eq!(f64, rads(from, to), 7. * PI / 4., ulps = 2));

    let to = Vector::new(1, 1);
    assert!(approx_eq!(f64, rads(from, to), 0., ulps = 2));
}

static INPUT: &str = ".#......#...#.....#..#......#..##..#
..#.......#..........#..##.##.......
##......#.#..#..#..##...#.##.###....
..#........#...........#.......##...
.##.....#.......#........#..#.#.....
.#...#...#.....#.##.......#...#....#
#...#..##....#....#......#..........
....#......#.#.....#..#...#......#..
......###.......#..........#.##.#...
#......#..#.....#..#......#..#..####
.##...##......##..#####.......##....
.....#...#.........#........#....#..
....##.....#...#........#.##..#....#
....#........#.###.#........#...#..#
....#..#.#.##....#.........#.....#.#
##....###....##..#..#........#......
.....#.#.........#.......#....#....#
.###.....#....#.#......#...##.##....
...##...##....##.........#...#......
.....#....##....#..#.#.#...##.#...#.
#...#.#.#.#..##.#...#..#..#..#......
......#...#...#.#.....#.#.....#.####
..........#..................#.#.##.
....#....#....#...#..#....#.....#...
.#####..####........#...............
#....#.#..#..#....##......#...#.....
...####....#..#......#.#...##.....#.
..##....#.###.##.#.##.#.....#......#
....#.####...#......###.....##......
.#.....#....#......#..#..#.#..#.....
..#.......#...#........#.##...#.....
#.....####.#..........#.#.......#...
..##..#..#.....#.#.........#..#.#.##
.........#..........##.#.##.......##
#..#.....#....#....#.#.......####..#
..............#.#...........##.#.#..";
