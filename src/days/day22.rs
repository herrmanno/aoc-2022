//! # Day 22 Monkey Map
//!
//! Given a 2d map with walls and empy (walkable) tiles and a set of commands (walk n steps, turn
//! clockwise, turn counter clockwise) find
//!
//! - a) the position where the path leads if the map 'wraps around' on edges
//! - b) the position where the path leads if the map is an unrolled cube
//!

use std::{collections::VecDeque, ops::ControlFlow};

use aoc_runner::Day;
use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};

#[derive(Clone, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
}

type C = i16;
type Coord = (C, C);
type Coord3 = (C, C, C);
type Board = HashMap<Coord, Tile>;

#[cfg(debug_assertions)]
fn print_board(board: &Board) {
    let max_x = board.keys().map(|(_, x)| *x).max().unwrap();
    let max_y = board.keys().map(|(y, _)| *y).max().unwrap();
    for y in 0..=max_y {
        for x in 0..=max_x {
            print!(
                "{}",
                match board.get(&(y, x)) {
                    Some(Tile::Empty) => ".",
                    Some(Tile::Wall) => "#",
                    _ => " ",
                }
            )
        }
        println!();
    }
}

#[derive(Debug, Clone)]
enum Command {
    Walk(u8),
    TurnR(),
    TurnL(),
}

type Direction = (C, C);

#[derive(Default, Clone)]
pub struct Day22 {
    board: Board,
    start: Coord,
    commands: Vec<Command>,
}

impl Day for Day22 {
    type Result1 = i32;
    type Result2 = i32;

    fn parse(&mut self, input: &str) {
        const SIDE_LENGTH: C = 50;
        let (board, commands) = input.split_once("\n\n").unwrap();
        self.board = board
            .lines()
            .enumerate()
            .flat_map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(move |(col, tile)| match tile {
                        '.' => Some(((row as C, col as C), Tile::Empty)),
                        '#' => Some(((row as C, col as C), Tile::Wall)),
                        _ => None,
                    })
            })
            .collect();

        self.start = (
            0,
            (0..SIDE_LENGTH * 6)
                .find(|i| self.board.contains_key(&(0, *i)))
                .unwrap(),
        );

        let commands_vec = {
            let mut vec = vec![];
            let mut num = 0;
            for c in commands.chars() {
                if c == 'R' {
                    if num != 0 {
                        vec.push(Command::Walk(num as u8));
                        num = 0;
                    }
                    vec.push(Command::TurnR());
                } else if c == 'L' {
                    if num != 0 {
                        vec.push(Command::Walk(num as u8));
                        num = 0;
                    }
                    vec.push(Command::TurnL());
                } else {
                    num = num * 10 + c.to_digit(10).unwrap();
                }
            }

            if num != 0 {
                vec.push(Command::Walk(num as u8));
            }

            vec
        };

        self.commands = commands_vec;
    }

    fn part1(&mut self) -> Self::Result1 {
        const SIDE_LENGTH: i16 = 50;
        let direction_right = (0, 1);
        let ((row, col), dir) = walk_board(
            &self.board,
            SIDE_LENGTH,
            self.start,
            direction_right,
            &self.commands,
        );
        1000 * (row as Self::Result1 + 1)
            + 4 * (col as Self::Result1 + 1)
            + match dir {
                (0, 1) => 0,
                (1, 0) => 1,
                (0, -1) => 2,
                (-1, 0) => 3,
                _ => unreachable!(),
            }
    }

    fn part2(&mut self) -> Self::Result2 {
        const SIDE_LENGTH: i16 = 50;
        let direction_right = (0, 1);
        let ((row, col), dir) = walk_cube(
            &self.board,
            SIDE_LENGTH,
            self.start,
            direction_right,
            &self.commands,
        );
        1000 * (row as Self::Result1 + 1)
            + 4 * (col as Self::Result1 + 1)
            + match dir {
                (0, 1) => 0,
                (1, 0) => 1,
                (0, -1) => 2,
                (-1, 0) => 3,
                _ => unreachable!(),
            }
    }
}

fn walk(
    board: &Board,
    side_length: C,
    start: Coord,
    direction: Direction,
    commands: &[Command],
    on_edge: fn(&Board, C, Coord, Direction) -> ControlFlow<(), (Coord, Direction)>,
) -> (Coord, Direction) {
    let mut pos = start;
    let mut dir = direction;
    for command in commands {
        match command {
            Command::Walk(n) => {
                'walk: for _ in 0..*n {
                    let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
                    match board.get(&new_pos) {
                        Some(Tile::Empty) => {
                            pos = new_pos;
                        }
                        Some(Tile::Wall) => {
                            break 'walk;
                        }
                        None => match on_edge(board, side_length, new_pos, dir) {
                            ControlFlow::Continue((new_pos, new_dir)) => {
                                pos = new_pos;
                                dir = new_dir;
                            }
                            ControlFlow::Break(_) => {
                                break 'walk;
                            }
                        },
                    }
                }
            }
            Command::TurnR() => {
                dir = match dir {
                    (0, 1) => (1, 0),
                    (1, 0) => (0, -1),
                    (0, -1) => (-1, 0),
                    (-1, 0) => (0, 1),
                    _ => unreachable!(),
                };
            }
            Command::TurnL() => {
                dir = match dir {
                    (0, 1) => (-1, 0),
                    (-1, 0) => (0, -1),
                    (0, -1) => (1, 0),
                    (1, 0) => (0, 1),
                    _ => unreachable!(),
                };
            }
        }
    }
    (pos, dir)
}

fn walk_board(
    board: &Board,
    side_length: C,
    start: Coord,
    direction: Direction,
    commands: &[Command],
) -> (Coord, Direction) {
    fn on_edge(
        board: &Board,
        side_length: C,
        pos: Coord,
        dir: Direction,
    ) -> ControlFlow<(), (Coord, Direction)> {
        let new_pos = {
            let dir = (-dir.0 * side_length, -dir.1 * side_length);
            let mut tmp_pos = (pos.0 + dir.0, pos.1 + dir.1);
            while board.contains_key(&tmp_pos) {
                tmp_pos.0 += dir.0;
                tmp_pos.1 += dir.1;
            }
            (tmp_pos.0 - dir.0, tmp_pos.1 - dir.1)
        };
        match board.get(&new_pos) {
            Some(Tile::Wall) => ControlFlow::Break(()),
            Some(Tile::Empty) => ControlFlow::Continue((new_pos, dir)),
            _ => unreachable!(),
        }
    }
    walk(board, side_length, start, direction, commands, on_edge)
}

fn walk_cube(
    board: &Board,
    side_length: C,
    start: Coord,
    direction: Direction,
    commands: &[Command],
) -> (Coord, Direction) {
    fn on_edge(
        board: &Board,
        side_length: C,
        pos: Coord,
        dir: Direction,
    ) -> ControlFlow<(), (Coord, Direction)> {
        let old_pos = (pos.0 - dir.0, pos.1 - dir.1);
        let new_pos = find_point_on_cube(board, side_length, old_pos, pos);
        if board.get(&new_pos) == Some(&Tile::Wall) {
            ControlFlow::Break(())
        } else {
            let pos = new_pos;
            let dir = [(0, 1), (0, -1), (1, 0), (-1, 0)]
                .into_iter()
                .find(|(dy, dx)| !board.contains_key(&(pos.0 - dy, pos.1 - dx)))
                .unwrap();
            ControlFlow::Continue((pos, dir))
        }
    }
    walk(board, side_length, start, direction, commands, on_edge)
}

fn walk_board_(
    board: &Board,
    side_length: C,
    start: Coord,
    direction: Direction,
    commands: &[Command],
) -> (Coord, Direction) {
    let mut pos = start;
    let mut dir = direction;
    for command in commands {
        match command {
            Command::Walk(n) => {
                'walk: for _ in 0..*n {
                    let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
                    match board.get(&new_pos) {
                        Some(Tile::Empty) => {
                            pos = new_pos;
                        }
                        Some(Tile::Wall) => {
                            break 'walk;
                        }
                        None => {
                            let new_pos = {
                                let dir = (-dir.0 * side_length, -dir.1 * side_length);
                                let mut tmp_pos = (new_pos.0 + dir.0, new_pos.1 + dir.1);
                                while board.contains_key(&tmp_pos) {
                                    tmp_pos.0 += dir.0;
                                    tmp_pos.1 += dir.1;
                                }
                                (tmp_pos.0 - dir.0, tmp_pos.1 - dir.1)
                            };
                            match board.get(&new_pos) {
                                Some(Tile::Wall) => {
                                    break 'walk;
                                }
                                Some(Tile::Empty) => {
                                    pos = new_pos;
                                }
                                _ => unreachable!(),
                            }
                        }
                    }
                }
            }
            Command::TurnR() => {
                dir = match dir {
                    (0, 1) => (1, 0),
                    (1, 0) => (0, -1),
                    (0, -1) => (-1, 0),
                    (-1, 0) => (0, 1),
                    _ => unreachable!(),
                };
            }
            Command::TurnL() => {
                dir = match dir {
                    (0, 1) => (-1, 0),
                    (-1, 0) => (0, -1),
                    (0, -1) => (1, 0),
                    (1, 0) => (0, 1),
                    _ => unreachable!(),
                };
            }
        }
    }
    (pos, dir)
}

fn walk_cube_(
    board: &Board,
    side_length: C,
    start: Coord,
    direction: Direction,
    commands: &[Command],
) -> (Coord, Direction) {
    let mut pos = start;
    let mut dir = direction;
    for command in commands {
        match command {
            Command::Walk(n) => {
                'walk: for _ in 0..*n {
                    let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
                    match board.get(&new_pos) {
                        Some(Tile::Empty) => {
                            pos = new_pos;
                        }
                        Some(Tile::Wall) => {
                            break 'walk;
                        }
                        // wrap around
                        None => {
                            let new_pos = find_point_on_cube(board, side_length, pos, new_pos);
                            if board.get(&new_pos) == Some(&Tile::Wall) {
                                break 'walk;
                            } else {
                                pos = new_pos;
                                dir = [(0, 1), (0, -1), (1, 0), (-1, 0)]
                                    .into_iter()
                                    .find(|(dy, dx)| !board.contains_key(&(pos.0 - dy, pos.1 - dx)))
                                    .unwrap();
                            }
                        }
                    }
                }
            }
            Command::TurnR() => {
                dir = match dir {
                    (0, 1) => (1, 0),
                    (1, 0) => (0, -1),
                    (0, -1) => (-1, 0),
                    (-1, 0) => (0, 1),
                    _ => unreachable!(),
                };
            }
            Command::TurnL() => {
                dir = match dir {
                    (0, 1) => (-1, 0),
                    (-1, 0) => (0, -1),
                    (0, -1) => (1, 0),
                    (1, 0) => (0, 1),
                    _ => unreachable!(),
                };
            }
        }
    }
    (pos, dir)
}

/// Given a board (an unrolled cube) and a point that is not part of the board, find the correspnding
/// point at another cube's face as board coordinate
fn find_point_on_cube(board: &Board, side_length: C, current: Coord, target: Coord) -> Coord {
    let face_origin = (
        current.0 - (current.0 % side_length),
        current.1 - (current.1 % side_length),
    );
    let relative_target = (target.0 - face_origin.0, target.1 - face_origin.1, 0);

    let mut visited: HashSet<(Coord, Coord3)> = Default::default();
    let mut queue: VecDeque<(Coord, Coord3)> = Default::default();
    queue.push_back((face_origin, relative_target));

    while let Some(state @ (face_origin, relative_target)) = queue.pop_front() {
        if !visited.insert(state) {
            continue;
        }

        let (o_y, o_x) = face_origin;
        let (t_y, t_x, t_z) = relative_target;

        if t_z == -1 && board.contains_key(&(o_y + t_y, o_x + t_x)) {
            return (o_y + t_y, o_x + t_x);
        }

        if board.contains_key(&(o_y - side_length, o_x)) {
            let new_state = ((o_y - side_length, o_x), (side_length - t_z - 1, t_x, t_y));
            queue.push_back(new_state);
        }

        if board.contains_key(&(o_y + side_length, o_x)) {
            let new_state = ((o_y + side_length, o_x), (t_z, t_x, side_length - t_y - 1));
            queue.push_back(new_state);
        }

        if board.contains_key(&(o_y, o_x - side_length)) {
            let new_state = ((o_y, o_x - side_length), (t_y, side_length - t_z - 1, t_x));
            queue.push_back(new_state);
        }

        if board.contains_key(&(o_y, o_x + side_length)) {
            let new_state = ((o_y, o_x + side_length), (t_y, t_z, side_length - t_x - 1));
            queue.push_back(new_state);
        }
    }

    panic!("Did not find point around edge")
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";

    #[test]
    fn part_1() {
        let mut day = Day22::default();
        day.parse(INPUT);
        let expected = walk_board(&day.board, 4, day.start, (0, 1), &day.commands);
        assert_eq!(expected, ((5, 7), (0, 1)));
    }

    #[test]
    fn part_2() {
        let mut day = Day22::default();
        day.parse(INPUT);
        let expected = walk_cube(&day.board, 4, day.start, (0, 1), &day.commands);
        assert_eq!(expected, ((4, 6), (-1, 0)));
    }
}
