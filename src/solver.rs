use crate::types::*;

pub struct Solver;

impl Solver {
    pub fn solve(puzzle: &Puzzle) -> Option<Solution> {
        let a: i32 = puzzle.chars.len() as i32;
        let b: i32 = puzzle.chars.iter().next().unwrap().len() as i32;
        let size = (a, b);
        let mut solutions: Vec<Solution> = vec![];
        let first = puzzle.target.chars().next().unwrap();
        for (i, start_row) in puzzle.chars.iter().enumerate() {
            for (j, point) in start_row.iter().enumerate() {
                if *point == first {
                    let mut placed_temp: Vec<(i32, i32)> = vec![];
                    placed_temp.push((i as i32, j as i32));
                    let ret = Self::solve_internal(puzzle, placed_temp, size);
                    if let Some(solution) = ret {
                        solutions.push(solution);
                    }
                }
            }
        }
        solutions.first().cloned()
    }

    fn solve_internal(
        puzzle: &Puzzle,
        placed: Vec<(i32, i32)>,
        size: (i32, i32),
    ) -> Option<Solution> {
        let cur = placed.last().unwrap();
        let mut solutions: Vec<Solution> = vec![];
        if placed.len() == puzzle.target.len() {
            return Some(Solution {
                seq: placed
            })
        }
        for i in -1..=1 {
            for j in -1..=1 {
                let point = (cur.0 + i, cur.1 + j);
                if (i, j) == (0, 0) {
                    continue;
                }
                if point.0 < 0 || point.1 < 0 {
                    continue;
                }
                if point.0 >= size.0 || point.1 >= size.1 {
                    continue;
                }
                if placed.contains(&point) {
                    continue;
                }
                if puzzle.chars.iter().nth(point.0 as usize).unwrap().iter().nth(point.1 as usize).unwrap()
                    == &puzzle.target.chars().nth(placed.len()).unwrap()
                {
                    let mut placed_temp = placed.clone();
                    placed_temp.push(point);
                    let ret = Self::solve_internal(puzzle, placed_temp, size);
                    if let Some(solution) = ret {
                        solutions.push(solution);
                    }
                }
            }
        }
        return solutions.first().cloned();
    }
}
