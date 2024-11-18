use crate::types::Puzzle;
use rand::*;
use rand::seq::SliceRandom;

pub struct PuzzleGenerator;

fn gen(placed: Vec<(i32, i32)>, target: usize, size: (i32, i32)) -> Option<Vec<(i32, i32)>> {
    if placed.len() == target {
        return Some(placed);
    }
    let mut possible: Vec<(i32, i32)> = if placed.len() > 0 {
        let head = placed.last().unwrap();
        (-1..2)
            .map(|n| (-1..2).map(|m| (m, n)).collect::<Vec<_>>())
            .flatten()
            .map(|(x, y)| (x + head.0, y + head.1))
            .filter(|p| {
                !placed.contains(&p) && {
                    p.0 + head.0 >= 0
                        && p.0 + head.0 < size.0
                        && p.1 + head.1 >= 0
                        && p.1 + head.1 < size.1
                }
            })
            .collect()
    } else {
        (0..size.1).map(|n| (0..size.0).map(|m| (m, n)).collect::<Vec<_>>()).flatten().collect()
    };


    if possible.len() == 0 {
        return None;
    }

    possible.shuffle(&mut thread_rng());

    possible
        .iter()
        .map(|p| {
            let mut a = placed.clone();
            a.push(*p);
            gen(a, target, size)
        }).filter_map(|f| f).next()
}

impl PuzzleGenerator {
    pub fn generate(word: String, size: (i32, i32)) -> Puzzle {
        let mut rng = thread_rng();
        let mut puzzle: Vec<Vec<char>> = vec![vec![' '; size.0 as usize]; size.1 as usize];

        // Random fill

        for p in puzzle.iter_mut() {
            for c in p {
                *c = word.chars().nth(rng.gen_range(0..word.chars().count())).unwrap();
            }
        }

        // Guaranteed word generation

        let placed = vec![];

        for (p, c) in gen(placed, word.chars().count(), size)
            .unwrap()
            .iter()
            .zip(word.chars())
        {
            let a = puzzle
                .iter_mut()
                .nth(p.0 as usize)
                .unwrap()
                .iter_mut()
                .nth(p.1 as usize)
                .unwrap();
            *a = c;
        }

        Puzzle {
            chars: puzzle,
            target: word,
        }
    }
}
