use std::collections::HashSet;

use itertools::Itertools;

pub fn part1(input: &str) {
    let input = input.lines().collect_vec();
    let mut grid = (0..input.len()).map(|_| (0..input[0].len()).map(|_| false).collect_vec()).collect_vec();

    let mut memory = HashSet::new();
    drive(Laser::default(), &input, &mut memory, &mut grid);

    // for g in grid.iter() {
    //     println!("{}", g.iter().map(|e| if *e { '#' } else { '.' }).join(""));
    // }

    let res = grid.iter().flat_map(|line| line.iter().filter(|e| **e)).count();
    println!("{res}");
}

pub fn part2(input: &str) {
    let input = input.lines().collect_vec();
    let grid = (0..input.len()).map(|_| (0..input[0].len()).map(|_| false).collect_vec()).collect_vec();

    let res = (0..input[0].len())
        .map(|x| Laser {
            pos: Position { y: 0, x: x as isize },
            direction: Direction::Down,
        })
        .chain((0..input.len()).map(|y| Laser {
            pos: Position {
                y: y as isize,
                x: input[0].len() as isize - 1,
            },
            direction: Direction::Left,
        }))
        .chain((0..input[0].len()).map(|x| Laser {
            pos: Position {
                y: input.len() as isize - 1,
                x: x as isize,
            },
            direction: Direction::Up,
        }))
        .chain((0..input.len()).map(|y| Laser {
            pos: Position { y: y as isize, x: 0 },
            direction: Direction::Right,
        }))
        .map(|laser| {
            let mut memory = HashSet::new();
            let mut grid = grid.clone();

            drive(laser, &input, &mut memory, &mut grid);

            grid.iter().flat_map(|line| line.iter().filter(|e| **e)).count()
        })
        .max()
        .unwrap();

    println!("{res}");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
struct Position {
    y: isize,
    x: isize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
enum Direction {
    #[default]
    Right,
    Down,
    Left,
    Up,
}

#[derive(Debug, PartialEq, Eq, Default, Hash, Clone)]
struct Laser {
    pos: Position,
    direction: Direction,
}

impl Laser {
    fn finished(&self, map: &[&str], memory: &HashSet<Laser>) -> bool {
        self.pos.x < 0 || self.pos.x as usize >= map[0].len() || self.pos.y < 0 || self.pos.y as usize >= map.len() || memory.contains(self)
    }
}

fn drive(mut laser: Laser, map: &[&str], memory: &mut HashSet<Laser>, grid: &mut [Vec<bool>]) {
    while !laser.finished(map, memory) {
        memory.insert(laser.clone());

        grid[laser.pos.y as usize][laser.pos.x as usize] = true;

        let tile = map[laser.pos.y as usize].as_bytes()[laser.pos.x as usize] as char;

        match tile {
            '.' => match laser.direction {
                Direction::Right => laser.pos.x += 1,
                Direction::Down => laser.pos.y += 1,
                Direction::Left => laser.pos.x -= 1,
                Direction::Up => laser.pos.y -= 1,
            },
            '/' => match laser.direction {
                Direction::Right => {
                    laser.pos.y -= 1;
                    laser.direction = Direction::Up
                }
                Direction::Down => {
                    laser.pos.x -= 1;
                    laser.direction = Direction::Left
                }
                Direction::Left => {
                    laser.pos.y += 1;
                    laser.direction = Direction::Down
                }
                Direction::Up => {
                    laser.pos.x += 1;
                    laser.direction = Direction::Right
                }
            },
            '\\' => match laser.direction {
                Direction::Right => {
                    laser.pos.y += 1;
                    laser.direction = Direction::Down
                }
                Direction::Down => {
                    laser.pos.x += 1;
                    laser.direction = Direction::Right
                }
                Direction::Left => {
                    laser.pos.y -= 1;
                    laser.direction = Direction::Up
                }
                Direction::Up => {
                    laser.pos.x -= 1;
                    laser.direction = Direction::Left
                }
            },
            '|' => match laser.direction {
                Direction::Down => laser.pos.y += 1,
                Direction::Up => laser.pos.y -= 1,
                Direction::Left | Direction::Right => {
                    drive(
                        Laser {
                            direction: Direction::Up,
                            ..laser
                        },
                        map,
                        memory,
                        grid,
                    );

                    laser.pos.y += 1;
                    laser.direction = Direction::Down
                }
            },
            '-' => match laser.direction {
                Direction::Right => laser.pos.x += 1,
                Direction::Left => laser.pos.x -= 1,
                Direction::Down | Direction::Up => {
                    drive(
                        Laser {
                            direction: Direction::Left,
                            ..laser
                        },
                        map,
                        memory,
                        grid,
                    );

                    laser.pos.x += 1;
                    laser.direction = Direction::Right
                }
            },
            _ => unreachable!(),
        }
    }
}
