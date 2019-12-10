use euclid::Point2D;
use num::Integer;
use std::collections::HashSet;

type Point = Point2D<isize, ()>;

struct Problem {
    asteroids: HashSet<Point>,
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
        Problem { asteroids }
    }

    // checks if there's a point between start and end,
    // only checking multiples of the difference.
    fn has_obstructor(&self, start: Point, end: Point) -> bool {
        let direction = end - start;
        let gcd = direction.x.gcd(&direction.y);
        let step = direction / gcd;
        for n in 1..gcd {
            let candidate = start + step * n;
            if self.asteroids.contains(&candidate) {
                return true;
            }
        }
        false
    }

    fn unobstructed_views(&self, start: Point) -> usize {
        self.asteroids
            .iter()
            .copied()
            .filter(|end| *end != start && !self.has_obstructor(start, *end))
            .count()
    }

    fn best_view_count(&self) -> usize {
        self.asteroids
            .iter()
            .copied()
            .map(|candidate| self.unobstructed_views(candidate))
            .max()
            .unwrap()
    }
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
    assert!(problem.has_obstructor(Point::new(3, 4), Point::new(0, 1)));
    assert!(!problem.has_obstructor(Point::new(3, 4), Point::new(2, 2)));

    assert_eq!(problem.best_view_count(), 8);
}

#[test]
fn problem_1() {
    assert_eq!(Problem::from_str(INPUT).best_view_count(), 278);
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
